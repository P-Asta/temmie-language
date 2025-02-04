mod tokenizer;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct Arg {
    pub path: String,
    // #[arg(short, long)]
    // pub flag: Option<String>,
}

fn main() {
    let args = Arg::parse();
    println!("{:?}", args);
    let mut code = std::fs::read_to_string(args.path).unwrap();
    code.push('\0');
    print!("{:?}", tokenizer::tokenizer(code.chars().collect()));
}
