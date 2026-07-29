//! Downloads a [Chrome for Testing](https://developer.chrome.com/blog/chrome-for-testing) build
//! (both the browser and the matching ChromeDriver) into a local directory.
//!
//! The downloaded binaries are cached: if the requested version is already present in the
//! install directory then nothing is downloaded.

use std::fs::{self, File};
use std::io::{self, IsTerminal, Write as _};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use log::*;

/// The endpoint listing the latest Chrome for Testing build of each release channel
const VERSIONS_URL: &str =
    "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";

/// The release channel that we download builds from
const CHANNEL: &str = "Stable";

/// How long to wait for a server to respond before giving up
const TIMEOUT: Duration = Duration::from_secs(10);

/// An HTTP agent that gives up rather than hanging indefinitely if a server stops responding
///
/// No timeout is applied to reading the response body, as the browser archives are large enough
/// that any limit which is safe on a slow connection would be too long to be useful.
fn agent() -> ureq::Agent {
    ureq::Agent::config_builder().timeout_connect(Some(TIMEOUT)).timeout_recv_response(Some(TIMEOUT)).build().into()
}

/// Paths to the binaries of a downloaded Chrome for Testing build
#[derive(Debug, Clone)]
pub struct ChromeForTesting {
    /// The Chrome for Testing version that was downloaded (e.g. "131.0.6778.85")
    pub version: String,
    /// Path to the Chrome for Testing browser binary
    pub chrome: PathBuf,
    /// Path to the ChromeDriver binary matching `chrome`
    pub chromedriver: PathBuf,
}

/// The directory that Chrome for Testing builds are downloaded into by default:
/// the `.chrome-for-testing` directory in the root of the repository.
pub fn default_install_dir() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.parent().and_then(Path::parent).unwrap();
    repo_root.join(".chrome-for-testing")
}

/// Download the latest stable Chrome for Testing build into [`default_install_dir`]
pub fn download_default() -> io::Result<ChromeForTesting> {
    download(&default_install_dir())
}

/// Download the latest stable Chrome for Testing build into `install_dir` (unless it is already
/// present there), returning the paths of the downloaded binaries.
///
/// Each version is downloaded into its own `<install_dir>/<version>` subdirectory, so multiple
/// versions can coexist.
pub fn download(install_dir: &Path) -> io::Result<ChromeForTesting> {
    let platform = platform()?;

    info!("querying the latest {CHANNEL} chrome-for-testing version");
    let versions = get_json(VERSIONS_URL)?;
    let channel = &versions["channels"][CHANNEL];
    let version = channel["version"]
        .as_str()
        .ok_or_else(|| invalid_data(format!("no {CHANNEL} version listed at {VERSIONS_URL}")))?
        .to_string();

    let version_dir = install_dir.join(&version);
    let paths = ChromeForTesting {
        chrome: version_dir.join(chrome_binary_subpath(platform)),
        chromedriver: version_dir.join(chromedriver_binary_subpath(platform)),
        version,
    };

    // The marker file is only written once both archives have been fully extracted, so that a
    // download that is interrupted part way through is not mistaken for a complete one.
    let marker_path = version_dir.join(".complete");
    if marker_path.exists() {
        info!("chrome-for-testing {} ({platform}) is already downloaded", paths.version);
        return Ok(paths);
    }

    info!("downloading chrome-for-testing {} ({platform}) to {}", paths.version, version_dir.display());
    let _ = fs::remove_dir_all(&version_dir);
    fs::create_dir_all(&version_dir)?;
    for binary in ["chrome", "chromedriver"] {
        let url = download_url(&channel["downloads"][binary], platform)
            .ok_or_else(|| invalid_data(format!("no {binary} download listed for platform {platform}")))?;
        download_and_unzip(url, &version_dir)?;
    }
    File::create(&marker_path)?;

    Ok(paths)
}

/// The chrome-for-testing platform identifier for the platform we are running on
fn platform() -> io::Result<&'static str> {
    Ok(match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "linux64",
        ("macos", "aarch64") => "mac-arm64",
        ("macos", "x86_64") => "mac-x64",
        ("windows", "x86_64") => "win64",
        ("windows", "x86") => "win32",
        (os, arch) => return Err(invalid_data(format!("chrome-for-testing does not provide builds for {os} {arch}"))),
    })
}

