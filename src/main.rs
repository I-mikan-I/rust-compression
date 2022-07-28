use clap::Parser;
use compression::{Bwt, Coder, Huffman, MoveToFront};
use std::error::Error;
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
    bwt: bool,
}

fn main() {
    if let Err(e) = (|| {
        let args = Args::parse();
        let i = Path::new(&args.infile);
        let o = Path::new(&args.outfile);
        let decode = args.decode;
        let bwt = args.bwt;

        let contents = std::fs::read(i)?;
        let output = if !decode {
            if bwt {
                Huffman::encode(MoveToFront::encode(Bwt::encode(&contents)?)?)?
            } else {
                Huffman::encode(&contents)?
            }
        } else if bwt {
            Bwt::decode(MoveToFront::decode(Huffman::decode(&contents)?)?)?
        } else {
            Huffman::decode(&contents)?
        };
        println!(
            "Size before: {} bytes.\nSize after: {} bytes [{}%].",
            contents.len(),
            output.len(),
            output.len() * 100 / contents.len()
        );
        std::fs::write(o, output)?;
        <Result<(), Box<dyn Error + Send + Sync + 'static>>>::Ok(())
    })() {
        eprintln!("Error during execution: {}", e);
        std::process::exit(1);
    }
}
