use roxmltree::Document;
use std::path::PathBuf;
use taffy::TaffyTree;

mod xml;

#[test]
fn get_dir() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_dir = root_dir.join("tests");
    let xml_test_dir = test_dir.join("xml");
    println!("{}", xml_test_dir.display())
}

fn run_xml_test(group: &str, name: &str) {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_dir = root_dir.join("tests");
    let xml_test_dir = test_dir.join("xml");

    let test_xml_file = {
        let mut path = xml_test_dir.join(group).join(name);
        path.set_extension("xml");
        path
    };

    let raw_xml = std::fs::read_to_string(&test_xml_file).expect("test file should exist");
    let test = Document::parse(&raw_xml).unwrap();

    let root = test.root_element();
    let input = root.children().find(|node| node.has_tag_name("input")).unwrap();
    let expectations = root.children().find(|node| node.has_tag_name("expectations")).unwrap();

    dbg!(input);
    dbg!(expectations);
}

fn construct_tree(xnode: &roxmltree::Node, tree: &mut TaffyTree, parent: Option<taffy::NodeId>) {
    let is_leaf = !xnode.has_children();

    if is_leaf {
        let text_content = xnode.text();
        let aspect_ratio = xnode.attribute("aspect-ratio");
        let writing_mode = xnode.attribute("writing_mode");

        let tnode = tree.new_leaf(build_style(xnode));
    }
}

fn build_style(xnode: &roxmltree::Node) -> taffy::Style {
    taffy::Style {
        dummy: std::marker::PhantomData,
        display: todo!(),
        item_is_table: todo!(),
        item_is_replaced: todo!(),
        box_sizing: todo!(),
        overflow: todo!(),
        scrollbar_width: todo!(),
        float: todo!(),
        clear: todo!(),
        position: todo!(),
        inset: todo!(),
        size: todo!(),
        min_size: todo!(),
        max_size: todo!(),
        aspect_ratio: todo!(),
        margin: todo!(),
        padding: todo!(),
        border: todo!(),
        align_items: todo!(),
        align_self: todo!(),
        justify_items: todo!(),
        justify_self: todo!(),
        align_content: todo!(),
        justify_content: todo!(),
        gap: todo!(),
        text_align: todo!(),
        flex_direction: todo!(),
        flex_wrap: todo!(),
        flex_basis: todo!(),
        flex_grow: todo!(),
        flex_shrink: todo!(),
        grid_template_rows: todo!(),
        grid_template_columns: todo!(),
        grid_auto_rows: todo!(),
        grid_auto_columns: todo!(),
        grid_auto_flow: todo!(),
        grid_template_areas: todo!(),
        grid_template_column_names: todo!(),
        grid_template_row_names: todo!(),
        grid_row: todo!(),
        grid_column: todo!(),
    }
}
