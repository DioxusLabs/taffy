use roxmltree::Document;
use std::{io::Write, path::PathBuf, str::FromStr};
use taffy::{
    prelude::TaffyZero as _, AvailableSpace, CheapCloneStr, Dimension, GridAutoTracks, GridTemplateComponent,
    GridTemplateTracks, LengthPercentage, LengthPercentageAuto, Line, NodeId, Point, PrintTree, Rect, Size, TaffyTree,
};
use taffy_test_helpers::{test_measure_function, TestNodeContext};

mod xml;

#[derive(Debug)]
struct OutputNode {
    node_id: NodeId,
    location: Point<f32>,
    size: Size<f32>,
    children: Vec<OutputNode>,
}

impl PartialEq for OutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
            && (self.location.x - other.location.x).abs() < 0.1
            && (self.location.y - other.location.y).abs() < 0.1
            && (self.size.width - other.size.width).abs() < 0.1
            && (self.size.height - other.size.height).abs() < 0.1
            && self.children == other.children
    }
}

impl OutputNode {
    fn write_metrics(&self, w: &mut impl Write) {
        write!(
            w,
            "[x: {:<4} y: {:<4} w: {:<4} h: {:<4}]",
            self.location.x, self.location.y, self.size.width, self.size.height
        )
        .unwrap();
    }

    /// Prints a debug representation of the computed layout for a tree of nodes
    fn write_tree(&self, out: &mut impl Write) {
        writeln!(out, "TREE").unwrap();
        print_node(out, self, false, String::new());

        /// Recursive function that prints each node in the tree
        fn print_node(out: &mut impl Write, node: &OutputNode, has_sibling: bool, lines_string: String) {
            let num_children = node.children.len();
            let fork_string = if has_sibling { "├── " } else { "└── " };

            write!(out, "{lines_string}{fork_string} {:?} ", node.node_id).unwrap();
            node.write_metrics(out);
            let bar = if has_sibling { "│   " } else { "    " };
            let new_string = lines_string + bar;
            writeln!(out).unwrap();

            // Recurse into children
            for (index, child) in node.children.iter().enumerate() {
                let has_sibling = index < num_children - 1;
                print_node(out, child, has_sibling, new_string.clone());
            }
        }
    }

    fn print_tree(&self) {
        let mut s: Vec<u8> = Vec::new();
        self.write_tree(&mut s);
        let s = String::from_utf8(s).unwrap();
        println!("{s}");
    }
}

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
    let use_rounding: bool = parse_or(root.attribute("use-rounding"), true);
    let viewport = root.children().find(|node| node.has_tag_name("viewport")).unwrap();
    let input = root.children().find(|node| node.has_tag_name("input")).unwrap();
    let expectations = root.children().find(|node| node.has_tag_name("expectations")).unwrap();

    let available_space = Size {
        width: parse_or(viewport.attribute("width"), AvailableSpace::MaxContent),
        height: parse_or(viewport.attribute("height"), AvailableSpace::MaxContent),
    };

    // Construct tree and expectations
    let mut tree = TaffyTree::<TestNodeContext>::new();
    if use_rounding {
        tree.enable_rounding();
    } else {
        tree.disable_rounding();
    }
    let expected_output = construct_tree(
        input.first_element_child().unwrap(),
        expectations.first_element_child().unwrap(),
        &mut tree,
        None,
    );
    let root_node_id = expected_output.node_id;

    // Compute layout
    tree.compute_layout_with_measure(root_node_id, available_space, test_measure_function).unwrap();
    let actual_output = get_computed_expectations(&tree, root_node_id);

    println!("\nINPUT");
    println!("{raw_xml}");

    tree.print_tree(root_node_id);

    println!("\nEXPECTED");
    expected_output.print_tree();
    println!("\nACTUAL");
    actual_output.print_tree();

    assert_eq!(expected_output, actual_output);
}

