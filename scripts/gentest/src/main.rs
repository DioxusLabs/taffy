use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use fantoccini::{Client, ClientBuilder};
use log::*;
use serde_json::Value;
use walkdir::WalkDir;
use xmlwriter::{Indent, Options, XmlWriter};

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

    info!("starting webdriver instance");
    let webdriver_url = "http://localhost:4444";
    let mut webdriver_handle = Command::new("chromedriver")
        .arg("--port=4444")
        .spawn()
        .expect("ChromeDriver not found: Make sure you have it installed and added to your PATH.");

    // this is silly, but it works
    std::thread::sleep(std::time::Duration::from_secs(1));

    let mut caps = serde_json::map::Map::new();
    let chrome_opts = serde_json::json!({ "args": ["--headless", "--disable-gpu"] });
    caps.insert("goog:chromeOptions".to_string(), chrome_opts.clone());

    info!("spawning webdriver client and collecting test descriptions");
    let client = ClientBuilder::native().capabilities(caps.clone()).connect(webdriver_url).await.unwrap();

    asserts_non_zero_width_scrollbars(client.clone()).await;

    let mut test_descs = vec![];
    for (name, fixture_path) in fixtures.iter() {
        test_descs.push(test_root_element(client.clone(), name.clone(), fixture_path).await);
    }

    info!("killing webdriver instance...");
    webdriver_handle.kill().unwrap();
    webdriver_handle.wait().unwrap();

    info!("generating test sources and concatenating...");

    let test_descs: Vec<_> = test_descs
        .iter()
        .flat_map(|(name, fixture_path, description)| {
            debug!("generating test contents for {}", &name);

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
        debug!("writing {} to disk...", &name);
        fs::write(test_filename, test_body).unwrap();
    }

    if current_module.is_some() {
        writeln!(&mut mod_file, "}}\n").unwrap();
    }

    info!("formatting the source directory");
    Command::new("cargo").arg("fmt").current_dir(repo_root).status().unwrap();
}

async fn asserts_non_zero_width_scrollbars(client: Client) {
    // Load minimal test page defined in the string
    const TEST_PAGE: &str = r#"data:text/html;charset=utf-8,<html><body><div style="overflow:scroll" /></body></html>"#;
    client.goto(TEST_PAGE).await.unwrap();

    // Determine the width of the scrollbar
    let scrollbar_width = client
        .execute("return document.body.firstChild.clientWidth - document.body.firstChild.offsetWidth;", vec![])
        .await
        .unwrap();
    let Value::Number(scrollbar_width) = scrollbar_width else { panic!("Error retrieving scrollbar_width") };
    let scrollbar_width = scrollbar_width.as_f64().unwrap();

    if scrollbar_width == 0.0 {
        panic!(concat!(
            "\n\n    Error: Scrollbar width of zero detected. This test generation script must be run with scrollbars set to take up space.\n",
            "    On macOS this can be done by setting Show Scrollbars to 'always' in the Appearance section of the System Settings app.\n\n"
        ))
    }
}

async fn test_root_element(client: Client, name: String, fixture_path: impl AsRef<Path>) -> (String, PathBuf, Value) {
    let fixture_path = fixture_path.as_ref();

    let url = format!("file://{}", fixture_path.display());

    client.goto(&url).await.unwrap();
    let description = client.execute("return getTestData()", vec![]).await.unwrap();
    let description_string = description.as_str().unwrap();
    let description = serde_json::from_str(description_string).unwrap();
    (name.to_string(), fixture_path.to_path_buf(), description)
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
