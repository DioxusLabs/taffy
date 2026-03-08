use roxmltree::Document;
use std::path::PathBuf;

mod xml;

#[test]
fn get_dir() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_dir = root_dir.join("tests");
    let xml_test_dir = test_dir.join("xml");
    println!("{}", xml_test_dir.display())
}

fn run_xml_test(name: &str) {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_dir = root_dir.join("tests");
    let xml_test_dir = test_dir.join("xml");

    let test_xml_file = {
        let mut path = xml_test_dir.join(name);
        path.set_extension("xml");
        path
    };

    let test = Document::parse(&std::fs::read_to_string(&test_xml_file).expect("test file should exist"));
}
