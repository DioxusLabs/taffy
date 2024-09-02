use crate::generators::java::create_java_enum;
use crate::transformers::java::create_java_tranformer;
use std::fs;

mod generators;
mod transformers;

struct RustEnum<'local> {
    name: &'local str,
    values: Vec<&'local str>,
    default: bool,
}

fn main() {
    fs::remove_file("./bindings/java/src/enums.rs").expect("Error: Unable to remove java/src/enums.rs file");

    let mut enums: Vec<RustEnum> = vec![
        RustEnum { name: "Display", values: vec!["Block", "Flex", "Grid", "None"], default: true },
        RustEnum { name: "BoxGenerationMode", values: vec!["Normal", "None"], default: true },
        RustEnum { name: "Position", values: vec!["Relative", "Absolute"], default: true },
        RustEnum { name: "BoxSizing", values: vec!["BorderBox", "ContentBox"], default: true },
        RustEnum { name: "Overflow", values: vec!["Visible", "Clip", "Hidden", "Scroll"], default: true },
        RustEnum {
            name: "TextAlign",
            values: vec!["Auto", "LegacyLeft", "LegacyRight", "LegacyCenter"],
            default: true,
        },
        RustEnum { name: "GridAutoFlow", values: vec!["Row", "Column", "RowDense", "ColumnDense"], default: true },
        RustEnum { name: "FlexWrap", values: vec!["NoWrap", "Wrap", "WrapReverse"], default: true },
        RustEnum { name: "FlexDirection", values: vec!["Row", "Column", "RowReverse", "ColumnReverse"], default: true },
        RustEnum {
            name: "AlignContent",
            values: vec![
                "Start",
                "End",
                "FlexStart",
                "FlexEnd",
                "Center",
                "Stretch",
                "SpaceBetween",
                "SpaceEvenly",
                "SpaceAround",
            ],
            default: false,
        },
        RustEnum {
            name: "AlignItems",
            values: vec!["Start", "End", "FlexStart", "FlexEnd", "Center", "Baseline", "Stretch"],
            default: false,
        },
        RustEnum { name: "AbsoluteAxis", values: vec!["Horizontal", "Vertical"], default: false },
    ];

    // Might look inverted at first, but it is needed for rustfmt to not throw formatting errors
    // (also known as ensuring alphabetical order in imports) 
    enums.sort_by(|a, b| b.name.cmp(a.name));

    for value in enums.into_iter() {
        create_enum(value.name, &value.values);
        create_transformer(value.name, &value.values, value.default);
    }
}

/// Enum generators

fn create_enum(name: &str, values: &[&str]) {
    create_java_enum(name, values);
}

/// Transformer generators

fn create_transformer(name: &str, values: &[&str], default: bool) {
    create_java_tranformer(name, values, default);
}
