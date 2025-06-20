use clap::Parser;
use core::eval;
use core::tokenizer;

#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct Arg {
    pub path: String,
    // #[arg(short, long)]
    // pub flag: Option<String>,
}

fn main() {
    env_logger::init();

    // let args = Arg::parse();
    // let path = args.path.clone();
    let path = "test/main.tem";
    let mut code = std::fs::read_to_string(&path).unwrap();
    code.push('\0');
    let tokens = tokenizer::tokenizer(path.to_string(), code.chars().collect());
    // println!("{tokens:?}");
    eval::eval(tokens, std::collections::HashMap::new());
}
