use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn create_java_tranformer(name: &str, values: &[&str], default: bool) {
    let mut file_content: String = "use crate::traits::FromJavaEnum;\n".to_string();

    if Path::new("./bindings/java/src/enums.rs").exists() {
        file_content = fs::read_to_string(Path::new("./bindings/java/src/enums.rs")).unwrap()
    }

    let mut enum_values: String = "".to_string();
    for (index, value) in values.iter().enumerate() {
        enum_values.push_str(format!("\n            {index} => {name}::{value},").as_str());
    }
    if default {
        enum_values.push_str(format!("\n            _ => {name}::default(),").as_str());
    } else {
        enum_values.push_str("\n            _ => panic!(\"Invalid value: {internal}\"),");
    }

    file_content = format!(
        "use taffy::{name};
{file_content}
impl FromJavaEnum<{name}> for {name} {{
    const JAVA_CLASS: &'static str = \"Lcom/dioxuslabs/taffy/enums/{name};\";

    fn from_ordinal(internal: i32) -> Option<{name}> {{
        Some(match internal {{{enum_values}
        }})
    }}
}}
"
    );

    let file = File::create("./bindings/java/src/enums.rs");
    file.expect("Error: File not found").write_all(file_content.as_ref()).expect("Error: Couldn't write to file");
}
