use std::collections::HashMap;

fn main() {
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

    for (key, value) in enums.into_iter() {
        create_enum(key, value);
    }
}

fn create_enum(name: &str, values: Vec<&str>) {
    println!("{}", create_java_enum(name, values));
}

fn create_java_enum(name: &str, values: Vec<&str>) -> String {
    use convert_case::{Case, Casing};

    let package = "java.com.dioxuslabs.taffy.enums";
    let enum_name = name.to_case(Case::Pascal);

    let mut result = format!("package {};\n\npublic enum {} {{\n", package, enum_name);

    for (index, value) in values.iter().enumerate() {
        let java_value = value.to_case(Case::UpperSnake);
        result.push_str("    ");
        result.push_str(&java_value);
        result.push('(');
        result.push_str(&index.to_string());
        result.push_str("),\n");
    }

    // eliminate the last comma
    if !values.is_empty() {
        result.pop();
        result.pop();
        result.push('\n');
    }

    result.push('\n');
    result.push_str("    ;\n");
    result.push('\n');

    result.push_str("    private final int ordinal;\n");

    result.push('\n');
    result.push_str("    ");
    result.push_str(enum_name.as_str());
    result.push_str("() {\n");
    result.push_str("        this.ordinal = ordinal();\n");
    result.push_str("    }\n");

    result.push_str("}\n");

    result
}
