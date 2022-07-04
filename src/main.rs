use clap::Parser;
use compression::{Coder, Huffman};
use std::path::Path;
#[derive(Parser)]
#[clap()]
struct Args {
    #[clap(short, long, value_parser)]
    infile: String,
    #[clap(short, long, value_parser)]
    outfile: String,
    #[clap(short, long, action)]
    decode: bool,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.decode);
    let i = Path::new(&args.infile);
    let o = Path::new(&args.outfile);
    let decode = args.decode;

    let contents = std::fs::read(i).unwrap_or_else(|e| {
        println!("Could not read from infile: {}", e);
        std::process::exit(1);
    });
    let output = if !decode {
        Huffman::encode(&contents)
    } else {
        Huffman::decode(&contents)
    };
    println!(
        "Size before: {} bytes.\nSize after: {} bytes [{}%].",
        contents.len(),
        output.len(),
        output.len() * 100 / contents.len()
    );
    if let Err(e) = std::fs::write(o, output) {
        println!("Could not write to outfile: {}", e);
        std::process::exit(1)
    }
}
