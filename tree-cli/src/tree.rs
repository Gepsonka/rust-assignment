use core::fmt;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
pub enum LeafType {
    Dir,
    File,
}

#[derive(Debug, Clone)]
pub struct TreeLeaf {
    pub path: String,
    pub level: u8,
    pub leaf_type: LeafType,
    pub child_nodes: Vec<TreeLeaf>,
}

impl TreeLeaf {
    pub fn new(path: &str, depth: u8, leaf_type: LeafType) -> TreeLeaf {
        let mut tree_leaf = TreeLeaf {
            path: String::from(path),
            level: depth,
            leaf_type,
            child_nodes: Vec::new(),
        };

        if tree_leaf.leaf_type == LeafType::Dir {
            tree_leaf.generate_tree(depth);
        }

        return tree_leaf;
    }

    fn generate_tree(&mut self, depth: u8) {
        if depth == 0 {
            return;
        }

        let dir_entries = fs::read_dir(self.path.clone()).unwrap();

        for entry_res in dir_entries {
            let entry = entry_res.unwrap();
            if entry.path().is_dir() {
                self.child_nodes.push(TreeLeaf::new(
                    entry.path().to_str().unwrap(),
                    depth - 1,
                    LeafType::Dir,
                ));
            } else {
                self.child_nodes.push(TreeLeaf::new(
                    entry.path().to_str().unwrap(),
                    depth - 1,
                    LeafType::File,
                ));
            }
        }
    }

    pub fn render_tree(&self, depth: u8) {
        for (index, entry) in self.child_nodes.clone().iter().enumerate() {
            let split_path: Vec<_> = std::path::Path::new(entry.path.as_str())
                .components()
                .collect();
            let filename = split_path[split_path.len() - 1]
                .as_os_str()
                .to_str()
                .unwrap();

            let connector_string: String;
            if index != 0 {
                connector_string =
                    String::from("|  ").repeat((depth - self.level) as usize) + "|--";
            } else {
                connector_string =
                    String::from("|  ").repeat((depth - self.level) as usize) + "└--";
            }

            println!("{}{}", connector_string, filename);

            // recursion
            match entry.leaf_type {
                LeafType::Dir => {
                    entry.render_tree(depth);
                }
                LeafType::File => {}
            }
        }
    }
}

impl fmt::Display for TreeLeaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.child_nodes)
    }
}
