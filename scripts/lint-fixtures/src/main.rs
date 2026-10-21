use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use regex::Regex;
use taffy::Style;
use walkdir::WalkDir;

/// Properties that Chrome resolves when generating fixtures but that Taffy does not model
const UNMODELLED_PROPERTIES: [&str; 8] =
    ["background", "background-color", "color", "border-style", "border-color", "outline", "writing-mode", "z-index"];

fn main() -> ExitCode {
    let fixtures_root = std::env::var("FIXTURE_DIR").map(PathBuf::from).unwrap_or_else(|_| {
        let root_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let repo_root = root_dir.parent().and_then(Path::parent).unwrap();
        repo_root.join("test_fixtures")
    });

    let fixture_paths: Vec<_> = WalkDir::new(fixtures_root)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "html"))
        .collect();

    let style_regex = Regex::new(r#"style="([^"]*)""#).unwrap();
    let comment_regex = Regex::new(r"(?s)/\*.*?\*/").unwrap();

    let mut failure_count = 0;
    for fixture_path in fixture_paths {
        let content = fs::read_to_string(&fixture_path).unwrap();
        for captures in style_regex.captures_iter(&content) {
            let style_attr = comment_regex.replace_all(&captures[1], "");
            for declaration in style_attr.split(';').map(str::trim).filter(|decl| !decl.is_empty()) {
                if let Err(error) = check_declaration(declaration) {
                    failure_count += 1;
                    println!("{}: `{declaration}`: {error}", fixture_path.display());
                }
            }
        }
    }

    if failure_count > 0 {
        println!("{failure_count} invalid test fixture style declaration(s)");
        ExitCode::FAILURE
    } else {
        println!("all test fixture style declarations are valid");
        ExitCode::SUCCESS
    }
}

fn check_declaration(declaration: &str) -> Result<(), String> {
    let (name, _) = declaration.split_once(':').ok_or("expected `property: value`")?;
    let name = name.trim().to_ascii_lowercase();
    if is_unmodelled(&name) {
        return Ok(());
    }
    declaration.parse::<Style<String>>().map(|_| ()).map_err(|err| err.to_string())
}

/// Paint-only properties and logical (`*-inline-*` / `*-block-*`) properties are resolved by Chrome
/// but not expressible in Taffy, so they are skipped rather than reported
fn is_unmodelled(name: &str) -> bool {
    UNMODELLED_PROPERTIES.contains(&name) || name.split('-').any(|word| word == "inline" || word == "block")
}
