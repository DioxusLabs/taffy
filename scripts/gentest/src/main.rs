use std::ffi::OsStr;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use convert_case::{Case, Casing};
use fantoccini::{Client, ClientBuilder};
use log::*;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde_json::Value;
use syn::Ident;
use walkdir::WalkDir;

macro_rules! dim_quoted_renamed {
    ($obj:ident, $in_name:ident, $out_name:ident, $value_mapper:ident, $default:expr) => {
        let $out_name = match $obj.get(stringify!($in_name)) {
            Some(Value::Object(ref value)) => {
                let dim = $value_mapper(value);
                quote!($out_name: #dim,)
            }
            _ => {
                let dim = $default;
                quote!($out_name: #dim,)
            }
        };
    };
}

macro_rules! dim_quoted {
    ($obj:ident, $dim_name:ident, $value_mapper: ident, $default:expr) => {
        dim_quoted_renamed!($obj, $dim_name, $dim_name, $value_mapper, $default)
    };
}

macro_rules! edges_quoted {
    ($style:ident, $val:ident, $value_mapper:ident, $default_value: expr) => {
        let $val = match $style[stringify!($val)] {
            Value::Object(ref value) => {
                dim_quoted!(value, left, $value_mapper, $default_value);
                dim_quoted!(value, right, $value_mapper, $default_value);
                dim_quoted!(value, top, $value_mapper, $default_value);
                dim_quoted!(value, bottom, $value_mapper, $default_value);

                let edges = quote!(taffy::geometry::Rect {
                    #left #right #top #bottom
                });

                quote!($val: #edges,)
            },
            _ => quote!(),
        };
    };
}

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
    fixtures.sort_unstable_by_key(|f| f.0.clone());

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
        .map(|(name, fixture_path, description)| {
            debug!("generating test contents for {}", &name);

            let border_box_test = generate_test(format!("{name}__border_box"), &description["borderBoxData"]);
            let content_box_test = generate_test(format!("{name}__content_box"), &description["contentBoxData"]);

            let test_file_content = [border_box_test, content_box_test].map(|test| test.to_string()).join("\n\n");

            (name.clone(), fixture_path, test_file_content)
        })
        .collect();

    info!("writing generated test file to disk...");
    let tests_base_path = repo_root.join("tests").join("generated");
    fs::remove_dir_all(&tests_base_path).unwrap();
    fs::create_dir(&tests_base_path).unwrap();

    // Open base mod.rs file for appending
    let mut base_mod_file = OpenOptions::new().create(true).append(true).open(tests_base_path.join("mod.rs")).unwrap();

    for (name, fixture_path, test_body) in test_descs {
        // Create test directory if it doesn't exist
        let test_path_stripped = fixture_path.parent().unwrap().strip_prefix(&fixtures_root).unwrap();
        let test_path = tests_base_path.join(test_path_stripped);
        if !test_path.exists() {
            fs::create_dir(&test_path).unwrap();

            let ident = Ident::new(test_path_stripped.to_str().unwrap(), Span::call_site());
            let token = quote!(mod #ident;);
            writeln!(&mut base_mod_file, "{token}").unwrap();
        }
        // Open mod file in current folder
        let mod_path = test_path.join("mod.rs");
        let mut mod_file = OpenOptions::new().create(true).append(true).open(mod_path).unwrap();

        let name_ident = Ident::new(&name, Span::call_site());
        let token = if name.starts_with("grid") {
            quote!(#[cfg(feature = "grid")] mod #name_ident;)
        } else {
            quote!(mod #name_ident;)
        };
        writeln!(&mut mod_file, "{token}").unwrap();
        let mut test_filename = test_path.join(&name);
        test_filename.set_extension("rs");
        debug!("writing {} to disk...", &name);
        fs::write(test_filename, test_body).unwrap();
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

fn generate_test(name: impl AsRef<str>, description: &Value) -> TokenStream {
    let name = name.as_ref();
    let name = Ident::new(name, Span::call_site());
    let use_rounding = description["useRounding"].as_bool().unwrap();
    let assertions = generate_assertions("node", description, use_rounding);
    let node_description = generate_node("node", description);

    let set_rounding_mode = if use_rounding { quote!() } else { quote!(taffy.disable_rounding();) };

    // Compute available space
    let viewport = &description["viewport"];
    let available_space = if viewport["width"]["unit"] == "max-content" && viewport["height"]["unit"] == "max-content" {
        quote!(taffy::geometry::Size::MAX_CONTENT)
    } else {
        dim_quoted!(viewport, width, generate_available_space, quote!(taffy::style::AvailableSpace::MAX_CONTENT));
        dim_quoted!(viewport, height, generate_available_space, quote!(taffy::style::AvailableSpace::MAX_CONTENT));
        quote!(
            taffy::geometry::Size {
                #width #height
            }
        )
    };

    quote!(
        #[test]
        #[allow(non_snake_case)]
        fn #name() {
            #[allow(unused_imports)]
            use taffy::{Layout, prelude::*};
            let mut taffy = crate::new_test_tree();
            #set_rounding_mode
            #node_description
            taffy.compute_layout_with_measure(node, #available_space, crate::test_measure_function).unwrap();

            println!("\nComputed tree:");
            taffy.print_tree(node);
            println!();

            #assertions
        }
    )
}

fn generate_assertions(ident: &str, node: &Value, use_rounding: bool) -> TokenStream {
    let layout = if use_rounding { &node["smartRoundedLayout"] } else { &node["unroundedLayout"] };

    fn is_scrollable(overflow: &Value) -> bool {
        match overflow {
            Value::String(ref value) => matches!(value.as_ref(), "hidden" | "scroll" | "auto"),
            _ => false,
        }
    }
    let is_scroll_container = is_scrollable(&node["style"]["overflowX"]) || is_scrollable(&node["style"]["overflowY"]);

    let read_f32 = |s: &str| layout[s].as_f64().unwrap() as f32;
    let read_naive_f32 = |s: &str| node["naivelyRoundedLayout"][s].as_f64().unwrap() as f32;
    let width = read_f32("width");
    let height = read_f32("height");
    let x = read_f32("x");
    let y = read_f32("y");

    let scroll_width = (read_f32("scrollWidth") - read_naive_f32("clientWidth")).max(0.0);
    let scroll_height = (read_f32("scrollHeight") - read_naive_f32("clientHeight")).max(0.0);

    let children = {
        let mut c = Vec::new();
        if let Value::Array(ref value) = node["children"] {
            for (idx, child) in value.iter().enumerate() {
                c.push(generate_assertions(&format!("{ident}{idx}"), child, use_rounding));
            }
        };
        c.into_iter().fold(quote!(), |a, b| quote!(#a #b))
    };

    let ident = Ident::new(ident, Span::call_site());

    // The scrollWidth reading from chrome is only accurate if the node is scroll container. So only assert in that case.
    // TODO: accurately test content size in the non-scroll-container case.
    let scroll_assertions = if is_scroll_container {
        quote!(
            #[cfg(feature = "content_size")]
            assert_eq!(layout.scroll_width(), #scroll_width, "scroll_width of node {:?}. Expected {}. Actual {}", #ident, #scroll_width, layout.scroll_width());
            #[cfg(feature = "content_size")]
            assert_eq!(layout.scroll_height(), #scroll_height, "scroll_height of node {:?}. Expected {}. Actual {}", #ident, #scroll_height, layout.scroll_height());
        )
    } else {
        quote!()
    };

    if use_rounding {
        quote!(
            let layout = taffy.layout(#ident).unwrap();
            let Layout { size, location, .. } = layout;
            assert_eq!(size.width, #width, "width of node {:?}. Expected {}. Actual {}", #ident,  #width, size.width);
            assert_eq!(size.height, #height, "height of node {:?}. Expected {}. Actual {}", #ident,  #height, size.height);
            assert_eq!(location.x, #x, "x of node {:?}. Expected {}. Actual {}", #ident,  #x, location.x);
            assert_eq!(location.y, #y, "y of node {:?}. Expected {}. Actual {}", #ident,  #y, location.y);
            #scroll_assertions

            #children
        )
    } else {
        quote!(
            let layout = taffy.layout(#ident).unwrap();
            let Layout { size, location, .. } = layout;
            assert!((size.width - #width).abs() < 0.1, "width of node {:?}. Expected {}. Actual {}", #ident, #width, size.width);
            assert!((size.height - #height).abs() < 0.1, "height of node {:?}. Expected {}. Actual {}", #ident, #height, size.height);
            assert!((location.x - #x).abs() < 0.1, "x of node {:?}. Expected {}. Actual {}", #ident, #x, location.x);
            assert!((location.y - #y).abs() < 0.1, "y of node {:?}. Expected {}. Actual {}", #ident, #y, location.y);
            #scroll_assertions

            #children
        )
    }
}

fn generate_node(ident: &str, node: &Value) -> TokenStream {
    let style = &node["style"];

    fn snake_case_ident(ident_name: &str) -> Ident {
        let name_snake_case = ident_name.to_case(Case::Snake);
        format_ident!("{}", name_snake_case)
    }

    fn quote_object_value(
        prop_name: &str,
        style: &Value,
        quoter: impl Fn(&serde_json::Map<String, Value>) -> TokenStream,
    ) -> Option<TokenStream> {
        match style[prop_name.to_case(Case::Camel)] {
            Value::Object(ref value) => Some(quoter(value)),
            _ => None,
        }
    }

    fn quote_prop(prop_name: &str, value: TokenStream) -> TokenStream {
        let ident = snake_case_ident(prop_name);
        quote!(#ident: #value,)
    }

    fn quote_object_prop(
        prop_name: &str,
        style: &Value,
        quoter: impl Fn(&serde_json::Map<String, Value>) -> TokenStream,
    ) -> TokenStream {
        match quote_object_value(prop_name, style, quoter) {
            Some(prop_value) => quote_prop(prop_name, prop_value),
            None => quote!(),
        }
    }

    fn quote_array_prop(prop_name: &str, style: &Value, quoter: impl Fn(&[Value]) -> TokenStream) -> TokenStream {
        let prop_name_snake_case = prop_name.to_case(Case::Snake);
        let prop_name_camel_case = prop_name.to_case(Case::Camel);
        let prop_name_ident = format_ident!("{}", prop_name_snake_case);
        match style[prop_name_camel_case] {
            Value::Array(ref value) => {
                let prop_value = quoter(value);
                quote!(#prop_name_ident: #prop_value,)
            }
            _ => quote!(),
        }
    }

    fn get_string_value<'a, 'b, 'c: 'b>(prop_name: &'a str, style: &'c Value) -> Option<&'b str> {
        match style[prop_name.to_case(Case::Camel)] {
            Value::String(ref value) => Some(value),
            _ => None,
        }
    }

    fn get_number_value<'a, 'b, 'c: 'b>(prop_name: &'a str, style: &'c Value) -> Option<f32> {
        match style[prop_name.to_case(Case::Camel)] {
            Value::Number(ref value) => Some(value.as_f64().unwrap() as f32),
            _ => None,
        }
    }

    fn quote_number_prop(prop_name: &str, style: &Value, quoter: impl Fn(f32) -> TokenStream) -> TokenStream {
        let prop_name_snake_case = prop_name.to_case(Case::Snake);
        let prop_name_camel_case = prop_name.to_case(Case::Camel);
        let prop_name_ident = format_ident!("{}", prop_name_snake_case);
        match style[prop_name_camel_case] {
            Value::Number(ref value) => {
                let prop_value = quoter(value.as_f64().unwrap() as f32);
                quote!(#prop_name_ident: #prop_value,)
            }
            _ => quote!(),
        }
    }

    let display = match style["display"] {
        Value::String(ref value) => match value.as_ref() {
            "none" => quote!(display: taffy::style::Display::None,),
            "block" => quote!(display: taffy::style::Display::Block,),
            "grid" => quote!(display: taffy::style::Display::Grid,),
            _ => quote!(display: taffy::style::Display::Flex,),
        },
        _ => quote!(),
    };

    let box_sizing = match style["boxSizing"] {
        Value::String(ref value) => match value.as_ref() {
            "content-box" => quote!(box_sizing: taffy::style::BoxSizing::ContentBox,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let position = match style["position"] {
        Value::String(ref value) => match value.as_ref() {
            "absolute" => quote!(position: taffy::style::Position::Absolute,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let direction = match style["direction"] {
        Value::String(ref value) => match value.as_ref() {
            "rtl" => quote!(direction: taffy::style::Direction::Rtl,),
            "ltr" => quote!(direction: taffy::style::Direction::Ltr,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let float = match style["cssFloat"] {
        Value::String(ref value) => match value.as_ref() {
            "left" => quote!(float: taffy::style::Float::Left,),
            "right" => quote!(float: taffy::style::Float::Right,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let clear = match style["clear"] {
        Value::String(ref value) => match value.as_ref() {
            "left" => quote!(clear: taffy::style::Clear::Left,),
            "right" => quote!(clear: taffy::style::Clear::Right,),
            "both" => quote!(clear: taffy::style::Clear::Right,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let flex_direction = match style["flexDirection"] {
        Value::String(ref value) => match value.as_ref() {
            "row-reverse" => quote!(flex_direction: taffy::style::FlexDirection::RowReverse,),
            "column" => quote!(flex_direction: taffy::style::FlexDirection::Column,),
            "column-reverse" => quote!(flex_direction: taffy::style::FlexDirection::ColumnReverse,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let flex_wrap = match style["flexWrap"] {
        Value::String(ref value) => match value.as_ref() {
            "wrap" => quote!(flex_wrap: taffy::style::FlexWrap::Wrap,),
            "wrap-reverse" => quote!(flex_wrap: taffy::style::FlexWrap::WrapReverse,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    fn quote_overflow(overflow: &Value) -> Option<TokenStream> {
        match overflow {
            Value::String(ref value) => match value.as_ref() {
                "hidden" => Some(quote!(taffy::style::Overflow::Hidden)),
                "scroll" => Some(quote!(taffy::style::Overflow::Scroll)),
                "auto" => Some(quote!(taffy::style::Overflow::Auto)),
                _ => None,
            },
            _ => None,
        }
    }
    let overflow_x = quote_overflow(&style["overflowX"]);
    let overflow_y = quote_overflow(&style["overflowY"]);
    let (overflow, scrollbar_width) = if overflow_x.is_some() || overflow_y.is_some() {
        let overflow_x = overflow_x.unwrap_or(quote!(taffy::style::Overflow::Visible));
        let overflow_y = overflow_y.unwrap_or(quote!(taffy::style::Overflow::Visible));
        let overflow = quote!(overflow: taffy::geometry::Point { x: #overflow_x, y: #overflow_y },);
        let scrollbar_width = quote_number_prop("scrollbar_width", style, |value: f32| quote!(#value));
        (overflow, scrollbar_width)
    } else {
        (quote!(), quote!())
    };

    let text_align = match style["textAlign"] {
        Value::String(ref value) => match value.as_ref() {
            "-webkit-left" => quote!(text_align: taffy::style::TextAlign::LegacyLeft,),
            "-webkit-right" => quote!(text_align: taffy::style::TextAlign::LegacyRight,),
            "-webkit-center" => quote!(text_align: taffy::style::TextAlign::LegacyCenter,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_items = match style["alignItems"] {
        Value::String(ref value) => match value.as_ref() {
            "start" => quote!(align_items: Some(taffy::style::AlignItems::Start),),
            "end" => quote!(align_items: Some(taffy::style::AlignItems::End),),
            "flex-start" => quote!(align_items: Some(taffy::style::AlignItems::FlexStart),),
            "flex-end" => quote!(align_items: Some(taffy::style::AlignItems::FlexEnd),),
            "center" => quote!(align_items: Some(taffy::style::AlignItems::Center),),
            "baseline" => quote!(align_items: Some(taffy::style::AlignItems::Baseline),),
            "stretch" => quote!(align_items: Some(taffy::style::AlignItems::Stretch),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_self = match style["alignSelf"] {
        Value::String(ref value) => match value.as_ref() {
            "start" => quote!(align_self: Some(taffy::style::AlignSelf::Start),),
            "end" => quote!(align_self: Some(taffy::style::AlignSelf::End),),
            "flex-start" => quote!(align_self: Some(taffy::style::AlignSelf::FlexStart),),
            "flex-end" => quote!(align_self: Some(taffy::style::AlignSelf::FlexEnd),),
            "center" => quote!(align_self: Some(taffy::style::AlignSelf::Center),),
            "baseline" => quote!(align_self: Some(taffy::style::AlignSelf::Baseline),),
            "stretch" => quote!(align_self: Some(taffy::style::AlignSelf::Stretch),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let justify_items = match style["justifyItems"] {
        Value::String(ref value) => match value.as_ref() {
            "start" => quote!(justify_items: Some(taffy::style::JustifyItems::Start),),
            "end" => quote!(justify_items: Some(taffy::style::JustifyItems::End),),
            "flex-start" => quote!(justify_items: Some(taffy::style::JustifyItems::FlexStart),),
            "flex-end" => quote!(justify_items: Some(taffy::style::JustifyItems::FlexEnd),),
            "center" => quote!(justify_items: Some(taffy::style::JustifyItems::Center),),
            "baseline" => quote!(justify_items: Some(taffy::style::JustifyItems::Baseline),),
            "stretch" => quote!(justify_items: Some(taffy::style::JustifyItems::Stretch),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let justify_self = match style["justifySelf"] {
        Value::String(ref value) => match value.as_ref() {
            "start" => quote!(justify_self: Some(taffy::style::JustifySelf::Start),),
            "end" => quote!(justify_self: Some(taffy::style::JustifySelf::End),),
            "flex-start" => quote!(justify_self: Some(taffy::style::JustifySelf::FlexStart),),
            "flex-end" => quote!(justify_self: Some(taffy::style::JustifySelf::FlexEnd),),
            "center" => quote!(justify_self: Some(taffy::style::JustifySelf::Center),),
            "baseline" => quote!(justify_self: Some(taffy::style::JustifySelf::Baseline),),
            "stretch" => quote!(justify_self: Some(taffy::style::JustifySelf::Stretch),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_content = match style["alignContent"] {
        Value::String(ref value) => match value.as_ref() {
            "start" => quote!(align_content: Some(taffy::style::AlignContent::Start),),
            "end" => quote!(align_content: Some(taffy::style::AlignContent::End),),
            "flex-start" => quote!(align_content: Some(taffy::style::AlignContent::FlexStart),),
            "flex-end" => quote!(align_content: Some(taffy::style::AlignContent::FlexEnd),),
            "center" => quote!(align_content: Some(taffy::style::AlignContent::Center),),
            "stretch" => quote!(align_content: Some(taffy::style::AlignContent::Stretch),),
            "space-between" => quote!(align_content: Some(taffy::style::AlignContent::SpaceBetween),),
            "space-around" => quote!(align_content: Some(taffy::style::AlignContent::SpaceAround),),
            "space-evenly" => quote!(align_content: Some(taffy::style::AlignContent::SpaceEvenly),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let justify_content = match style["justifyContent"] {
        Value::String(ref value) => match value.as_ref() {
            "start" => quote!(justify_content: Some(taffy::style::JustifyContent::Start),),
            "end" => quote!(justify_content: Some(taffy::style::JustifyContent::End),),
            "flex-start" => quote!(justify_content: Some(taffy::style::JustifyContent::FlexStart),),
            "flex-end" => quote!(justify_content: Some(taffy::style::JustifyContent::FlexEnd),),
            "center" => quote!(justify_content: Some(taffy::style::JustifyContent::Center),),
            "stretch" => quote!(justify_content: Some(taffy::style::AlignContent::Stretch),),
            "space-between" => quote!(justify_content: Some(taffy::style::JustifyContent::SpaceBetween),),
            "space-around" => quote!(justify_content: Some(taffy::style::JustifyContent::SpaceAround),),
            "space-evenly" => quote!(justify_content: Some(taffy::style::JustifyContent::SpaceEvenly),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let flex_grow = quote_number_prop("flex_grow", style, |value: f32| quote!(#value));
    let flex_shrink = quote_number_prop("flex_shrink", style, |value: f32| quote!(#value));

    let flex_basis = quote_object_prop("flex_basis", style, generate_dimension);
    let size = quote_object_prop("size", style, generate_size);
    let min_size = quote_object_prop("min_size", style, generate_size);
    let max_size = quote_object_prop("max_size", style, generate_size);
    let aspect_ratio = quote_number_prop("aspect_ratio", style, |value: f32| quote!(Some(#value)));

    let gap = quote_object_prop("gap", style, generate_gap);

    let grid_template_rows = quote_array_prop("grid_template_rows", style, generate_track_definition_list);
    let grid_template_columns = quote_array_prop("grid_template_columns", style, generate_track_definition_list);
    let grid_auto_rows = quote_array_prop("grid_auto_rows", style, generate_track_definition_list);
    let grid_auto_columns = quote_array_prop("grid_auto_columns", style, generate_track_definition_list);
    let grid_auto_flow = quote_object_prop("grid_auto_flow", style, generate_grid_auto_flow);

    let default_grid_placement = quote!(taffy::style::GridPlacement::Auto);

    let grid_row_start = quote_object_value("grid_row_start", style, generate_grid_position);
    let grid_row_end = quote_object_value("grid_row_end", style, generate_grid_position);
    let grid_row = if grid_row_start.is_some() || grid_row_end.is_some() {
        quote_prop(
            "grid_row",
            generate_line(
                grid_row_start.unwrap_or(default_grid_placement.clone()),
                grid_row_end.unwrap_or(default_grid_placement.clone()),
            ),
        )
    } else {
        quote!()
    };

    let grid_column_start = quote_object_value("grid_column_start", style, generate_grid_position);
    let grid_column_end = quote_object_value("grid_column_end", style, generate_grid_position);
    let grid_column = if grid_column_start.is_some() || grid_column_end.is_some() {
        quote_prop(
            "grid_column",
            generate_line(
                grid_column_start.unwrap_or(default_grid_placement.clone()),
                grid_column_end.unwrap_or(default_grid_placement),
            ),
        )
    } else {
        quote!()
    };

    let text_content = get_string_value("text_content", node);
    let writing_mode = get_string_value("writingMode", style);
    let raw_aspect_ratio = get_number_value("aspect_ratio", style);
    let node_context: Option<_> = text_content.map(|text| generate_node_context(text, writing_mode, raw_aspect_ratio));

    edges_quoted!(style, margin, generate_length_percentage_auto, quote!(zero()));
    edges_quoted!(style, padding, generate_length_percentage, quote!(zero()));
    edges_quoted!(style, border, generate_length_percentage, quote!(zero()));
    edges_quoted!(style, inset, generate_length_percentage_auto, quote!(auto()));

    // Quote children
    let child_descriptions: Vec<Value> = match node["children"] {
        Value::Array(ref value) => value.clone(),
        _ => vec![],
    };
    let has_children = !child_descriptions.is_empty();
    let (children_body, children) = if has_children {
        let body = child_descriptions
            .iter()
            .enumerate()
            .map(|(i, child)| generate_node(&format!("{ident}{i}"), child))
            .collect();
        let idents = child_descriptions
            .iter()
            .enumerate()
            .map(|(i, _)| Ident::new(&format!("{ident}{i}"), Span::call_site()))
            .collect::<Vec<_>>();
        (body, quote!(&[#(#idents),*]))
    } else {
        (quote!(), quote!())
    };

    let ident = Ident::new(ident, Span::call_site());

    let style = quote!(taffy::style::Style {
        #display
        #box_sizing
        #direction
        #position
        #float
        #clear
        #text_align
        #flex_direction
        #flex_wrap
        #overflow
        #scrollbar_width
        #align_items
        #align_self
        #justify_items
        #justify_self
        #align_content
        #justify_content
        #flex_grow
        #flex_shrink
        #flex_basis
        #gap
        #grid_template_rows
        #grid_template_columns
        #grid_auto_rows
        #grid_auto_columns
        #grid_auto_flow
        #grid_row
        #grid_column
        #size
        #min_size
        #max_size
        #aspect_ratio
        #margin
        #padding
        #inset
        #border
        ..Default::default()
    });

    if has_children {
        quote!(
            #children_body
            let #ident = taffy.new_with_children(#style,#children).unwrap();
        )
    } else if node_context.is_some() {
        quote!(let #ident = taffy.new_leaf_with_context(#style,#node_context,).unwrap();)
    } else {
        quote!(let #ident = taffy.new_leaf(#style).unwrap();)
    }
}

fn generate_size(size: &serde_json::Map<String, Value>) -> TokenStream {
    dim_quoted!(size, width, generate_dimension, quote!(auto()));
    dim_quoted!(size, height, generate_dimension, quote!(auto()));
    quote!(
        taffy::geometry::Size {
            #width #height
        }
    )
}

fn generate_gap(size: &serde_json::Map<String, Value>) -> TokenStream {
    dim_quoted_renamed!(size, column, width, generate_length_percentage, quote!(zero()));
    dim_quoted_renamed!(size, row, height, generate_length_percentage, quote!(zero()));
    quote!(
        taffy::geometry::Size {
            #width #height
        }
    )
}

fn generate_length_percentage(dimen: &serde_json::Map<String, Value>) -> TokenStream {
    let unit = dimen.get("unit").unwrap();
    let value = || dimen.get("value").unwrap().as_f64().unwrap() as f32;

    match unit {
        Value::String(ref unit) => match unit.as_ref() {
            "px" => {
                let value = value();
                quote!(length(#value))
            }
            "percent" => {
                let value = value();
                quote!(percent(#value))
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_length_percentage_auto(dimen: &serde_json::Map<String, Value>) -> TokenStream {
    let unit = dimen.get("unit").unwrap();
    let value = || dimen.get("value").unwrap().as_f64().unwrap() as f32;

    match unit {
        Value::String(ref unit) => match unit.as_ref() {
            "auto" => quote!(taffy::style::LengthPercentageAuto::AUTO),
            "px" => {
                let value = value();
                quote!(length(#value))
            }
            "percent" => {
                let value = value();
                quote!(percent(#value))
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_dimension(dimen: &serde_json::Map<String, Value>) -> TokenStream {
    let unit = dimen.get("unit").unwrap();
    let value = || dimen.get("value").unwrap().as_f64().unwrap() as f32;

    match unit {
        Value::String(ref unit) => match unit.as_ref() {
            "auto" => quote!(taffy::style::Dimension::AUTO),
            "px" => {
                let value = value();
                quote!(taffy::style::Dimension::from_length(#value))
            }
            "percent" => {
                let value = value();
                quote!(taffy::style::Dimension::from_percent(#value))
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_available_space(dimen: &serde_json::Map<String, Value>) -> TokenStream {
    let unit = dimen.get("unit").unwrap();
    let value = || dimen.get("value").unwrap().as_f64().unwrap() as f32;

    match unit {
        Value::String(ref unit) => match unit.as_ref() {
            "max-content" => quote!(taffy::style::AvailableSpace::MaxContent),
            "min-content" => quote!(taffy::style::AvailableSpace::MaxContent),
            "px" => {
                let value = value();
                quote!(taffy::style::AvailableSpace::Definite(#value))
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_grid_auto_flow(auto_flow: &serde_json::Map<String, Value>) -> TokenStream {
    let direction = auto_flow.get("direction").unwrap().as_str().unwrap();
    let algorithm = auto_flow.get("algorithm").unwrap().as_str().unwrap();

    match (direction, algorithm) {
        ("row", "sparse") => quote!(taffy::style::GridAutoFlow::Row),
        ("column", "sparse") => quote!(taffy::style::GridAutoFlow::Column),
        ("row", "dense") => quote!(taffy::style::GridAutoFlow::RowDense),
        ("column", "dense") => quote!(taffy::style::GridAutoFlow::ColumnDense),
        _ => unreachable!(),
    }
}

fn generate_line(start: TokenStream, end: TokenStream) -> TokenStream {
    quote!(taffy::geometry::Line { start:#start, end:#end })
}

fn generate_grid_position(grid_position: &serde_json::Map<String, Value>) -> TokenStream {
    let kind = grid_position.get("kind").unwrap();
    let value = || grid_position.get("value").unwrap().as_f64().unwrap() as f32;

    match kind {
        Value::String(ref kind) => match kind.as_ref() {
            "auto" => quote!(taffy::style::GridPlacement::Auto),
            "span" => {
                let value = value() as u16;
                quote!(taffy::style::GridPlacement::Span(#value))
            }
            "line" => {
                let value = value() as i16;
                quote!(line(#value))
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_track_definition_list(raw_list: &[Value]) -> TokenStream {
    let list = raw_list.iter().map(|obj| match obj {
        Value::Object(inner) => generate_track_definition(inner),
        _ => unreachable!(),
    });

    quote!(vec![#(#list),*])
}

fn generate_track_definition(track_definition: &serde_json::Map<String, Value>) -> TokenStream {
    let kind = track_definition.get("kind").unwrap().as_str().unwrap();
    let name = || track_definition.get("name").unwrap().as_str().unwrap();
    let arguments = || track_definition.get("arguments").unwrap();

    match kind {
        "scalar" => generate_scalar_definition(track_definition),
        "function" => match (name(), arguments()) {
            ("fit-content", Value::Array(arguments)) => {
                if arguments.len() != 1 {
                    panic!("fit-content function with the wrong number of arguments");
                }
                let argument = match arguments[0] {
                    Value::Object(ref arg) => generate_scalar_definition(arg),
                    _ => unreachable!(),
                };
                quote!(fit_content(#argument))
            }
            ("minmax", Value::Array(arguments)) => {
                if arguments.len() != 2 {
                    panic!("minmax function with the wrong number of arguments");
                }
                let min = match arguments[0] {
                    Value::Object(ref arg) => generate_scalar_definition(arg),
                    _ => unreachable!(),
                };
                let max = match arguments[1] {
                    Value::Object(ref arg) => generate_scalar_definition(arg),
                    _ => unreachable!(),
                };
                quote!(minmax(#min, #max))
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
                            "auto-fill" => quote!(RepetitionCount::AutoFill),
                            "auto-fit" => quote!(RepetitionCount::AutoFit),
                            "integer" => {
                                let repetition_count = value();
                                quote!(RepetitionCount::Count(#repetition_count))
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                };
                let track_list = generate_track_definition_list(&arguments[1..]);
                quote!(repeat(#repetition, #track_list))
            }
            // TODO: Add support for fit-content
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_scalar_definition(track_definition: &serde_json::Map<String, Value>) -> TokenStream {
    let unit = || track_definition.get("unit").unwrap().as_str().unwrap();
    let value = || track_definition.get("value").unwrap().as_f64().unwrap() as f32;

    match unit() {
        "auto" => quote!(auto()),
        "min-content" => quote!(min_content()),
        "max-content" => quote!(max_content()),
        "px" | "percent" | "fraction" => {
            let value = value();
            match unit() {
                "px" => quote!(length(#value)),
                "percent" => quote!(percent(#value)),
                "fraction" => quote!(fr(#value)),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn generate_node_context(text_content: &str, writing_mode: Option<&str>, aspect_ratio: Option<f32>) -> TokenStream {
    let trimmed_text_content = text_content.trim();

    let writing_mode_token = match writing_mode {
        Some("vertical-rl" | "vertical-lr") => quote!(crate::WritingMode::Vertical),
        _ => quote!(crate::WritingMode::Horizontal),
    };

    let _aspect_ratio_token = match aspect_ratio {
        Some(ratio) => quote!(Some(#ratio)),
        None => quote!(None),
    };

    quote!(
        crate::TestNodeContext::ahem_text(#trimmed_text_content, #writing_mode_token)
    )
}
