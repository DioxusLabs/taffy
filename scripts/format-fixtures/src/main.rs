use std::{path::{PathBuf, Path}, fs};

use regex::{Regex, Captures};

fn main() {
    let fixtures_root = std::env::var("FIXTURE_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        let root_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let repo_root = root_dir.parent().and_then(Path::parent).unwrap();
        let fixtures_root = repo_root.join("test_fixtures");
        fixtures_root
    });
    let fixtures = fs::read_dir(fixtures_root).unwrap();

    println!("reading test fixtures from disk");
    let fixtures: Vec<_> = fixtures
        .into_iter()
        .filter_map(|a| a.ok())
        .filter(|f| f.path().is_file() && f.path().extension().map(|p| p == "html").unwrap_or(false))
        .map(|f| {
            let fixture_path = f.path();
            let name = fixture_path.file_stem().unwrap().to_str().unwrap().to_string();
            (name, fixture_path)
        })
        .collect();

    let style_regex = Regex::new(r#"style="([^"]*)""#).unwrap();

    for (_, fixture_path) in fixtures {
        let content = fs::read_to_string(&fixture_path).unwrap();

        let formatted_content = style_regex.replace_all(&content, |captures: &Captures| {
            let style_attr = captures.get(0).unwrap().as_str();
            format_style_attr(style_attr)
        });

        fs::write(&fixture_path, formatted_content.as_ref()).unwrap();
    }
}

fn format_style_attr(input: &str) -> String {
    let mut attr = String::with_capacity(1024);
    attr.push_str("style=\"");

    let input = &input[7..(input.len() - 1)];
    let declarations = input.split(";").map(|decl| decl.trim()).filter(|decl| !decl.is_empty());
    let mut has_declarations = false;
    for decl in declarations {
        has_declarations = true;
        let (key, value) = decl.split_once(":").unwrap();
        attr.push_str(key.trim());
        attr.push_str(": ");
        attr.push_str(value.trim());
        attr.push_str("; ");
    }
    if has_declarations {
        attr.pop();
    }
    attr.push('"');

    attr
}