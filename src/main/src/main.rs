use clap::Parser;
use dodosuko::tano_singo;

#[derive(Debug, Parser)]
/// # About
/// 
/// コマンドライン引数を受け付ける構造体です.
struct Args {
    /// tano_singoを一行ごとにドドスコさせるか、一度に全ドドスコさせるかのフラグ.
    /// 
    /// trueなら、一行ごとにドドスコする.
    #[clap(long)]
    pub is_new_line: bool,
}

fn main() {
    // println!("Hello, world!");
    let args = Args::parse();
    println!("{}", args.is_new_line);

    tano_singo(args.is_new_line).unwrap();
}
