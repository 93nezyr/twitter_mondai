use dodosuko::tano_singo;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// tano_singoを一行ごとにドドスコさせるか、一度に全ドドスコさせるかのフラグ.
    #[clap(long)]
    pub is_new_line: bool,
}

fn main() {
    // println!("Hello, world!");
    let args = Args::parse();
    println!("{}", args.is_new_line);

    tano_singo(args.is_new_line).unwrap();
}
