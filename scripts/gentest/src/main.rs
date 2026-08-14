use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io::{IsTerminal, Write as _};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use fantoccini::wd::TimeoutConfiguration;
use fantoccini::{Client, ClientBuilder};
use log::*;
use serde_json::Value;
use tokio::time::timeout;
use walkdir::WalkDir;
use xmlwriter::{Indent, Options, XmlWriter};

/// How long to wait for any single step of test generation before giving up
///
/// This bounds starting ChromeDriver, starting and quitting the browser, and each command sent to
/// the browser. All of these normally take well under a second.
const TIMEOUT: Duration = Duration::from_secs(10);

/// How long to wait for the browser to exit on its own before killing it
const BROWSER_EXIT_GRACE_PERIOD: Duration = Duration::from_secs(2);

#[tokio::main]
async fn main() {
    env_logger::init();
    let root_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = root_dir.parent().and_then(Path::parent).unwrap();

    let fixtures_root = repo_root.join("test_fixtures");

    info!("reading test fixtures from disk");
    let mut fixtures: Vec<_> = WalkDir::new(fixtures_root.clone())
        .into_iter()
        .filter_map(|a| a.ok())
        .filter(|f| !f.path().components().any(|c| c.as_os_str() == OsStr::new("_scratch")))
        .filter(|f| !f.file_name().to_string_lossy().starts_with('x')) // ignore tests beginning with x
        .filter(|f| f.path().is_file() && f.path().extension().map(|p| p == "html").unwrap_or(false))
        .map(|f| {
            let fixture_path = f.path().to_path_buf();
            let name = fixture_path.file_stem().unwrap().to_str().unwrap().to_string();
            (name, fixture_path)
        })
        .collect();
    fixtures.sort_unstable_by_key(|f| f.1.clone());

    info!("obtaining chrome-for-testing");
    let chrome = getchrome::download_default().unwrap_or_else(|err| fatal(&err.to_string()));
    info!("using chrome-for-testing {}", chrome.version);

    let mut webdriver = WebDriver::start(&chrome.chromedriver).unwrap_or_else(|err| fatal(&err));

    // Collect the test descriptions, making sure that the browser and webdriver are shut down
    // afterwards regardless of whether doing so succeeded.
    let result = match webdriver.new_session(&chrome.chrome).await {
        Ok(client) => {
            let result = collect_test_descs(&client, &fixtures).await;
            webdriver.close_session(client).await;
            result
        }
        Err(err) => Err(err),
    };
    webdriver.shutdown();
    let test_descs = result.unwrap_or_else(|err| fatal(&err));

    info!("generating test sources and concatenating...");

    let test_descs: Vec<_> = test_descs
        .iter()
        .flat_map(|(name, fixture_path, description)| {
            debug!("generating test contents for {}", name);

            let border_box_ltr_test =
                generate_test(format!("{name}__border_box_ltr"), &description["borderBoxLtrData"]);
            let content_box_ltr_test =
                generate_test(format!("{name}__content_box_ltr"), &description["contentBoxLtrData"]);
            let border_box_rtl_test =
                generate_test(format!("{name}__border_box_rtl"), &description["borderBoxRtlData"]);
            let content_box_rtl_test =
                generate_test(format!("{name}__content_box_rtl"), &description["contentBoxRtlData"]);

            [
                (format!("{name}__border_box_ltr"), fixture_path, border_box_ltr_test),
                (format!("{name}__content_box_ltr"), fixture_path, content_box_ltr_test),
                (format!("{name}__border_box_rtl"), fixture_path, border_box_rtl_test),
                (format!("{name}__content_box_rtl"), fixture_path, content_box_rtl_test),
            ]
        })
        .collect();

    info!("writing generated test file to disk...");
    let tests_base_path = repo_root.join("tests");
    let xml_base_path = tests_base_path.join("xml");
    let _ = fs::remove_dir_all(&xml_base_path);
    fs::create_dir(&xml_base_path).unwrap();

    let mut mod_file = OpenOptions::new().create(true).append(true).open(xml_base_path.join("mod.rs")).unwrap();
    writeln!(&mut mod_file, "//! Generated XML tests").unwrap();
    writeln!(&mut mod_file, "#![allow(non_snake_case)]").unwrap();

    let mut current_module: Option<OsString> = None;

    for (name, fixture_path, test_body) in test_descs {
        // Create test directory if it doesn't exist
        let test_path_stripped = fixture_path.parent().unwrap().strip_prefix(&fixtures_root).unwrap();
        let test_path = xml_base_path.join(test_path_stripped);
        if !test_path.exists() {
            fs::create_dir(&test_path).unwrap();
        }

        let Some(Component::Normal(module)) = test_path_stripped.components().next() else {
            panic!("unexpected module name")
        };

        if current_module.as_deref() != Some(module) {
            if current_module.is_some() {
                writeln!(&mut mod_file, "}}\n").unwrap();
            }
            current_module = Some(module.to_owned());
            writeln!(&mut mod_file, "mod {} {{", module.display()).unwrap();
        }

        if name.starts_with("grid") {
            writeln!(&mut mod_file, r#"#[cfg(feature = "grid")]"#).unwrap();
        }
        writeln!(
            &mut mod_file,
            "#[test]
            fn {name} () {{
                crate::run_xml_test(\"{}\", \"{name}\");
            }}
        ",
            module.display()
        )
        .unwrap();

        let mut test_filename = test_path.join(&name);
        test_filename.set_extension("xml");
        debug!("writing {} to disk...", name);
        fs::write(test_filename, test_body).unwrap();
    }

    if current_module.is_some() {
        writeln!(&mut mod_file, "}}\n").unwrap();
    }

    info!("formatting the source directory");
    // The tests have already been written at this point, so a formatting failure is not fatal
    match Command::new("cargo").arg("fmt").current_dir(repo_root).status() {
        Ok(status) if !status.success() => warn!("`cargo fmt` failed ({status})"),
        Err(err) => warn!("could not run `cargo fmt`: {err}"),
        Ok(_) => {}
    }
}

/// Copies a child process output stream to the given writer on a background thread.
/// The child gets a pipe rather than gentest's own stdout/stderr handles, so processes
/// that outlive gentest cannot hold gentest's output open; the thread simply stops
/// (and is torn down when the process exits) once the stream ends.
fn forward_output(mut reader: impl std::io::Read + Send + 'static, mut writer: impl std::io::Write + Send + 'static) {
    std::thread::spawn(move || {
        let _ = std::io::copy(&mut reader, &mut writer);
    });
}

/// Report a fatal error and exit without generating any tests
///
/// Aborting rather than continuing means that an incomplete run cannot delete existing tests
/// (test generation removes and re-creates the whole generated tests directory).
fn fatal(message: &str) -> ! {
    error!("{message}");
    eprintln!("\nError: {message}\n\nNo tests have been generated.");
    std::process::exit(1);
}

/// A running ChromeDriver process, along with the URL that it is listening on
struct WebDriver {
    process: Child,
    url: String,
    /// The profile directory that the browser is told to use
    ///
    /// Passing our own directory (rather than letting ChromeDriver pick a temporary one) makes it
    /// possible to identify the browser processes belonging to this run, so that they can be
    /// cleaned up even when ChromeDriver fails to do so.
    profile_dir: PathBuf,
}

impl WebDriver {
    /// Start ChromeDriver on a free port and wait for it to start accepting connections
    fn start(chromedriver_path: &Path) -> Result<Self, String> {
        // Rather than a fixed port, use a free port chosen by the OS. This prevents us from
        // interfering with (or being confused by) any other WebDriver server on the machine,
        // including a ChromeDriver left behind by a previous run of this script.
        let port = free_port().map_err(|err| format!("could not find a free port for ChromeDriver: {err}"))?;

        info!("starting webdriver instance on port {port}");
        // Pipe chromedriver's output and forward it manually rather than letting it inherit
        // gentest's stdout/stderr. Chrome processes spawned by chromedriver would otherwise
        // inherit those file descriptors, and any that outlive gentest would keep a pipeline
        // reading gentest's output (e.g. `just gentest | tail`) blocked waiting for EOF.
        let mut process = Command::new(chromedriver_path)
            .arg(format!("--port={port}"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| format!("could not start ChromeDriver ({}): {err}", chromedriver_path.display()))?;
        forward_output(process.stdout.take().unwrap(), std::io::stdout());
        forward_output(process.stderr.take().unwrap(), std::io::stderr());

        let profile_dir = std::env::temp_dir().join(format!("taffy-gentest-{}", std::process::id()));
        let mut webdriver = WebDriver { process, url: format!("http://127.0.0.1:{port}"), profile_dir };
        webdriver.wait_until_listening(port)?;
        Ok(webdriver)
    }

    /// Wait for ChromeDriver to start accepting connections on `port`
    ///
    /// ChromeDriver only binds its port once it is ready to serve requests, so this avoids sending
    /// requests to a server that is not up yet (and avoids waiting for a fixed amount of time).
    fn wait_until_listening(&mut self, port: u16) -> Result<(), String> {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let start = Instant::now();
        loop {
            // Detect ChromeDriver failing to start (e.g. because the port is already in use)
            // rather than waiting for it to accept a connection that will never come.
            match self.process.try_wait() {
                Ok(Some(status)) => return Err(format!("ChromeDriver exited during startup ({status})")),
                Err(err) => return Err(format!("could not determine whether ChromeDriver is running: {err}")),
                Ok(None) => {}
            }

            if TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_ok() {
                return Ok(());
            }

            if start.elapsed() > TIMEOUT {
                return Err(format!(
                    "ChromeDriver did not start listening on port {port} within {} seconds",
                    TIMEOUT.as_secs()
                ));
            }

            std::thread::sleep(Duration::from_millis(50));
        }
    }

    /// Create a WebDriver session that runs the browser at `chrome_path`
    async fn new_session(&self, chrome_path: &Path) -> Result<Client, String> {
        let mut caps = serde_json::map::Map::new();
        let chrome_opts = serde_json::json!({
            "binary": chrome_path,
            "args": ["--headless", "--no-sandbox", "--disable-gpu", format!("--user-data-dir={}", self.profile_dir.display())],
        });
        caps.insert("goog:chromeOptions".to_string(), chrome_opts);

        info!("spawning webdriver client");
        // Creating the session launches the browser, which can hang indefinitely (for example if
        // the browser cannot start), so give up rather than hanging forever.
        let mut builder = ClientBuilder::native();
        builder.capabilities(caps);
        let client = timeout(TIMEOUT, builder.connect(&self.url))
            .await
            .map_err(|_| {
                format!(
                    "the browser did not start within {} seconds. Check the ChromeDriver output above for details.",
                    TIMEOUT.as_secs()
                )
            })?
            .map_err(|err| format!("could not create a WebDriver session: {err}"))?;

        // Bound how long the browser itself will spend on a single navigation or script, so that a
        // problematic test fixture results in an error rather than a hang.
        let timeouts = TimeoutConfiguration::new(Some(TIMEOUT), Some(TIMEOUT), Some(Duration::ZERO));
        client.update_timeouts(timeouts).await.map_err(|err| format!("could not set WebDriver timeouts: {err}"))?;

        Ok(client)
    }

    /// Close the WebDriver session, which quits the browser that it launched
    async fn close_session(&self, client: Client) {
        info!("closing webdriver session...");
        // If the session is not closed then the browser is left running as an orphan process when
        // ChromeDriver is killed, so log loudly if this fails.
        match timeout(TIMEOUT, client.close()).await {
            Ok(Ok(())) => {}
            Ok(Err(err)) => warn!("could not close the webdriver session (the browser may still be running): {err}"),
            Err(_) => warn!("closing the webdriver session timed out (the browser may still be running)"),
        }
    }

    /// Shut the ChromeDriver process down and clean up after the browser it launched
    fn shutdown(&mut self) {
        self.stop_process();
        self.shutdown_browser();
        self.remove_profile_dir();
    }

    /// Wait for the browser to exit, killing it if it does not
    ///
    /// The browser is identified by its profile directory. Waiting for it matters because a browser
    /// writes to its profile until it exits, which would otherwise repopulate the profile directory
    /// after it has been removed.
    #[cfg(unix)]
    fn shutdown_browser(&self) {
        if !self.profile_dir.exists() {
            return;
        }
        // Only processes launched with our profile directory match this pattern
        let pattern = format!("--user-data-dir={}", self.profile_dir.display());

        // A browser whose session was closed is already on its way out
        if wait_for_exit(&pattern, BROWSER_EXIT_GRACE_PERIOD) {
            return;
        }

        // A browser that has stopped responding is not shut down by ChromeDriver, and would
        // otherwise be left behind as an orphan process holding on to a lot of memory.
        // The `--` is required because the pattern itself starts with dashes.
        let killed = Command::new("pkill").args(["-9", "-f", "--"]).arg(&pattern).status();
        if matches!(killed, Ok(status) if status.success()) {
            warn!("killed browser processes that did not shut down cleanly");
        }
        wait_for_exit(&pattern, TIMEOUT);
    }

    #[cfg(not(unix))]
    fn shutdown_browser(&self) {}

    /// Remove the profile directory of the browser
    ///
    /// This is retried for a short while because a browser that has just been killed may still be
    /// writing to its profile, which makes removing the directory fail.
    fn remove_profile_dir(&self) {
        let start = Instant::now();
        loop {
            match fs::remove_dir_all(&self.profile_dir) {
                Ok(()) => return,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => return,
                Err(err) if start.elapsed() > TIMEOUT => {
                    warn!("could not remove {}: {err}", self.profile_dir.display());
                    return;
                }
                Err(_) => std::thread::sleep(Duration::from_millis(100)),
            }
        }
    }

    /// Stop the ChromeDriver process
    fn stop_process(&mut self) {
        if matches!(self.process.try_wait(), Ok(Some(_))) {
            return;
        }

        info!("stopping webdriver instance...");

        // Ask ChromeDriver to shut down gracefully so that it gets the chance to quit any browser
        // that it is still responsible for. `Child::kill` sends SIGKILL, which would leave such a
        // browser running as an orphan process.
        #[cfg(unix)]
        {
            let _ = Command::new("kill").arg(self.process.id().to_string()).status();
            let start = Instant::now();
            while start.elapsed() < TIMEOUT {
                if matches!(self.process.try_wait(), Ok(Some(_))) {
                    return;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            warn!("ChromeDriver did not shut down gracefully. Killing it.");
        }

        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

impl Drop for WebDriver {
    fn drop(&mut self) {
        // Backstop for exit paths that do not shut the webdriver down explicitly (such as a panic)
        self.shutdown();
    }
}

/// Wait for all of the processes matching `pattern` to exit, returning whether they all did
#[cfg(unix)]
fn wait_for_exit(pattern: &str, patience: Duration) -> bool {
    let start = Instant::now();
    loop {
        // The `--` is required because the patterns used here start with dashes
        let running = Command::new("pgrep").args(["-f", "--"]).arg(pattern).stdout(Stdio::null()).status();
        if !matches!(running, Ok(status) if status.success()) {
            return true;
        }
        if start.elapsed() > patience {
            return false;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// Ask the OS for a free TCP port
fn free_port() -> std::io::Result<u16> {
    // The port is free when the listener is dropped at the end of this function. This is racy in
    // theory, but in practice the port is bound again immediately, and ChromeDriver failing to
    // bind it is detected and reported.
    Ok(TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?.local_addr()?.port())
}

/// Collect the layout of every test fixture from the browser
async fn collect_test_descs(
    client: &Client,
    fixtures: &[(String, PathBuf)],
) -> Result<Vec<(String, PathBuf, Value)>, String> {
    asserts_non_zero_width_scrollbars(client).await?;

    info!("collecting test descriptions");
    let progress = Progress::new(fixtures.len());
    let mut test_descs = Vec::with_capacity(fixtures.len());
    for (index, (name, fixture_path)) in fixtures.iter().enumerate() {
        progress.step(index, name);
        test_descs.push(test_root_element(client, name, fixture_path).await?);
    }
    progress.finish(&format!("collected {} test descriptions", test_descs.len()));
    Ok(test_descs)
}

/// The width of the terminal in characters, defaulting to 80 if it cannot be determined
fn terminal_width() -> usize {
    let from_tput = || {
        let output = Command::new("tput").arg("cols").stderr(Stdio::null()).output().ok()?;
        String::from_utf8_lossy(&output.stdout).trim().parse().ok()
    };
    std::env::var("COLUMNS").ok().and_then(|columns| columns.parse().ok()).or_else(from_tput).unwrap_or(80)
}

/// Reports the progress of a long running sequence of steps
///
/// When stderr is a terminal a single line is updated in place. Otherwise (when the output is being
/// piped to a file or a CI log) one line is logged per step.
struct Progress {
    total: usize,
    in_place: bool,
    /// The width of the terminal, which progress lines are truncated to so that they do not wrap
    terminal_width: usize,
}

impl Progress {
    fn new(total: usize) -> Self {
        Progress { total, in_place: std::io::stderr().is_terminal(), terminal_width: terminal_width() }
    }

    /// Report that the (zero-based) step `index`, described by `label`, is about to start
    fn step(&self, index: usize, label: &str) {
        let (count, total) = (index + 1, self.total);
        if self.in_place {
            // A line that is exactly as wide as the terminal already wraps, hence the -1
            let width = self.terminal_width.saturating_sub(1);
            let line: String = format!("[{count}/{total}] {label}").chars().take(width).collect();
            // `\r` moves back to the start of the line and `\x1b[K` clears the rest of it
            eprint!("\r\x1b[K{line}");
            let _ = std::io::stderr().flush();
        } else {
            info!("[{count}/{total}] {label}");
        }
    }

    /// Replace the progress line with a final message
    fn finish(&self, label: &str) {
        if self.in_place {
            eprintln!("\r\x1b[K{label}");
        } else {
            info!("{label}");
        }
    }
}

async fn asserts_non_zero_width_scrollbars(client: &Client) -> Result<(), String> {
    // Load minimal test page defined in the string
    const TEST_PAGE: &str = r#"data:text/html;charset=utf-8,<html><style>::-webkit-scrollbar{ width: 15px; height: 15px; }</style><body><div style="overflow:scroll" /></body></html>"#;
    client.goto(TEST_PAGE).await.map_err(|err| format!("could not load the scrollbar test page: {err}"))?;

    // Determine the width of the scrollbar
    let scrollbar_width = client
        .execute("return document.body.firstChild.clientWidth - document.body.firstChild.offsetWidth;", vec![])
        .await
        .map_err(|err| format!("could not determine the width of scrollbars: {err}"))?;
    let scrollbar_width =
        scrollbar_width.as_f64().ok_or_else(|| format!("unexpected scrollbar width: {scrollbar_width}"))?;

    if scrollbar_width == 0.0 {
        return Err(concat!(
            "Scrollbar width of zero detected. This test generation script must be run with scrollbars set to take up space.\n",
            "    On macOS this can be done by setting Show Scrollbars to 'always' in the Appearance section of the System Settings app."
        ).to_string());
    }

    Ok(())
}

async fn test_root_element(
    client: &Client,
    name: &str,
    fixture_path: &Path,
) -> Result<(String, PathBuf, Value), String> {
    let url = format!("file://{}", fixture_path.display());

    // The browser occasionally stops responding to a command entirely, so time each command out
    // rather than waiting forever. A fixture normally takes a few milliseconds.
    timeout(TIMEOUT, client.goto(&url))
        .await
        .map_err(|_| format!("timed out loading the test fixture {name} ({})", fixture_path.display()))?
        .map_err(|err| format!("could not load the test fixture {name}: {err}"))?;

    // Navigation can occasionally return before the document is fully loaded, in which case the
    // test helper script and stylesheet may not have been applied yet. Wait for the load event
    // (after which stylesheets are guaranteed to be applied) before reading anything back.
    const AWAIT_LOAD: &str = "
        const done = arguments[arguments.length - 1];
        if (document.readyState === 'complete') { done(); } else { addEventListener('load', done); }
    ";
    timeout(TIMEOUT, client.execute_async(AWAIT_LOAD, vec![]))
        .await
        .map_err(|_| format!("timed out waiting for the test fixture {name} to finish loading"))?
        .map_err(|err| format!("could not wait for the test fixture {name} to finish loading: {err}"))?;

    // Retry a few times as a backstop in case anything is still not ready
    let mut attempts = 0;
    let description = loop {
        let result = timeout(TIMEOUT, client.execute("return getTestData()", vec![]))
            .await
            .map_err(|_| format!("timed out running getTestData() for {name}"))?;
        match result {
            Ok(description) => break description,
            Err(err) if attempts < 3 => {
                attempts += 1;
                warn!("getTestData() failed for {name} (attempt {attempts}): {err}");
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
            Err(err) => return Err(format!("getTestData() failed for {name}: {err}")),
        }
    };
    let description_string =
        description.as_str().ok_or_else(|| format!("unexpected test data for {name}: {description}"))?;
    let description =
        serde_json::from_str(description_string).map_err(|err| format!("invalid test data for {name}: {err}"))?;
    Ok((name.to_string(), fixture_path.to_path_buf(), description))
}

fn generate_test(name: impl AsRef<str>, description: &Value) -> String {
    let use_rounding = description["useRounding"].as_bool().unwrap();

    let mut w =
        XmlWriter::new(Options { use_single_quote: false, indent: Indent::Spaces(2), attributes_indent: Indent::None });
    w.start_element("test");
    w.write_attribute("name", name.as_ref());
    w.write_attribute("use-rounding", &use_rounding);

    // Viewport
    let viewport = &description["viewport"];
    w.start_element("viewport");
    w.write_attribute("width", &serialize_dimension(&viewport["width"]).unwrap());
    w.write_attribute("height", &serialize_dimension(&viewport["height"]).unwrap());
    w.end_element();

    // Input styles
    w.start_element("input");
    generate_node(&mut w, description);
    w.end_element();

    // Expectations
    w.start_element("expectations");
    let use_rounding = description["useRounding"].as_bool().unwrap();
    generate_assertions(&mut w, description, use_rounding);
    w.end_element();

    w.end_document()
}

fn generate_assertions(w: &mut XmlWriter, node: &Value, use_rounding: bool) {
    let layout = if use_rounding { &node["smartRoundedLayout"] } else { &node["unroundedLayout"] };

    let read_f32 = |s: &str| layout[s].as_f64().unwrap() as f32;
    let read_naive_f32 = |s: &str| node["naivelyRoundedLayout"][s].as_f64().unwrap() as f32;
    let scroll_width = (read_f32("scrollWidth") - read_naive_f32("clientWidth")).max(0.0);
    let scroll_height = (read_f32("scrollHeight") - read_naive_f32("clientHeight")).max(0.0);

    fn is_scrollable(overflow: &Value) -> bool {
        match overflow {
            Value::String(ref value) => matches!(value.as_ref(), "hidden" | "scroll" | "auto"),
            _ => false,
        }
    }
    let is_scroll_container = is_scrollable(&node["style"]["overflowX"]) || is_scrollable(&node["style"]["overflowY"]);

    w.start_element("node");

    w.write_attribute("x", &read_f32("x"));
    w.write_attribute("y", &read_f32("y"));
    w.write_attribute("width", &read_f32("width"));
    w.write_attribute("height", &read_f32("height"));

    if is_scroll_container {
        w.write_attribute("scroll_width", &scroll_width);
        w.write_attribute("scroll_height", &scroll_height);
    }

    if let Value::Array(ref value) = node["children"] {
        for child in value {
            generate_assertions(w, child, use_rounding);
        }
    };

    w.end_element();
}

fn generate_node(w: &mut XmlWriter, node: &Value) {
    let style = &node["style"];

    fn get_string_value(value: &Value) -> Option<&str> {
        match value {
            Value::String(ref value) => Some(value),
            _ => None,
        }
    }

    // Handle text/leaf node case
    let text_content = get_string_value(&node["textContent"]);
    if text_content.is_some() {
        w.start_element("text");
    } else {
        w.start_element("div");
    }

    fn maybe_write<T: Display>(w: &mut XmlWriter, name: &str, value: Option<T>) {
        if let Some(attr) = value {
            w.write_attribute(name, &attr);
        }
    }

    maybe_write(w, "display", get_str_attr(&style["display"], None));
    maybe_write(w, "box-sizing", get_str_attr(&style["boxSizing"], Some("border-box")));
    maybe_write(w, "direction", get_str_attr(&style["direction"], None));
    maybe_write(w, "writing-mode", get_str_attr(&style["writingMode"], None));
    maybe_write(w, "position", get_str_attr(&style["position"], Some("relative")));
    maybe_write(w, "float", get_str_attr(&style["cssFloat"], None));
    maybe_write(w, "clear", get_str_attr(&style["clear"], None));
    maybe_write(w, "flex-direction", get_str_attr(&style["flexDirection"], Some("row")));
    maybe_write(w, "flex-wrap", get_str_attr(&style["flexWrap"], Some("nowrap")));
    maybe_write(w, "overflow-x", get_str_attr(&style["overflowX"], Some("visible")));
    maybe_write(w, "overflow-y", get_str_attr(&style["overflowY"], Some("visible")));

    let overflow_x = get_str_attr(&style["overflowX"], Some("visible"));
    let overflow_y = get_str_attr(&style["overflowY"], Some("visible"));
    if overflow_x.is_some() || overflow_y.is_some() {
        maybe_write(w, "scrollbar-width", get_num_attr(&style["scrollbarWidth"], None));
    }

    maybe_write(w, "text-align", get_str_attr(&style["textAlign"], None));
    maybe_write(w, "align-items", get_str_attr(&style["alignItems"], None));
    maybe_write(w, "align-self", get_str_attr(&style["alignSelf"], None));
    maybe_write(w, "justify-items", get_str_attr(&style["justifyItems"], None));
    maybe_write(w, "justify-self", get_str_attr(&style["justifySelf"], None));
    maybe_write(w, "align-content", get_str_attr(&style["alignContent"], None));
    maybe_write(w, "justify-content", get_str_attr(&style["justifyContent"], None));

    maybe_write(w, "flex-grow", get_num_attr(&style["flexGrow"], Some(0.0)));
    maybe_write(w, "flex-shrink", get_num_attr(&style["flexShrink"], Some(1.0)));
    maybe_write(w, "flex-basis", get_dim_attr(&style["flexBasis"], Some("auto")));

    maybe_write(w, "width", get_dim_attr(&style["size"]["width"], Some("auto")));
    maybe_write(w, "height", get_dim_attr(&style["size"]["height"], Some("auto")));
    maybe_write(w, "min-width", get_dim_attr(&style["minSize"]["width"], Some("auto")));
    maybe_write(w, "min-height", get_dim_attr(&style["minSize"]["height"], Some("auto")));
    maybe_write(w, "max-width", get_dim_attr(&style["maxSize"]["width"], Some("auto")));
    maybe_write(w, "max-height", get_dim_attr(&style["maxSize"]["height"], Some("auto")));

    maybe_write(w, "aspect-ratio", get_num_attr(&style["aspectRatio"], None));

    // TODO: null check in no gap case
    maybe_write(w, "row-gap", get_dim_attr(&style["gap"]["row"], None));
    maybe_write(w, "column-gap", get_dim_attr(&style["gap"]["column"], None));

    maybe_write(w, "margin-top", get_dim_attr(&style["margin"]["top"], None));
    maybe_write(w, "margin-left", get_dim_attr(&style["margin"]["left"], None));
    maybe_write(w, "margin-bottom", get_dim_attr(&style["margin"]["bottom"], None));
    maybe_write(w, "margin-right", get_dim_attr(&style["margin"]["right"], None));

    maybe_write(w, "padding-top", get_dim_attr(&style["padding"]["top"], None));
    maybe_write(w, "padding-left", get_dim_attr(&style["padding"]["left"], None));
    maybe_write(w, "padding-bottom", get_dim_attr(&style["padding"]["bottom"], None));
    maybe_write(w, "padding-right", get_dim_attr(&style["padding"]["right"], None));

    maybe_write(w, "border-top", get_dim_attr(&style["border"]["top"], None));
    maybe_write(w, "border-left", get_dim_attr(&style["border"]["left"], None));
    maybe_write(w, "border-bottom", get_dim_attr(&style["border"]["bottom"], None));
    maybe_write(w, "border-right", get_dim_attr(&style["border"]["right"], None));

    maybe_write(w, "top", get_dim_attr(&style["inset"]["top"], None));
    maybe_write(w, "left", get_dim_attr(&style["inset"]["left"], None));
    maybe_write(w, "bottom", get_dim_attr(&style["inset"]["bottom"], None));
    maybe_write(w, "right", get_dim_attr(&style["inset"]["right"], None));

    maybe_write(w, "grid-auto-flow", serialize_grid_auto_flow(&style["gridAutoFlow"]));
    maybe_write(w, "grid-template-rows", serialize_array(&style["gridTemplateRows"], ' ', serialize_track_definition));
    maybe_write(
        w,
        "grid-template-columns",
        serialize_array(&style["gridTemplateColumns"], ' ', serialize_track_definition),
    );
    maybe_write(w, "grid-auto-rows", serialize_array(&style["gridAutoRows"], ' ', serialize_track_definition));
    maybe_write(w, "grid-auto-columns", serialize_array(&style["gridAutoColumns"], ' ', serialize_track_definition));

    maybe_write(w, "grid-row-start", serialize_grid_position(&style["gridRowStart"]));
    maybe_write(w, "grid-row-end", serialize_grid_position(&style["gridRowEnd"]));
    maybe_write(w, "grid-column-start", serialize_grid_position(&style["gridColumnStart"]));
    maybe_write(w, "grid-column-end", serialize_grid_position(&style["gridColumnEnd"]));

    // Recurse into children
    if let Value::Array(ref value) = node["children"] {
        for child_desc in value {
            generate_node(w, child_desc);
        }
    };

    if let Some(text_content) = text_content {
        w.write_text(text_content.trim());
    }

    w.end_element();
}

fn get_str_attr<'a>(value: &'a Value, elide_if: Option<&str>) -> Option<&'a str> {
    if let Value::String(ref value) = value {
        if Some(value.as_str()) != elide_if {
            return Some(value.as_str());
        }
    }

    None
}

fn get_num_attr(value: &Value, elide_if: Option<f64>) -> Option<f64> {
    if let Value::Number(ref value) = value {
        if let Some(num) = value.as_f64() {
            if Some(num) != elide_if {
                return Some(num);
            }
        }
    }

    None
}

fn get_dim_attr(value: &Value, elide_if: Option<&str>) -> Option<Cow<'static, str>> {
    if let Some(attr) = serialize_dimension(value) {
        if Some(attr.as_ref()) != elide_if {
            return Some(attr);
        }
    }
    None
}

fn serialize_dimension(obj: &serde_json::Value) -> Option<Cow<'static, str>> {
    if let Value::Object(ref dimen) = &obj {
        let unit = dimen.get("unit").unwrap();
        let value = dimen.get("value").and_then(|v| v.as_f64());
        match unit {
            Value::String(ref unit) => {
                return Some(match unit.as_str() {
                    "auto" => Cow::from("auto"),
                    "max-content" => Cow::from("max-content"),
                    "min-content" => Cow::from("min-content"),
                    "fit-content" => Cow::from("fit-content"),
                    "stretch" => Cow::from("stretch"),
                    "fit-content-px" => {
                        Cow::from(format!("fit-content({}px)", value.expect("Expected value for fit-content(px) unit")))
                    }
                    "fit-content-percent" => Cow::from(format!(
                        "fit-content({}%)",
                        value.expect("Expected value for fit-content(%) unit") * 100.0
                    )),
                    "px" => Cow::from(format!("{}px", value.expect("Expected value for px unit"))),
                    "percent" => Cow::from(format!("{}%", value.expect("Expected value for % unit") * 100.0)),
                    "fraction" => Cow::from(format!("{}fr", value.expect("Expected value for fr unit"))),
                    _ => unreachable!(),
                })
            }
            _ => panic!("Tried to serialize dimension object, but unit was not a string"),
        }
    };

    None
}

fn serialize_grid_auto_flow(obj: &serde_json::Value) -> Option<Cow<'static, str>> {
    if let Value::Object(ref auto_flow) = &obj {
        let direction = auto_flow.get("direction").unwrap().as_str().unwrap();
        let algorithm = auto_flow.get("algorithm").unwrap().as_str().unwrap();

        let value = match (direction, algorithm) {
            ("row", "sparse") => "row",
            ("column", "sparse") => "column",
            ("row", "dense") => "row dense",
            ("column", "dense") => "column dense",
            _ => unreachable!(),
        };

        return Some(Cow::from(value));
    }
    None
}

fn serialize_grid_position(grid_position: &serde_json::Value) -> Option<Cow<'static, str>> {
    if let Value::Object(ref grid_position) = &grid_position {
        let kind = grid_position.get("kind").unwrap();
        let value = || grid_position.get("value").unwrap().as_f64().unwrap() as f32;

        return match kind {
            Value::String(ref kind) => match kind.as_ref() {
                "auto" => None, //Some(Cow::from("auto")),
                "span" => Some(Cow::from(format!("span {}", value()))),
                "line" => Some(Cow::from((value() as i32).to_string())),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
    }
    None
}

fn serialize_array(value: &Value, sep: char, quoter: impl Fn(&Value) -> Option<Cow<'static, str>>) -> Option<String> {
    match &value {
        Value::Array(ref values) => serialize_value_list(values, sep, quoter),
        _ => None,
    }
}

fn serialize_value_list(
    values: &[Value],
    sep: char,
    quoter: impl Fn(&Value) -> Option<Cow<'static, str>>,
) -> Option<String> {
    let mut out = String::new();
    for item in values {
        out.push_str(quoter(item)?.as_ref());
        out.push(sep);
    }
    out.pop();
    Some(out)
}

fn serialize_track_definition(track_definition: &serde_json::Value) -> Option<Cow<'static, str>> {
    let serde_json::Value::Object(map) = track_definition else {
        return None;
    };

    let kind = map.get("kind").unwrap().as_str().unwrap();
    let name = || map.get("name").unwrap().as_str().unwrap();
    let arguments = || map.get("arguments").unwrap();

    match kind {
        "scalar" => serialize_dimension(track_definition),
        "function" => match (name(), arguments()) {
            ("fit-content", Value::Array(arguments)) => {
                if arguments.len() != 1 {
                    panic!("fit-content function with the wrong number of arguments");
                }
                let limit = serialize_dimension(&arguments[0])?;
                Some(Cow::from(format!("fit-content({limit})")))
            }
            ("minmax", Value::Array(arguments)) => {
                if arguments.len() != 2 {
                    panic!("minmax function with the wrong number of arguments");
                }
                let min = serialize_dimension(&arguments[0])?;
                let max = serialize_dimension(&arguments[1])?;

                Some(Cow::from(format!("minmax({min},{max})")))
            }
            ("repeat", Value::Array(arguments)) => {
                if arguments.len() < 2 {
                    panic!("repeat function with the wrong number of arguments");
                }
                let repetition = match arguments[0] {
                    Value::Object(ref arg) => {
                        let unit = arg.get("unit").unwrap().as_str().unwrap();
                        let value = || arg.get("value").unwrap().as_u64().unwrap() as u16;

                        match unit {
                            "auto-fill" => Cow::from("auto-fill"),
                            "auto-fit" => Cow::from("auto-fit"),
                            "integer" => {
                                let repetition_count = value();
                                Cow::from(repetition_count.to_string())
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                };
                let track_list = serialize_value_list(&arguments[1..], ' ', serialize_track_definition)?;
                Some(Cow::from(format!("repeat({repetition}, {track_list})")))
            }
            // TODO: Add support for fit-content
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
