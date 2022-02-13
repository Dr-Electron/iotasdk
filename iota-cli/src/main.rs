use iota_cli::{Args, SubCommand};
use clap::Parser;

fn main() {
    let args = Args::parse();

    match args.subcmd {
        SubCommand::Node(node) => {
            node.run()
        }
    }

    println!("Hello, world!");
}
