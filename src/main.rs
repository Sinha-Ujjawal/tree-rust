use clap::Parser;
use tree::Tree;
mod tree;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let tree = Tree::new(cli.path).expect("Make sure that the path exists and is a directory");
    tree.tree();
}
