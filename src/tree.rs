pub struct Tree {
    pub root_path: std::path::PathBuf,
    _hidden: (),
}

impl Tree {
    pub fn new(root_path: std::path::PathBuf) -> Option<Self> {
        if root_path.exists() && root_path.is_dir() {
            Some(Tree { root_path, _hidden: ()})
        } else {
            None
        }
    }

    pub fn tree(self) -> () {
        Self::tree_recurse(0, &self.root_path);
    }

    fn tree_recurse(indent: usize, path: &std::path::PathBuf) -> () {
        let entries = std::fs::read_dir(path).unwrap();
        entries.filter_map(|d| d.ok()).for_each(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            entry.file_type().iter().for_each(|file_type| {
                let prefix = "  ".repeat(indent);
                print!("{}|-", prefix);
                println!("{}", file_name);
                if file_type.is_dir() {
                    Self::tree_recurse(indent + 1, &entry.path());
                }
            })
        })
    }
}