/// The path of the browser binary relative to the root of the extracted archives
fn chrome_binary_subpath(platform: &str) -> PathBuf {
    let dir = PathBuf::from(format!("chrome-{platform}"));
    if platform.starts_with("mac") {
        dir.join("Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing")
    } else if platform.starts_with("win") {
        dir.join("chrome.exe")
    } else {
        dir.join("chrome")
    }
}

/// The path of the ChromeDriver binary relative to the root of the extracted archives
fn chromedriver_binary_subpath(platform: &str) -> PathBuf {
    let file = if platform.starts_with("win") { "chromedriver.exe" } else { "chromedriver" };
    PathBuf::from(format!("chromedriver-{platform}")).join(file)
}

/// Find the download URL for `platform` in a chrome-for-testing list of downloads
fn download_url(downloads: &serde_json::Value, platform: &str) -> Option<String> {
    downloads.as_array()?.iter().find(|download| download["platform"].as_str() == Some(platform))?["url"]
        .as_str()
        .map(str::to_string)
}

fn get_json(url: &str) -> io::Result<serde_json::Value> {
    let body = agent().get(url).call().map_err(http_error)?.into_body();
    serde_json::from_reader(body.into_reader()).map_err(Into::into)
}

/// Download the zip archive at `url` and extract it into `dest_dir`
///
/// The chrome-for-testing archives each contain a single top-level directory, so this results in
/// `dest_dir` containing one directory per extracted archive.
fn download_and_unzip(url: String, dest_dir: &Path) -> io::Result<()> {
    debug!("downloading {url}");
    let response = agent().get(&url).call().map_err(http_error)?;
    let total_bytes = response
        .headers()
        .get("content-length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());
    let mut body = response.into_body();

    let name = url.rsplit('/').next().unwrap_or(&url).to_string();

    // Stream the archive to a temporary file rather than buffering it in memory (the browser
    // archives are hundreds of megabytes) as reading a zip archive requires random access.
    let zip_path = dest_dir.join(".download.zip");
    let mut zip_file = File::create(&zip_path)?;
    copy_with_progress(&mut body.as_reader(), &mut zip_file, &name, total_bytes)?;
    drop(zip_file);

    progress(&format!("extracting {name}"));
    let mut archive = zip::ZipArchive::new(File::open(&zip_path)?)?;
    archive.extract(dest_dir)?;
    fs::remove_file(&zip_path)?;
    clear_progress();

    Ok(())
}

/// Copy `reader` to `writer`, reporting how much has been copied as it goes
fn copy_with_progress(
    reader: &mut impl io::Read,
    writer: &mut impl io::Write,
    name: &str,
    total_bytes: Option<u64>,
) -> io::Result<()> {
    const MIB: f64 = (1024 * 1024) as f64;

    let mut buffer = vec![0; 128 * 1024];
    let mut copied: u64 = 0;
    let mut last_report = Instant::now();
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        writer.write_all(&buffer[..count])?;
        copied += count as u64;

        // Rate limit the reporting so that it does not dominate the time spent downloading
        if last_report.elapsed() >= Duration::from_millis(100) {
            last_report = Instant::now();
            match total_bytes {
                Some(total) => progress(&format!(
                    "downloading {name} ({:.0}%, {:.0}MiB of {:.0}MiB)",
                    (copied as f64 / total as f64) * 100.0,
                    copied as f64 / MIB,
                    total as f64 / MIB
                )),
                None => progress(&format!("downloading {name} ({:.0}MiB)", copied as f64 / MIB)),
            }
        }
    }
    Ok(())
}

/// Report progress on a single line of the terminal, replacing whatever was reported before it
///
/// Nothing is printed when stderr is not a terminal (the messages are logged instead), as the
/// in place updates would just fill up a log file.
fn progress(message: &str) {
    if std::io::stderr().is_terminal() {
        // `\r` moves back to the start of the line and `\x1b[K` clears the rest of it
        eprint!("\r\x1b[K{message}");
        let _ = std::io::stderr().flush();
    } else {
        debug!("{message}");
    }
}

/// Remove the line last written by [`progress`]
fn clear_progress() {
    if std::io::stderr().is_terminal() {
        eprint!("\r\x1b[K");
        let _ = std::io::stderr().flush();
    }
}

fn http_error(error: ureq::Error) -> io::Error {
    io::Error::other(error)
}

fn invalid_data(message: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
