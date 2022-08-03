use clap::Parser;
use dodosuko::tano_singo;

#[derive(Debug, Parser)]
/// # About
/// 
/// ドドスコプログラムを実行します.
/// 
/// これが見えているということは--help or -hを引数に渡したのでしょう.
struct Args {
    /// tano_singoを一行ごとにドドスコさせるか、一度に全ドドスコさせるかのフラグ.
    /// 
    /// この引数を指定すると、一行ごとにドドスコする.
    #[clap(long)]
    pub is_new_line: bool,
}

fn main() {
    // println!("Hello, world!");
    let args = Args::parse();
    println!("{}", args.is_new_line);

    tano_singo(args.is_new_line).unwrap();
}
