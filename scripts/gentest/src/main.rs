use selenium_rs::element::Element;
use selenium_rs::webdriver::{Browser, Selector, WebDriver};

use std::process::Command;
use std::path::Path;
use std::fs;
use std::{thread, time};

use json;

fn main() {
    // std::env::current_exe should be <path/to/repo>/scripts/gentest/target/debug/gentest
    // move up five times to get to repo root
    let root_dir = std::env::current_exe().unwrap();
    let repo_root = root_dir.parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .unwrap();

    let fixtures_root = repo_root.join("test_fixtures");

    let mut selenium = Command::new("java")
        .arg("-jar")
        .arg("selenium.jar")
        .current_dir(&format!("{}/scripts/gentest", repo_root.display()))
        .spawn()
        .expect("failed to execute selenium, most likely missing java");

    // Wait for a bit until selenium is up and running...
    let ten_millis = time::Duration::from_millis(2000);
    thread::sleep(ten_millis);

    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();

    let mut src = String::new();
    src.push_str("#[cfg(test)]\n");
    src.push_str("mod generated {\n");

    let fixtures = fs::read_dir(fixtures_root).unwrap();
    for fixture in fixtures {
        let fixture_path = fixture.unwrap().path();
        if fixture_path.is_file() {
            let name = fixture_path.file_stem().unwrap().to_str().unwrap();

            driver.navigate(&format!("file://{}", fixture_path.display()));
            let root_element = driver.query_element(Selector::CSS, "#test-root").unwrap();
            src.push_str(&format!("{}\n", generate_test(&name, root_element)));
        }
    }
    
    src.push_str("}\n");
    fs::write(repo_root.join("tests").join("generated.rs"), src);

    Command::new("cargo")
        .arg("fmt")
        .current_dir(repo_root)
        .status();

    driver.delete_session();
    selenium.kill();
}

fn generate_test(name: &str, root: Element) -> String {
    let description_string = root.get_property("__stretch_description__").unwrap();
    let description = json::parse(&description_string).unwrap();

    let mut src = String::new();
    src.push_str("#[test]\n");
    src.push_str(&format!("fn {}() {{\n", name));
    
    src.push_str("let layout = stretch::compute(\n");
    src.push_str(&format!("&{}", &generate_node(&description)));
    src.push_str(");\n\n");

    src.push_str(&generate_assertions("layout".to_string(), &description));
    src.push_str("}\n");
    src
}

fn generate_assertions(prefix: String, node: &json::JsonValue) -> String {
    let layout = &node["layout"];
    let mut src = String::new();

    src.push_str(&format!("assert_eq!({}.size.width, {:.4});\n", prefix, layout["width"].as_f32().unwrap()));
    src.push_str(&format!("assert_eq!({}.size.height, {:.4});\n", prefix, layout["height"].as_f32().unwrap()));
    src.push_str(&format!("assert_eq!({}.location.x, {:.4});\n", prefix, layout["x"].as_f32().unwrap()));
    src.push_str(&format!("assert_eq!({}.location.y, {:.4});\n\n", prefix, layout["y"].as_f32().unwrap()));

    match node["children"] {
        json::JsonValue::Array(ref value) => {
            for i in 0..value.len() {
                let child = &value[i];
                src.push_str(&generate_assertions(format!("{}.children[{}]", prefix, i), child));
            }
        },
        _ => (),
    };
    src
}

