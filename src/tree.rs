use std::io;

pub struct Tree {
    pub root_path: std::path::PathBuf,
    _hidden: (),
}

impl Tree {
    pub fn new(root_path: std::path::PathBuf) -> Option<Self> {
        if root_path.exists() && root_path.is_dir() {
            Some(Tree {
                root_path,
                _hidden: (),
            })
        } else {
            None
        }
    }

    pub fn tree(self, w: &mut impl io::Write) -> io::Result<()> {
        return Self::tree_recurse(w, 0, &self.root_path);
    }

    fn tree_recurse(
        w: &mut impl io::Write,
        indent: usize,
        path: &std::path::PathBuf,
    ) -> io::Result<()> {
        let entries = std::fs::read_dir(path).unwrap();
        entries
            .take_while(Result::is_ok)
            .map(Result::unwrap)
            .try_for_each(move |entry| {
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap();
                let file_type = entry.file_type()?;
                let prefix = "  ".repeat(indent);

                w.write(format!("{}|-", prefix).as_bytes())?;
                w.write(file_name.as_bytes())?;
                w.write(b"\n")?;

                if file_type.is_dir() {
                    Self::tree_recurse(w, indent + 1, &entry.path())
                } else {
                    Ok(())
                }
            })
    }
}
