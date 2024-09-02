use std::collections::HashMap;
use std::fs;
use crate::generators::java::create_java_enum;
use crate::transformers::java::create_java_tranformer;

mod generators;
mod transformers;

fn main() {
    fs::remove_file("./bindings/java/src/enums.rs").expect("Error: Unable to remove java/src/enums.rs file");

    let mut enums: HashMap<&str, Vec<&str>> = HashMap::new();
    enums.insert("Display", vec!["Block", "Flex", "Grid", "None"]);
    enums.insert("BoxGenerationMode", vec!["Normal", "None"]);
    enums.insert("Position", vec!["Relative", "Absolute"]);
    enums.insert("BoxSizing", vec!["BorderBox", "ContentBox"]);
    enums.insert("Overflow", vec!["Visible", "Clip", "Hidden", "Scroll"]);
    enums.insert("TextAlign", vec!["Auto", "LegacyLeft", "LegacyRight", "LegacyCenter"]);
    enums.insert("GridAutoFlow", vec!["Row", "Column", "RowDense", "ColumnDense"]);
    enums.insert("FlexWrap", vec!["NoWrap", "Wrap", "WrapReverse"]);
    enums.insert("FlexDirection", vec!["Row", "Column", "RowReverse", "ColumnReverse"]);
    enums.insert(
        "AlignContent",
        vec!["Start", "End", "FlexStart", "FlexEnd", "Center", "Stretch", "SpaceBetween", "SpaceEvenly", "SpaceAround"],
    );
    enums.insert("AlignItems", vec!["Start", "End", "FlexStart", "FlexEnd", "Center", "Baseline", "Stretch"]);
    enums.insert("AbsoluteAxis", vec!["Horizontal", "Vertical"]);

    for (key, value) in enums.into_iter() {
        create_enum(key, &value);
        create_transformer(key, &value);
    }
}

/// Enum generators

fn create_enum(name: &str, values: &[&str]) {
    create_java_enum(name, values);
}

/// Transformer generators

fn create_transformer(name: &str, values: &[&str]) {
    create_java_tranformer(name, values);
}


