use std::{fs, path::Path};
use crate::utils::bytes_to_string;

#[derive(Debug, Clone, Copy)]
pub enum EntryType {
    File,
    Dir,
    Else
}

pub struct FsTreeNode {
    weight: u64,
    name: String,
    children: Vec<FsTreeNode>,
    entry_type: EntryType
}

impl FsTreeNode {
    pub fn new() -> Self {
        FsTreeNode { weight: 0, name: String::new(), children: Vec::new(), entry_type: EntryType::Else }
    }

    pub fn from(source: &Path) -> Option<Self> {
        if source.is_file() || source.is_dir() {
            Some(FsTreeNode::fetch_from_source_rec(source))
        } else {
            None
        }
    }

    fn fetch_from_source_rec(source: &Path) -> Self {
        let mut children = Vec::new();

        let entry_type = if source.is_dir() { 
            EntryType::Dir 
        } else if source.is_file() {
            EntryType::File 
        } else {
            EntryType::Else
        };
        
        if let EntryType::Dir = entry_type {
            match fs::read_dir(source) {
                Ok(dir_iter) => for entry in dir_iter {
                    let entry = entry.expect("Cannot access dir-entry");
    
                    children.push(FsTreeNode::fetch_from_source_rec(entry.path().as_path()));
                },
                Err(_) => {}
            };
        }

        let weight: u64 = match entry_type {
            EntryType::File => source.metadata().unwrap().len(),
            EntryType::Dir => FsTreeNode::fetch_weight_from_children(&children),
            _ => 0
        };

        let name = match source.file_name() {
            Some(os_name) => {
                String::from(os_name.to_string_lossy())
            },
            None => String::new()
        };

        FsTreeNode { weight, name, children, entry_type }
    }

    fn fetch_weight_from_children(children: &Vec<FsTreeNode>) -> u64 {
        children.iter().fold(0, |acc, x| acc + x.weight)
    }

    pub fn print(&self, indent: u32) {
        println!("{}{:?}:/{}/: {}", 
            (0..indent).map(|_| "  ").collect::<String>(), 
            self.entry_type, self.name, 
            bytes_to_string(self.weight)
        );
        for entry in self.children.iter() {
            FsTreeNode::print(entry, indent + 1);
        }
    }

    pub fn get_weight(&self) -> u64 {
        self.weight
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_children(&self) -> &Vec<Self> {
        &self.children
    }

    pub fn get_type(&self) -> EntryType {
        self.entry_type
    }
}