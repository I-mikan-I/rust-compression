use clap::Parser;
use compression::{Coder, Huffman, MoveToFront};
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
    #[clap(short, long, action)]
    m2f: bool,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.decode);
    let i = Path::new(&args.infile);
    let o = Path::new(&args.outfile);
    let decode = args.decode;
    let m2f = args.m2f;

    let contents = std::fs::read(i).unwrap_or_else(|e| {
        println!("Could not read from infile: {}", e);
        std::process::exit(1);
    });
    let output = if !decode {
        if m2f {
            Huffman::encode(&MoveToFront::encode(&contents))
        } else {
            Huffman::encode(&contents)
        }
    } else if m2f {
        MoveToFront::decode(&Huffman::decode(&contents))
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
