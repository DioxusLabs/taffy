use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use convert_case::{Case, Casing};
use fantoccini::{Client, ClientBuilder};

use log::*;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
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

    let test_mods = test_descs
        .iter()
        .map(|(name, _)| {
            let name = Ident::new(name, Span::call_site());
            quote!(mod #name;)
        })
        .fold(quote!(), |a, b| quote!(#a #b));

    for (name, bench_body) in bench_descs {
        let mut bench_filename = repo_root.join("benches").join("generated").join(&name);
        bench_filename.set_extension("rs");
        debug!("writing {} to disk...", &name);
        fs::write(bench_filename, bench_body.to_string()).unwrap();
    }

    for (name, test_body) in test_descs {
        let mut test_filename = repo_root.join("tests").join("generated").join(&name);
        test_filename.set_extension("rs");
        debug!("writing {} to disk...", &name);
        fs::write(test_filename, test_body.to_string()).unwrap();
    }

    let bench_mods = quote!(
        use criterion::{criterion_group, criterion_main, Criterion};

        #test_mods

        fn benchmark(c: &mut Criterion) {
            c.bench_function("generated benchmarks", |b| {
                b.iter(|| { #(#benchmarks;)* })
            });
        }

        criterion_group!(benches, benchmark);
        criterion_main!(benches);
    );

    info!("writing generated test file to disk...");
    fs::write(repo_root.join("benches").join("generated").join("mod.rs"), bench_mods.to_string()).unwrap();
    fs::write(repo_root.join("tests").join("generated").join("mod.rs"), test_mods.to_string()).unwrap();

    info!("formatting the source directory");
    Command::new("cargo").arg("fmt").current_dir(repo_root).status().unwrap();
}

async fn test_root_element(client: Client, name: String, fixture_path: impl AsRef<Path>) -> (String, json::JsonValue) {
    let fixture_path = fixture_path.as_ref();

    let url = format!("file://{}", fixture_path.display());

    client.goto(&url).await.unwrap();
    let description = client
        .execute("return JSON.stringify(describeElement(document.getElementById('test-root')))", vec![])
        .await
        .unwrap();
    let description_string = description.as_str().unwrap();
    let description = json::parse(description_string).unwrap();
    (name, description)
}

fn generate_bench(description: &json::JsonValue) -> TokenStream {
    let node_description = generate_node("node", description);

    quote!(
        pub fn compute() {
            let mut taffy = taffy::Taffy::new();
            #node_description
            taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
        }
    )
}

fn generate_test(name: impl AsRef<str>, description: &json::JsonValue) -> TokenStream {
    let name = name.as_ref();
    let name = Ident::new(name, Span::call_site());
    let node_description = generate_node("node", description);
    let assertions = generate_assertions("node", description);

    quote!(
        #[test]
        fn #name() {
            let mut taffy = taffy::Taffy::new();
            #node_description
            taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
            #assertions
        }
    )
}

fn generate_assertions(ident: &str, node: &json::JsonValue) -> TokenStream {
    let layout = &node["layout"];

    let read_f32 = |s: &str| layout[s].as_f32().unwrap();
    let width = read_f32("width");
    let height = read_f32("height");
    let x = read_f32("x");
    let y = read_f32("y");

    let children = {
        let mut c = Vec::new();
        if let json::JsonValue::Array(ref value) = node["children"] {
            for (idx, child) in value.iter().enumerate() {
                c.push(generate_assertions(&format!("{}{}", ident, idx), child));
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

// type QuoteProp = Fn(prop_name: &str, quoter: impl Fn(&json::object::Object) -> TokenStream) -> TokenStream;

fn generate_node(ident: &str, node: &json::JsonValue) -> TokenStream {
    let style = &node["style"];

    fn quote_object_prop(
        prop_name: &str,
        style: &json::JsonValue,
        quoter: impl Fn(&json::object::Object) -> TokenStream,
    ) -> TokenStream {
        let prop_name_snake_case = prop_name.to_case(Case::Snake);
        let prop_name_camel_case = prop_name.to_case(Case::Camel);
        let prop_name_ident = format_ident!("{}", prop_name_snake_case);
        match style[prop_name_camel_case.clone()] {
            json::JsonValue::Object(ref value) => {
                let prop_value = quoter(value);
                quote!(#prop_name_ident: #prop_value,)
            }
            _ => quote!(),
        }
    }

    fn quote_array_prop(
        prop_name: &str,
        style: &json::JsonValue,
        quoter: impl Fn(&Vec<json::JsonValue>) -> TokenStream,
    ) -> TokenStream {
        let prop_name_snake_case = prop_name.to_case(Case::Snake);
        let prop_name_camel_case = prop_name.to_case(Case::Camel);
        let prop_name_ident = format_ident!("{}", prop_name_snake_case);
        match style[prop_name_camel_case.clone()] {
            json::JsonValue::Array(ref value) => {
                let prop_value = quoter(value);
                quote!(#prop_name_ident: #prop_value,)
            }
            _ => quote!(),
        }
    }

    fn quote_string_prop(
        prop_name: &str,
        style: &json::JsonValue,
        quoter: impl Fn(&str) -> TokenStream,
    ) -> TokenStream {
        let prop_name_snake_case = prop_name.to_case(Case::Snake);
        let prop_name_camel_case = prop_name.to_case(Case::Camel);
        let prop_name_ident = format_ident!("{}", prop_name_snake_case);
        match style[prop_name_camel_case.clone()] {
            json::JsonValue::Short(ref value) => {
                let prop_value = quoter(value.as_ref());
                quote!(#prop_name_ident: #prop_value,)
            }
            json::JsonValue::String(ref value) => {
                let prop_value = quoter(value.as_ref());
                quote!(#prop_name_ident: #prop_value,)
            }
            _ => quote!(),
        }
    }

    fn quote_number_prop<T: From<json::number::Number>>(
        prop_name: &str,
        style: &json::JsonValue,
        quoter: impl Fn(T) -> TokenStream,
    ) -> TokenStream {
        let prop_name_snake_case = prop_name.to_case(Case::Snake);
        let prop_name_camel_case = prop_name.to_case(Case::Camel);
        let prop_name_ident = format_ident!("{}", prop_name_snake_case);
        match style[prop_name_camel_case.clone()] {
            json::JsonValue::Number(ref value) => {
                let prop_value = quoter(value.clone().into());
                quote!(#prop_name_ident: #prop_value,)
            }
            _ => quote!(),
        }
    }

    let display = match style["display"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "none" => quote!(display: taffy::style::Display::None,),
            "grid" => quote!(display: taffy::style::Display::Grid,),
            _ => quote!(display: taffy::style::Display::Flex,),
        },
        _ => quote!(),
    };

    let position_type = match style["positionType"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "absolute" => quote!(position_type: taffy::style::PositionType::Absolute,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let direction = match style["direction"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "rtl" => quote!(direction: taffy::style::Direction::RTL,),
            "ltr" => quote!(direction: taffy::style::Direction::LTR,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let flex_direction = match style["flexDirection"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "row-reverse" => quote!(flex_direction: taffy::style::FlexDirection::RowReverse,),
            "column" => quote!(flex_direction: taffy::style::FlexDirection::Column,),
            "column-reverse" => quote!(flex_direction: taffy::style::FlexDirection::ColumnReverse,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let flex_wrap = match style["flexWrap"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "wrap" => quote!(flex_wrap: taffy::style::FlexWrap::Wrap,),
            "wrap-reverse" => quote!(flex_wrap: taffy::style::FlexWrap::WrapReverse,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let overflow = match style["overflow"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "hidden" => quote!(overflow: taffy::style::Overflow::Hidden,),
            "scroll" => quote!(overflow: taffy::style::Overflow::Scroll,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_items = match style["alignItems"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "flex-start" => quote!(align_items: taffy::style::AlignItems::FlexStart,),
            "flex-end" => quote!(align_items: taffy::style::AlignItems::FlexEnd,),
            "center" => quote!(align_items: taffy::style::AlignItems::Center,),
            "baseline" => quote!(align_items: taffy::style::AlignItems::Baseline,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_self = match style["alignSelf"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "flex-start" => quote!(align_self: taffy::style::AlignSelf::FlexStart,),
            "flex-end" => quote!(align_self: taffy::style::AlignSelf::FlexEnd,),
            "center" => quote!(align_self: taffy::style::AlignSelf::Center,),
            "baseline" => quote!(align_self: taffy::style::AlignSelf::Baseline,),
            "stretch" => quote!(align_self: taffy::style::AlignSelf::Stretch,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let align_content = match style["alignContent"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "flex-start" => quote!(align_content: taffy::style::AlignContent::FlexStart,),
            "flex-end" => quote!(align_content: taffy::style::AlignContent::FlexEnd,),
            "center" => quote!(align_content: taffy::style::AlignContent::Center,),
            "space-between" => quote!(align_content: taffy::style::AlignContent::SpaceBetween,),
            "space-around" => quote!(align_content: taffy::style::AlignContent::SpaceAround,),
            "space-evenly" => quote!(align_content: taffy::style::AlignContent::SpaceEvenly,),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let justify_content = match style["justifyContent"] {
        json::JsonValue::Short(ref value) => match value.as_ref() {
            "flex-end" => quote!(justify_content: taffy::style::JustifyContent::FlexEnd,),
            "center" => quote!(justify_content: taffy::style::JustifyContent::Center,),
            "space-between" => quote!(justify_content: taffy::style::JustifyContent::SpaceBetween,),
            "space-around" => quote!(justify_content: taffy::style::JustifyContent::SpaceAround,),
            "space-evenly" => quote!(justify_content: taffy::style::JustifyContent::SpaceEvenly,),
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

    let grid_row_start = quote_object_prop("grid_row_start", style, generate_grid_position);
    let grid_row_end = quote_object_prop("grid_row_end", style, generate_grid_position);
    let grid_column_start = quote_object_prop("grid_column_start", style, generate_grid_position);
    let grid_column_end = quote_object_prop("grid_column_end", style, generate_grid_position);

    macro_rules! edges_quoted {
        ($style:ident, $val:ident) => {
            let $val = match $style[stringify!($val)] {
                json::JsonValue::Object(ref value) => {
                    let edges = generate_edges(value);
                    quote!($val: #edges,)
                },
                _ => quote!(),
            };
        };
    }

    edges_quoted!(style, margin);
    edges_quoted!(style, padding);
    edges_quoted!(style, position);
    edges_quoted!(style, border);

    let (children_body, children) = match node["children"] {
        json::JsonValue::Array(ref value) => {
            if !value.is_empty() {
                let body = value
                    .iter()
                    .enumerate()
                    .map(|(i, child)| generate_node(&format!("{}{}", ident, i), child))
                    .collect();
                let idents = value
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Ident::new(&format!("{}{}", ident, i), Span::call_site()))
                    .collect::<Vec<_>>();
                (body, quote!(&[#(#idents),*]))
            } else {
                (quote!(), quote!(&[]))
            }
        }
        _ => (quote!(), quote!()),
    };

    let ident = Ident::new(ident, Span::call_site());

    quote!(
        #children_body
        let #ident = taffy.new_with_children(
        taffy::style::FlexboxLayout {
            #display
            #direction
            #position_type
            #flex_direction
            #flex_wrap
            #overflow
            #align_items
            #align_self
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
            #grid_row_start
            #grid_row_end
            #grid_column_start
            #grid_column_end
            #size
            #min_size
            #max_size
            #margin
            #padding
            #position
            #border
            ..Default::default()
        },
        #children
        // TODO: Only add children if they exist
    ).unwrap();)
}

macro_rules! dim_quoted_renamed {
    ($obj:ident, $in_name:ident, $out_name:ident) => {
        let $out_name = match $obj.get(stringify!($in_name)) {
            Some(json::JsonValue::Object(ref value)) => {
                let dim = generate_dimension(value);
                quote!($out_name: #dim,)
            }
            _ => quote!(),
        };
    };
}

macro_rules! dim_quoted {
    ($obj:ident, $dim_name:ident) => {
        dim_quoted_renamed!($obj, $dim_name, $dim_name)
    };
}

fn generate_size(size: &json::object::Object) -> TokenStream {
    dim_quoted!(size, width);
    dim_quoted!(size, height);
    quote!(
        taffy::geometry::Size {
            #width #height
            ..Default::default()
        }
    )
}

fn generate_gap(size: &json::object::Object) -> TokenStream {
    dim_quoted_renamed!(size, column, width);
    dim_quoted_renamed!(size, row, height);
    quote!(
        taffy::geometry::Size {
            #width #height
            ..Default::default()
        }
    )
}

fn generate_dimension(dimen: &json::object::Object) -> TokenStream {
    let unit = dimen.get("unit").unwrap();
    let value = || dimen.get("value").unwrap().as_f32().unwrap();

    match unit {
        json::JsonValue::Short(ref unit) => match unit.as_ref() {
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

fn generate_edges(dimen: &json::object::Object) -> TokenStream {
    dim_quoted!(dimen, start);
    dim_quoted!(dimen, end);
    dim_quoted!(dimen, top);
    dim_quoted!(dimen, bottom);

    quote!(taffy::geometry::Rect {
        #start #end #top #bottom
        ..Default::default()
    })
}

fn generate_grid_auto_flow(auto_flow: &json::object::Object) -> TokenStream {
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

fn generate_grid_position(grid_position: &json::object::Object) -> TokenStream {
    let kind = grid_position.get("kind").unwrap();
    let value = || grid_position.get("value").unwrap().as_f32().unwrap();

    match kind {
        json::JsonValue::Short(ref kind) => match kind.as_ref() {
            "auto" => quote!(taffy::style::GridLine::Auto),
            "span" => {
                let value = value() as u16;
                quote!(taffy::style::GridLine::Span(#value))
            }
            "track" => {
                let value = value() as i16;
                quote!(taffy::style::GridLine::Track(#value))
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_track_definition_list(raw_list: &Vec<json::JsonValue>) -> TokenStream {
    let list = raw_list.into_iter().map(|obj| match obj {
        json::JsonValue::Object(inner) => generate_track_definition(inner),
        _ => unreachable!(),
    });

    quote!(vec![#(#list),*])
}

fn generate_track_definition(track_definition: &json::object::Object) -> TokenStream {
    let kind = track_definition.get("kind").unwrap().as_str().unwrap();
    let name = || track_definition.get("name").unwrap().as_str().unwrap();
    let arguments = || track_definition.get("arguments").unwrap();

    match kind {
        "scalar" => generate_scalar_definition(track_definition),
        "function" => match (name(), arguments()) {
            ("minmax", json::JsonValue::Array(arguments)) => {
                if arguments.len() != 2 {
                    panic!("minmax function with the wrong number of arguments");
                }
                let min = match arguments[0] {
                    json::JsonValue::Object(ref arg) => generate_scalar_definition(arg),
                    _ => unreachable!(),
                };
                let max = match arguments[1] {
                    json::JsonValue::Object(ref arg) => generate_scalar_definition(arg),
                    _ => unreachable!(),
                };
                quote!(taffy::style::GridTrackSizingFunction::MinMax{ min: #min, max: #max })
            }
            // TODO: Add support for fit-content
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn generate_scalar_definition(track_definition: &json::object::Object) -> TokenStream {
    let unit = || track_definition.get("unit").unwrap().as_str().unwrap();
    let value = || track_definition.get("value").unwrap().as_f32().unwrap();

    match unit() {
        "auto" => quote!(taffy::style::GridTrackSizingFunction::Auto),
        "min-content" => quote!(taffy::style::GridTrackSizingFunction::MinContent),
        "max-content" => quote!(taffy::style::GridTrackSizingFunction::MaxContent),
        "points" | "percent" => {
            let value = generate_dimension(track_definition);
            quote!(taffy::style::GridTrackSizingFunction::Fixed(#value))
        }
        "fraction" => {
            let value: f32 = value();
            quote!(taffy::style::GridTrackSizingFunction::Flex(#value))
        }
        _ => unreachable!(),
    }
}
