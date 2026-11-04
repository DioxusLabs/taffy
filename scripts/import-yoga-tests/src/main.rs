use regex::{Captures, Regex};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

struct YogaFixture {
    #[allow(dead_code)]
    file_name: PathBuf,
    name: String,
    content: String,
}

fn main() {
    // Get Taffy fixtures dir
    let script_root_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let taffy_repo_root = script_root_dir.parent().and_then(Path::parent).unwrap();
    let taffy_fixtures_dir = taffy_repo_root.join("test_fixtures");

    // Get Taffy fixture names
    let taffy_fixture_names = WalkDir::new(&taffy_fixtures_dir)
        .into_iter()
        .filter_map(|a| a.ok())
        .map(|f| {
            let file_name = f.file_name();
            let file_name = if file_name.to_string_lossy().starts_with('x') {
                file_name.to_string_lossy()[1..].replace(".html", "")
            } else {
                file_name.to_string_lossy().replace(".html", "")
            };
            file_name
        })
        .collect::<HashSet<String>>();

    // Get Taffy fixture template
    let fixture_template_path = script_root_dir.join("FIXTURE_TEMPLATE.html");
    let fixture_template = fs::read_to_string(fixture_template_path).unwrap();

    // Get Yoga fixtures dir
    let yoga_fixtures_dir =
        PathBuf::from(std::env::var("YOGA_FIXTURE_DIR").expect("YOGA_FIXTURE_DIR env var must be set"));
    let fixtures_file_list = fs::read_dir(yoga_fixtures_dir).unwrap();

    let div_tag = Regex::new(r#"</?div\b[^>]*>"#).unwrap();
    let id_attr = Regex::new(r#"\bid="([^"]+)""#).unwrap();
    let element_tag = Regex::new(r#"<(?:div|span)\b[^>]*>"#).unwrap();
    let style_attr = Regex::new(r#"style="([^"]*)""#).unwrap();

    let yoga_fixtures: Vec<YogaFixture> = fixtures_file_list
        .filter_map(|a| a.ok())
        .filter(|f| f.path().is_file() && f.path().extension().map(|p| p == "html").unwrap_or(false))
        .flat_map(|f| {
            let file_name: PathBuf = f.file_name().into();

            let file_content = fs::read_to_string(f.path()).unwrap();

            let mut fixtures = Vec::new();
            let mut depth = 0;
            let mut start = 0;
            for tag in div_tag.find_iter(&file_content) {
                if tag.as_str().starts_with("</") {
                    depth -= 1;
                    if depth == 0 {
                        let snippet = &file_content[start..tag.end()];
                        let name = id_attr.captures(snippet).unwrap()[1].to_owned();
                        let content = id_attr.replacen(snippet, 1, "id=\"test-root\"");
                        let content = element_tag
                            .replace_all(&content, |tag: &Captures| {
                                let style = style_attr.captures(&tag[0]);
                                let original = style.as_ref().map(|s| &s[1]).unwrap_or("");
                                let properties: HashSet<_> = original
                                    .split(';')
                                    .filter_map(|d| d.split_once(':').map(|(p, _)| p.trim()))
                                    .collect();
                                let mut defaults = String::new();
                                for (property, value) in [
                                    ("flex-direction", "column"),
                                    ("flex-shrink", "0"),
                                    ("align-content", "flex-start"),
                                ] {
                                    if !properties.contains(property)
                                        && !(property == "flex-shrink" && properties.contains("flex"))
                                    {
                                        defaults.push_str(&format!("{property}: {value}; "));
                                    }
                                }
                                if style.is_some() {
                                    style_attr
                                        .replace(&tag[0], format!("style=\"{defaults}{original}\"").as_str())
                                        .into_owned()
                                } else {
                                    format!("{} style=\"{}\">", tag[0].trim_end_matches('>'), defaults.trim())
                                }
                            })
                            .into_owned();
                        let name = name.replace("row_gap", "gap_row_gap").replace("column_gap", "gap_column_gap");
                        fixtures.push(YogaFixture { file_name: file_name.clone(), name, content });
                    }
                } else {
                    if depth == 0 {
                        start = tag.start();
                    }
                    depth += 1;
                }
            }
            fixtures
        })
        .collect();

    for fixture in yoga_fixtures {
        if !taffy_fixture_names.contains(&fixture.name) {
            let mut new_fixture_path = taffy_fixtures_dir.join("flex").join(&fixture.name);
            new_fixture_path.set_extension("html");
            let new_fixture_content = fixture_template.replace("__HTML_GOES_HERE__", &fixture.content);

            println!("Writing new fixture {}", fixture.name);
            fs::write(new_fixture_path, new_fixture_content).unwrap();
        }
    }
}