fn generate_node(node: &json::JsonValue) -> String {
    let mut src = String::new();
    src.push_str("stretch::style::Node {\n");

    let style = &node["style"];

    match style["display"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "none" => src.push_str("display: stretch::style::Display::None,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["position_type"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "absolute" => src.push_str("position_type: stretch::style::PositionType::Absolute,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["direction"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "rtl" => src.push_str("direction: stretch::style::Direction::RTL,\n"),
                "ltr" => src.push_str("direction: stretch::style::Direction::LTR,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["flexDirection"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "row-reverse" => src.push_str("flex_direction: stretch::style::FlexDirection::RowReverse,\n"),
                "column" => src.push_str("flex_direction: stretch::style::FlexDirection::Column,\n"),
                "column-reverse" => src.push_str("flex_direction: stretch::style::FlexDirection::ColumnReverse,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["flexWrap"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "wrap" => src.push_str("flex_wrap: stretch::style::FlexWrap::Wrap,\n"),
                "wrap-reverse" => src.push_str("flex_wrap: stretch::style::FlexWrap::WrapReverse,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["overflow"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "hidden" => src.push_str("overflow: stretch::style::Overflow::Hidden,\n"),
                "scroll" => src.push_str("overflow: stretch::style::Overflow::Scroll,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["alignItems"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "flex-start" => src.push_str("align_items: stretch::style::AlignItems::FlexStart,\n"),
                "flex-end" => src.push_str("align_items: stretch::style::AlignItems::FlexEnd,\n"),
                "center" => src.push_str("align_items: stretch::style::AlignItems::Center,\n"),
                "baseline" => src.push_str("align_items: stretch::style::AlignItems::Baseline,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["alignSelf"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "flex-start" => src.push_str("align_self: stretch::style::AlignSelf::FlexStart,\n"),
                "flex-end" => src.push_str("align_self: stretch::style::AlignSelf::FlexEnd,\n"),
                "center" => src.push_str("align_self: stretch::style::AlignSelf::Center,\n"),
                "baseline" => src.push_str("align_self: stretch::style::AlignSelf::Baseline,\n"),
                "stretch" => src.push_str("align_self: stretch::style::AlignSelf::Stretch,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["alignContent"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "flex-start" => src.push_str("align_content: stretch::style::AlignContent::FlexStart,\n"),
                "flex-end" => src.push_str("align_content: stretch::style::AlignContent::FlexEnd,\n"),
                "center" => src.push_str("align_content: stretch::style::AlignContent::Center,\n"),
                "space-between" => src.push_str("align_content: stretch::style::AlignContent::SpaceBetween,\n"),
                "space-around" => src.push_str("align_content: stretch::style::AlignContent::SpaceAround,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["justifyContent"] {
        json::JsonValue::Short(ref value) => {
            match value.as_ref() {
                "flex-end" => src.push_str("justify_content: stretch::style::JustifyContent::FlexEnd,\n"),
                "center" => src.push_str("justify_content: stretch::style::JustifyContent::Center,\n"),
                "space-between" => src.push_str("justify_content: stretch::style::JustifyContent::SpaceBetween,\n"),
                "space-around" => src.push_str("justify_content: stretch::style::JustifyContent::SpaceAround,\n"),
                "space-evenly" => src.push_str("justify_content: stretch::style::JustifyContent::SpaceEvenly,\n"),
                _ => (),
            };
        },
        _ => (),
    };

    match style["flexGrow"] {
        json::JsonValue::Number(value) => {
            let value: f32 = value.into();
            src.push_str(&format!("flex_grow: {:.4},\n", value))
        },
        _ => (),
    };

    match style["flexShrink"] {
        json::JsonValue::Number(value) => {
            let value: f32 = value.into();
            src.push_str(&format!("flex_shrink: {:.4},\n", value))
        },
        _ => (),
    };

    match style["flexBasis"] {
        json::JsonValue::Object(ref value) => src.push_str(&format!("flex_basis: {},\n", generate_dimension(value))),
        _ => (),
    };

    match style["size"] {
        json::JsonValue::Object(ref value) => src.push_str(&format!("size: {},\n", generate_size(value))),
        _ => (),
    };

    match style["min_size"] {
        json::JsonValue::Object(ref value) => src.push_str(&format!("min_size: {},\n", generate_size(value))),
        _ => (),
    };

    match style["max_size"] {
        json::JsonValue::Object(ref value) => src.push_str(&format!("max_size: {},\n", generate_size(value))),
        _ => (),
    };

    match style["margin"] {
        json::JsonValue::Object(ref value) => src.push_str(&format!("margin: {},\n", generate_edges(value))),
        _ => (),
    };

    match style["padding"] {
        json::JsonValue::Object(ref value) => src.push_str(&format!("padding: {},\n", generate_edges(value))),
        _ => (),
    };

    match style["position"] {
        json::JsonValue::Object(ref value) => src.push_str(&format!("position: {},\n", generate_edges(value))),
        _ => (),
    };

    match style["border"] {
        json::JsonValue::Object(ref value) => src.push_str(&format!("border: {},\n", generate_edges(value))),
        _ => (),
    };

    match node["children"] {
        json::JsonValue::Array(ref value) => {
            if value.len() > 0 {
                src.push_str("children: vec![\n");
                value.iter().for_each(|child| {
                    src.push_str(&format!("{},\n", generate_node(child)));
                });
                src.push_str("],\n");
            }
        },
        _ => (),
    };

    src.push_str("..Default::default()\n");
    src.push_str("}\n");
    src
}

fn generate_size(size: &json::object::Object) -> String {
    let mut src = String::new();
    src.push_str("stretch::geometry::Size {\n");

    match size.get("width") {
        Some(value) => match value {
            json::JsonValue::Object(ref value) => src.push_str(&format!("width: {},\n", generate_dimension(value))),
            _ => (),
        },
        None => (),
    };

    match size.get("height") {
        Some(value) => match value {
            json::JsonValue::Object(ref value) => src.push_str(&format!("height: {},\n", generate_dimension(value))),
            _ => (),
        },
        None => (),
    };

    src.push_str("..Default::default()\n");
    src.push_str("}\n");
    src
}

fn generate_dimension(dimen: &json::object::Object) -> String {
    let unit = dimen.get("unit").unwrap();
    let value = || dimen.get("value").unwrap().as_f32().unwrap();

    match unit {
        json::JsonValue::Short(ref unit) => {
            match unit.as_ref() {
                "auto" => format!("stretch::style::Dimension::Auto"),
                "points" => format!("stretch::style::Dimension::Points({:.4})", value()),
                "percent" => format!("stretch::style::Dimension::Percent({:.4})", value()),
                _ => panic!(),
            }
        },
        _ => panic!(),
    }
}

fn generate_edges(dimen: &json::object::Object) -> String {
    let mut src = String::new();
    src.push_str("stretch::geometry::Rect {\n");

    match dimen.get("start") {
        Some(value) => match value {
            json::JsonValue::Object(ref value) => src.push_str(&format!("start: {},\n", generate_dimension(value))),
            _ => (),
        },
        None => (),
    };

    match dimen.get("end") {
        Some(value) => match value {
            json::JsonValue::Object(ref value) => src.push_str(&format!("end: {},\n", generate_dimension(value))),
            _ => (),
        },
        None => (),
    };

    match dimen.get("top") {
        Some(value) => match value {
            json::JsonValue::Object(ref value) => src.push_str(&format!("top: {},\n", generate_dimension(value))),
            _ => (),
        },
        None => (),
    };

    match dimen.get("bottom") {
        Some(value) => match value {
            json::JsonValue::Object(ref value) => src.push_str(&format!("bottom: {},\n", generate_dimension(value))),
            _ => (),
        },
        None => (),
    };

    src.push_str("..Default::default()\n");
    src.push_str("}\n");
    src
}
