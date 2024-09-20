use std::env;
use tree::{LeafType, TreeLeaf};

pub mod tree;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: &str = if args.len() == 1 {
        "."
    } else {
        args[1].as_str()
    };

    println!("path: {}", path);

    let tree = TreeLeaf::new(path, 8, LeafType::Dir);
    tree.render_tree(8);
}
