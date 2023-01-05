use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

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
    let taffy_fixture_names = fs::read_dir(&taffy_fixtures_dir)
        .unwrap()
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

    let yoga_fixtures: Vec<YogaFixture> = fixtures_file_list
        .filter_map(|a| a.ok())
        .filter(|f| f.path().is_file() && f.path().extension().map(|p| p == "html").unwrap_or(false))
        .flat_map(|f| {
            let file_name: PathBuf = f.file_name().into();

            let file_content = fs::read_to_string(f.path()).unwrap();

            file_content
                .split("\n\n")
                .map(|snippet| {
                    let snippet = snippet.trim();
                    let first_quote_idx = snippet.chars().position(|c| c == '"').unwrap();
                    let name: String = snippet.chars().skip(first_quote_idx + 1).take_while(|c| *c != '"').collect();
                    let name_replace_snippet = snippet.replace(&name, "test-root");

                    let taffy_name = {
                        let name = name.replace("row_gap", "gap_row_gap");
                        name.replace("column_gap", "gap_column_gap")
                    };
                    YogaFixture { file_name: file_name.clone(), name: taffy_name, content: name_replace_snippet }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    for fixture in yoga_fixtures {
        if !taffy_fixture_names.contains(&fixture.name) {
            let mut new_fixture_path = taffy_fixtures_dir.join(&fixture.name);
            new_fixture_path.set_extension("html");
            let new_fixture_content = fixture_template.replace("__HTML_GOES_HERE__", &fixture.content);

            println!("Writing new fixture {}", &fixture.name);
            fs::write(new_fixture_path, new_fixture_content).unwrap();
        }
    }
}
