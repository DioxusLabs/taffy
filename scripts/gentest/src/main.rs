use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use convert_case::{Case, Casing};
use fantoccini::{Client, ClientBuilder};
use log::*;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use serde_json::Value;
use syn::Ident;

#[tokio::main]
async fn main() {
    env_logger::init();
    // this requires being run by cargo, which is iffy
    let root_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = root_dir.parent().and_then(Path::parent).unwrap();

    let fixtures_root = repo_root.join("test_fixtures");
    let fixtures = fs::read_dir(fixtures_root).unwrap();

    info!("reading test fixtures from disk");
    let mut fixtures: Vec<_> = fixtures
        .into_iter()
        .filter_map(|a| a.ok())
        .filter(|f| !f.file_name().to_string_lossy().starts_with('x')) // ignore tests beginning with x
        .filter(|f| f.path().is_file() && f.path().extension().map(|p| p == "html").unwrap_or(false))
        .map(|f| {
            let fixture_path = f.path();
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

    let mut test_descs = vec![];
    for (name, fixture_path) in fixtures {
        test_descs.push(test_root_element(client.clone(), name, fixture_path).await);
    }

    info!("killing webdriver instance...");
    webdriver_handle.kill().unwrap();

    info!("generating test sources and concatenating...");

    let bench_descs: Vec<_> = test_descs
        .iter()
        .map(|(name, description)| {
            debug!("generating bench contents for {}", &name);
            (name.clone(), generate_bench(description))
        })
        .collect();

    let test_descs: Vec<_> = test_descs
        .iter()
        .map(|(name, description)| {
            debug!("generating test contents for {}", &name);
            (name.clone(), generate_test(name, description))
        })
        .collect();

    let benchmarks: Vec<_> = test_descs
        .iter()
        .map(|(name, _)| {
            let bench_mod = Ident::new(name, Span::call_site());
            quote!(#bench_mod::compute())
        })
        .collect();

    let mod_statemnts = test_descs
        .iter()
        .map(|(name, _)| {
            let name = Ident::new(name, Span::call_site());
            quote!(mod #name;)
        })
        .fold(quote!(), |a, b| quote!(#a #b));
    let generic_measure_function = generate_generic_measure_function();

    let test_mod_file = quote!(
        #generic_measure_function
        #mod_statemnts
    );

    let bench_mod_file = quote!(
        use criterion::{criterion_group, criterion_main, Criterion};

        #generic_measure_function

        #mod_statemnts

        fn benchmark(c: &mut Criterion) {
            c.bench_function("generated benchmarks", |b| {
                b.iter(|| { #(#benchmarks;)* })
            });
        }

        criterion_group!(benches, benchmark);
        criterion_main!(benches);
    );

    info!("writing generated benchmarks file to disk...");
    let benches_base_path = repo_root.join("benches").join("generated");
    fs::remove_dir_all(&benches_base_path).unwrap();
    fs::create_dir(&benches_base_path).unwrap();
    for (name, bench_body) in bench_descs {
        let mut bench_filename = benches_base_path.join(&name);
        bench_filename.set_extension("rs");
        debug!("writing {} to disk...", &name);
        fs::write(bench_filename, bench_body.to_string()).unwrap();
    }
    fs::write(benches_base_path.join("mod.rs"), bench_mod_file.to_string()).unwrap();

    info!("writing generated test file to disk...");
    let tests_base_path = repo_root.join("tests").join("generated");
    fs::remove_dir_all(&tests_base_path).unwrap();
    fs::create_dir(&tests_base_path).unwrap();
    for (name, test_body) in test_descs {
        let mut test_filename = tests_base_path.join(&name);
        test_filename.set_extension("rs");
        debug!("writing {} to disk...", &name);
        fs::write(test_filename, test_body.to_string()).unwrap();
    }
    fs::write(tests_base_path.join("mod.rs"), test_mod_file.to_string()).unwrap();

    info!("formatting the source directory");
    Command::new("cargo").arg("fmt").current_dir(repo_root).status().unwrap();
}

async fn test_root_element(client: Client, name: String, fixture_path: impl AsRef<Path>) -> (String, Value) {
    let fixture_path = fixture_path.as_ref();

    let url = format!("file://{}", fixture_path.display());

    client.goto(&url).await.unwrap();
    let description = client
        .execute("return JSON.stringify(describeElement(document.getElementById('test-root')))", vec![])
        .await
        .unwrap();
    let description_string = description.as_str().unwrap();
    let description = serde_json::from_str(description_string).unwrap();
    (name, description)
}

fn generate_bench(description: &Value) -> TokenStream {
    let node_description = generate_node("node", description);

    quote!(
        pub fn compute() {
            #[allow(unused_imports)]
            use taffy::prelude::*;
            let mut taffy = taffy::Taffy::new();
            #node_description
            taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
        }
    )
}

fn generate_test(name: impl AsRef<str>, description: &Value) -> TokenStream {
    let name = name.as_ref();
    let name = Ident::new(name, Span::call_site());
    let node_description = generate_node("node", description);
    let assertions = generate_assertions("node", description);

    quote!(
        #[test]
        fn #name() {
            #[allow(unused_imports)]
            use taffy::prelude::*;
            let mut taffy = taffy::Taffy::new();
            #node_description
            taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();

            println!("\nComputed tree:");
            taffy::debug::print_tree(&taffy, node);
            println!();

            #assertions
        }
    )
}

fn generate_assertions(ident: &str, node: &Value) -> TokenStream {
    let layout = &node["layout"];

    let read_f32 = |s: &str| layout[s].as_f64().unwrap() as f32;
    let width = read_f32("width");
    let height = read_f32("height");
    let x = read_f32("x");
    let y = read_f32("y");

    let children = {
        let mut c = Vec::new();
        if let Value::Array(ref value) = node["children"] {
            for (idx, child) in value.iter().enumerate() {
                c.push(generate_assertions(&format!("{ident}{idx}"), child));
            }
        };
        c.into_iter().fold(quote!(), |a, b| quote!(#a #b))
    };

    let ident = Ident::new(ident, Span::call_site());

    quote!(
        assert_eq!(taffy.layout(#ident).unwrap().size.width, #width);
        assert_eq!(taffy.layout(#ident).unwrap().size.height, #height);
        assert_eq!(taffy.layout(#ident).unwrap().location.x, #x);
        assert_eq!(taffy.layout(#ident).unwrap().location.y, #y);

        #children
    )
}

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
            "grid" => quote!(display: taffy::style::Display::Grid,),
            _ => quote!(display: taffy::style::Display::Flex,),
        },
        _ => quote!(),
    };

    let position_type = match style["positionType"] {
        Value::String(ref value) => match value.as_ref() {
            "absolute" => quote!(position_type: taffy::style::PositionType::Absolute,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let direction = match style["direction"] {
        Value::String(ref value) => match value.as_ref() {
            "rtl" => quote!(direction: taffy::style::Direction::RTL,),
            "ltr" => quote!(direction: taffy::style::Direction::LTR,),
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

    let overflow = match style["overflow"] {
        Value::String(ref value) => match value.as_ref() {
            "hidden" => quote!(overflow: taffy::style::Overflow::Hidden,),
            "scroll" => quote!(overflow: taffy::style::Overflow::Scroll,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_items = match style["alignItems"] {
        Value::String(ref value) => match value.as_ref() {
            "flex-start" | "start" => quote!(align_items: Some(taffy::style::AlignItems::Start),),
            "flex-end" | "end" => quote!(align_items: Some(taffy::style::AlignItems::End),),
            "center" => quote!(align_items: Some(taffy::style::AlignItems::Center),),
            "baseline" => quote!(align_items: Some(taffy::style::AlignItems::Baseline),),
            "stretch" => quote!(align_items: Some(taffy::style::AlignItems::Stretch),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_self = match style["alignSelf"] {
        Value::String(ref value) => match value.as_ref() {
            "flex-start" | "start" => quote!(align_self: Some(taffy::style::AlignSelf::Start),),
            "flex-end" | "end" => quote!(align_self: Some(taffy::style::AlignSelf::End),),
            "center" => quote!(align_self: Some(taffy::style::AlignSelf::Center),),
            "baseline" => quote!(align_self: Some(taffy::style::AlignSelf::Baseline),),
            "stretch" => quote!(align_self: Some(taffy::style::AlignSelf::Stretch),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let justify_items = match style["justifyItems"] {
        Value::String(ref value) => match value.as_ref() {
            "flex-start" | "start" => quote!(justify_items: Some(taffy::style::JustifyItems::Start),),
            "flex-end" | "end" => quote!(justify_items: Some(taffy::style::JustifyItems::End),),
            "center" => quote!(justify_items: Some(taffy::style::JustifyItems::Center),),
            "baseline" => quote!(justify_items: Some(taffy::style::JustifyItems::Baseline),),
            "stretch" => quote!(justify_items: Some(taffy::style::JustifyItems::Stretch),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let justify_self = match style["justifySelf"] {
        Value::String(ref value) => match value.as_ref() {
            "flex-start" | "start" => quote!(justify_self: Some(taffy::style::JustifySelf::Start),),
            "flex-end" | "end" => quote!(justify_self: Some(taffy::style::JustifySelf::End),),
            "center" => quote!(justify_self: Some(taffy::style::JustifySelf::Center),),
            "baseline" => quote!(justify_self: Some(taffy::style::JustifySelf::Baseline),),
            "stretch" => quote!(justify_self: Some(taffy::style::JustifySelf::Stretch),),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_content = match style["alignContent"] {
        Value::String(ref value) => match value.as_ref() {
            "flex-start" | "start" => quote!(align_content: Some(taffy::style::AlignContent::Start),),
            "flex-end" | "end" => quote!(align_content: Some(taffy::style::AlignContent::End),),
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
            "flex-start" | "start" => quote!(justify_content: Some(taffy::style::JustifyContent::Start),),
            "flex-end" | "end" => quote!(justify_content: Some(taffy::style::JustifyContent::End),),
            "center" => quote!(justify_content: Some(taffy::style::JustifyContent::Center),),
            "stretch" => quote!(align_content: Some(taffy::style::AlignContent::Stretch),),
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

    let measure_func: Option<_> = get_string_value("text_content", node).map(generate_measure_function);

    edges_quoted!(style, margin, generate_length_percentage_auto, quote!(zero()));
    edges_quoted!(style, padding, generate_length_percentage, quote!(zero()));
    edges_quoted!(style, border, generate_length_percentage, quote!(zero()));
    edges_quoted!(style, position, generate_length_percentage_auto, quote!(auto()));

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
        #direction
        #position_type
        #flex_direction
        #flex_wrap
        #overflow
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
        #margin
        #padding
        #position
        #border
        ..Default::default()
    });

    if has_children {
        quote!(
            #children_body
            let #ident = taffy.new_with_children(#style,#children).unwrap();
        )
    } else if measure_func.is_some() {
        quote!(let #ident = taffy.new_leaf_with_measure(#style,#measure_func,).unwrap();)
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
            "points" => {
                let value = value();
                quote!(taffy::style::LengthPercentage::Points(#value))
            }
            "percent" => {
                let value = value();
                quote!(taffy::style::LengthPercentage::Percent(#value))
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
            "auto" => quote!(taffy::style::LengthPercentageAuto::Auto),
            "points" => {
                let value = value();
                quote!(taffy::style::LengthPercentageAuto::Points(#value))
            }
            "percent" => {
                let value = value();
                quote!(taffy::style::LengthPercentageAuto::Percent(#value))
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
            "auto" => quote!(taffy::style::Dimension::Auto),
            "points" => {
                let value = value();
                quote!(taffy::style::Dimension::Points(#value))
            }
            "percent" => {
                let value = value();
                quote!(taffy::style::Dimension::Percent(#value))
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
            "track" => {
                let value = value() as i16;
                quote!(taffy::style::GridPlacement::Track(#value))
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
        "scalar" => generate_scalar_definition(track_definition, quote!(TrackSizingFunction)),
        "function" => match (name(), arguments()) {
            ("minmax", Value::Array(arguments)) => {
                if arguments.len() != 2 {
                    panic!("minmax function with the wrong number of arguments");
                }
                let min = match arguments[0] {
                    Value::Object(ref arg) => generate_scalar_definition(arg, quote!(MinTrackSizingFunction)),
                    _ => unreachable!(),
                };
                let max = match arguments[1] {
                    Value::Object(ref arg) => generate_scalar_definition(arg, quote!(MaxTrackSizingFunction)),
                    _ => unreachable!(),
                };
                quote!(taffy::style::TrackSizingFunction::MinMax{ min: #min, max: #max })
            }
            // TODO: Add support for fit-content
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_scalar_definition(
    track_definition: &serde_json::Map<String, Value>,
    sizing_function: TokenStream,
) -> TokenStream {
    let unit = || track_definition.get("unit").unwrap().as_str().unwrap();
    let value = || track_definition.get("value").unwrap().as_f64().unwrap() as f32;

    match unit() {
        "auto" => quote!(taffy::style::#sizing_function::Auto),
        "min-content" => quote!(taffy::style::#sizing_function::MinContent),
        "max-content" => quote!(taffy::style::#sizing_function::MaxContent),
        "points" | "percent" => {
            let value = generate_dimension(track_definition);
            quote!(taffy::style::#sizing_function::Fixed(#value))
        }
        "fraction" => {
            let value: f32 = value();
            quote!(taffy::style::#sizing_function::Flex(#value))
        }
        _ => unreachable!(),
    }
}

fn generate_generic_measure_function() -> TokenStream {
    quote!(
        // WARNING: This function is generated by the gentest script. Do not edit directly
        fn measure_standard_text(
            known_dimensions: taffy::geometry::Size<Option<f32>>,
            available_space: taffy::geometry::Size<taffy::layout::AvailableSpace>,
            text_content: &str,
        ) -> taffy::geometry::Size<f32> {
            use taffy::prelude::*;
            const ZWS: char = '\u{200B}';
            const H_WIDTH: f32 = 10.0;
            const H_HEIGHT: f32 = 10.0;

            if let Size { width: Some(width), height: Some(height) } = known_dimensions {
                return Size { width, height };
            }

            let lines: Vec<&str> = text_content.split(ZWS).collect();
            if lines.is_empty() {
                return Size::ZERO;
            }

            let min_line_length: usize = lines.iter().map(|line| line.len()).max().unwrap_or(0);
            let max_line_length: usize = lines.iter().map(|line| line.len()).sum();
            let width = known_dimensions.width.unwrap_or_else(|| match available_space.width {
                AvailableSpace::MinContent => min_line_length as f32 * H_WIDTH,
                AvailableSpace::MaxContent => max_line_length as f32 * H_WIDTH,
                AvailableSpace::Definite(width) => {
                    width.min(max_line_length as f32 * H_WIDTH).max(min_line_length as f32 * H_WIDTH)
                }
            });
            let height = known_dimensions.height.unwrap_or_else(|| {
                let width_line_length = (width / H_WIDTH).floor() as usize;
                let mut line_count = 1;
                let mut current_line_length = 0;
                for line in &lines {
                    if current_line_length + line.len() > width_line_length {
                        line_count += 1;
                        current_line_length = line.len();
                    } else {
                        current_line_length += line.len();
                    };
                }
                (line_count as f32) * H_HEIGHT
            });

            Size { width, height }
        }
    )
}

fn generate_measure_function(text_content: &str) -> TokenStream {
    quote!(
        taffy::node::MeasureFunc::Raw(|known_dimensions, available_space| {
            const TEXT : &str = #text_content;
            super::measure_standard_text(known_dimensions, available_space, TEXT)
        })
    )
}
