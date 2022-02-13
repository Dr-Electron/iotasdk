use clap::{ArgEnum, Parser};

#[derive(Parser)]
pub enum Node {
    /// Install a single node or a private tangle
    Install(Install)
}

impl Node {
    pub fn run(self) {
        match self {
            Node::Install(install) => install.run()
        }
    }
}

#[derive(Parser)]
pub struct Install {
    /// Node type
    #[clap(arg_enum)]
    node: NodeType,
}

impl Install {
    fn run(self) {
        println!("Installing {:?} Node...", self.node)
    }
}

#[derive(ArgEnum, Clone, Debug)]
enum NodeType {
    Bee,
    GoShimmer,
    Hornet,
    Wasp
}