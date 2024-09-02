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

    let mut enums: Vec<RustEnum> = Vec::new();
    enums.push(RustEnum { name: "Display", values: vec!["Block", "Flex", "Grid", "None"], default: true });
    enums.push(RustEnum { name: "BoxGenerationMode", values: vec!["Normal", "None"], default: true });
    enums.push(RustEnum { name: "Position", values: vec!["Relative", "Absolute"], default: true });
    enums.push(RustEnum { name: "BoxSizing", values: vec!["BorderBox", "ContentBox"], default: true });
    enums.push(RustEnum { name: "Overflow", values: vec!["Visible", "Clip", "Hidden", "Scroll"], default: true });
    enums.push(RustEnum {
        name: "TextAlign",
        values: vec!["Auto", "LegacyLeft", "LegacyRight", "LegacyCenter"],
        default: true,
    });
    enums.push(RustEnum {
        name: "GridAutoFlow",
        values: vec!["Row", "Column", "RowDense", "ColumnDense"],
        default: true,
    });
    enums.push(RustEnum { name: "FlexWrap", values: vec!["NoWrap", "Wrap", "WrapReverse"], default: true });
    enums.push(RustEnum {
        name: "FlexDirection",
        values: vec!["Row", "Column", "RowReverse", "ColumnReverse"],
        default: true,
    });
    enums.push(RustEnum {
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
    });
    enums.push(RustEnum {
        name: "AlignItems",
        values: vec!["Start", "End", "FlexStart", "FlexEnd", "Center", "Baseline", "Stretch"],
        default: false,
    });
    enums.push(RustEnum { name: "AbsoluteAxis", values: vec!["Horizontal", "Vertical"], default: false });

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