fn construct_tree(
    input: roxmltree::Node,
    expected_x: roxmltree::Node,
    tree: &mut TaffyTree<TestNodeContext>,
    parent: Option<taffy::NodeId>,
) -> OutputNode {
    if input.first_element_child().is_some() {
        let tnode = tree.new_with_children(build_style(input), &[]).unwrap();
        let mut expected = build_expectations(expected_x, tnode);

        if let Some(parent) = parent {
            tree.add_child(parent, tnode).unwrap();
        }

        for (child, child_expected_x) in input
            .children()
            .filter(|node| node.is_element())
            .zip(expected_x.children().filter(|node| node.is_element()))
        {
            expected.children.push(construct_tree(child, child_expected_x, tree, Some(tnode)));
        }
        expected
    } else {
        let text_content = input.text().map(|text| text.trim());
        // let aspect_ratio = input.attribute("aspect-ratio");
        let writing_mode = parse_or_default(input.attribute("writing-mode"));

        let tnode = tree.new_leaf(build_style(input)).unwrap();
        tree.set_node_context(
            tnode,
            text_content.map(|text_content| TestNodeContext::ahem_text(text_content.to_string(), writing_mode)),
        )
        .unwrap();

        if let Some(parent) = parent {
            tree.add_child(parent, tnode).unwrap();
        }

        build_expectations(expected_x, tnode)
    }
}

fn parse_or_default<T: FromStr + Default>(input: Option<&str>) -> T {
    parse_or(input, Default::default())
}

fn parse_or<T: FromStr>(input: Option<&str>, fallback: T) -> T {
    // input.map(|input| input.parse().map_err(|_| "").unwrap()).unwrap_or(fallback)
    input.and_then(|input| input.parse().ok()).unwrap_or(fallback)
}

fn maybe_parse<T: FromStr>(input: Option<&str>) -> Option<T> {
    input.and_then(|input| input.parse().ok())
}

fn get_computed_expectations(tree: &TaffyTree<TestNodeContext>, node_id: NodeId) -> OutputNode {
    let layout = tree.get_final_layout(node_id);
    let mut output = OutputNode { node_id, location: layout.location, size: layout.size, children: Vec::new() };

    for child_id in tree.children(node_id).unwrap() {
        output.children.push(get_computed_expectations(tree, child_id));
    }

    output
}

fn build_expectations(xnode: roxmltree::Node, node_id: NodeId) -> OutputNode {
    OutputNode {
        node_id,
        location: Point {
            x: xnode.attribute("x").unwrap().parse().unwrap(),
            y: xnode.attribute("y").unwrap().parse().unwrap(),
        },
        size: Size {
            width: xnode.attribute("width").unwrap().parse().unwrap(),
            height: xnode.attribute("height").unwrap().parse().unwrap(),
        },
        children: Vec::new(),
    }
}

fn build_style<S: CheapCloneStr>(xnode: roxmltree::Node) -> taffy::Style<S> {
    let grid_template_rows: GridTemplateTracks<S, GridTemplateComponent<S>> =
        parse_or_default(xnode.attribute("grid-template-rows"));
    let grid_template_columns: GridTemplateTracks<S, GridTemplateComponent<S>> =
        parse_or_default(xnode.attribute("grid-template-columns"));

    taffy::Style {
        dummy: std::marker::PhantomData,
        display: parse_or_default(xnode.attribute("display")),
        item_is_table: false,
        item_is_replaced: false,
        box_sizing: parse_or_default(xnode.attribute("box-sizing")),
        overflow: Point {
            x: parse_or_default(xnode.attribute("overflow-x")),
            y: parse_or_default(xnode.attribute("overflow-y")),
        },
        scrollbar_width: parse_or_default(xnode.attribute("scrollbar-width")),
        float: parse_or_default(xnode.attribute("float")),
        clear: parse_or_default(xnode.attribute("clear")),
        position: parse_or_default(xnode.attribute("position")),

        size: Size {
            width: parse_or(xnode.attribute("width"), Dimension::auto()),
            height: parse_or(xnode.attribute("height"), Dimension::auto()),
        },
        min_size: Size {
            width: parse_or(xnode.attribute("min-width"), Dimension::auto()),
            height: parse_or(xnode.attribute("min-height"), Dimension::auto()),
        },
        max_size: Size {
            width: parse_or(xnode.attribute("max-width"), Dimension::auto()),
            height: parse_or(xnode.attribute("max-height"), Dimension::auto()),
        },
        inset: Rect {
            top: parse_or(xnode.attribute("top"), LengthPercentageAuto::auto()),
            left: parse_or(xnode.attribute("left"), LengthPercentageAuto::auto()),
            bottom: parse_or(xnode.attribute("bottom"), LengthPercentageAuto::auto()),
            right: parse_or(xnode.attribute("right"), LengthPercentageAuto::auto()),
        },
        margin: Rect {
            top: parse_or(xnode.attribute("margin-top"), LengthPercentageAuto::ZERO),
            left: parse_or(xnode.attribute("margin-left"), LengthPercentageAuto::ZERO),
            bottom: parse_or(xnode.attribute("margin-bottom"), LengthPercentageAuto::ZERO),
            right: parse_or(xnode.attribute("margin-right"), LengthPercentageAuto::ZERO),
        },
        padding: Rect {
            top: parse_or(xnode.attribute("padding-top"), LengthPercentage::ZERO),
            left: parse_or(xnode.attribute("padding-left"), LengthPercentage::ZERO),
            bottom: parse_or(xnode.attribute("padding-bottom"), LengthPercentage::ZERO),
            right: parse_or(xnode.attribute("padding-right"), LengthPercentage::ZERO),
        },
        border: Rect {
            top: parse_or(xnode.attribute("border-top"), LengthPercentage::ZERO),
            left: parse_or(xnode.attribute("border-left"), LengthPercentage::ZERO),
            bottom: parse_or(xnode.attribute("border-bottom"), LengthPercentage::ZERO),
            right: parse_or(xnode.attribute("border-right"), LengthPercentage::ZERO),
        },
        gap: Size {
            width: parse_or(xnode.attribute("column-gap"), LengthPercentage::ZERO),
            height: parse_or(xnode.attribute("row-gap"), LengthPercentage::ZERO),
        },

        aspect_ratio: maybe_parse(xnode.attribute("aspect-ratio")),
        align_items: maybe_parse(xnode.attribute("align-items")),
        align_self: maybe_parse(xnode.attribute("align-self")),
        justify_items: maybe_parse(xnode.attribute("justify-items")),
        justify_self: maybe_parse(xnode.attribute("justify-self")),
        align_content: maybe_parse(xnode.attribute("align-content")),
        justify_content: maybe_parse(xnode.attribute("justify-content")),

        text_align: parse_or_default(xnode.attribute("text-align")),
        flex_direction: parse_or_default(xnode.attribute("flex-direction")),
        flex_wrap: parse_or_default(xnode.attribute("flex-wrap")),
        flex_grow: parse_or(xnode.attribute("flex-grow"), 0.0),
        flex_shrink: parse_or(xnode.attribute("flex-shrink"), 1.0),
        flex_basis: parse_or(xnode.attribute("flex-basis"), Dimension::auto()),

        grid_auto_flow: parse_or_default(xnode.attribute("grid-auto-flow")),

        grid_template_rows: grid_template_rows.tracks,
        grid_template_row_names: grid_template_rows.line_names,

        grid_template_columns: grid_template_columns.tracks,
        grid_template_column_names: grid_template_columns.line_names,

        // TODO
        grid_auto_rows: parse_or_default::<GridAutoTracks>(xnode.attribute("grid-auto-rows")).0,
        grid_auto_columns: parse_or_default::<GridAutoTracks>(xnode.attribute("grid-auto-columns")).0,
        grid_template_areas: Default::default(),

        grid_row: Line {
            start: parse_or_default(xnode.attribute("grid-row-start")),
            end: parse_or_default(xnode.attribute("grid-row-end")),
        },
        grid_column: Line {
            start: parse_or_default(xnode.attribute("grid-column-start")),
            end: parse_or_default(xnode.attribute("grid-column-end")),
        },
    }
}
