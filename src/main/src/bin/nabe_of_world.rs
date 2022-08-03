use nabe_of_world::count_of_world;
use clap::Parser;

#[derive(Debug, Parser)]
/// # About
/// 
/// 世界の愚者をカウントします.
/// 
/// これが見えているということは--help or -hを引数に渡したのでしょう.
struct Args {
    /// 世界の愚者を数える上限を指定します.
    /// 
    /// --limit-of-world [n] の形で指定すると、nまで愚者を数えていただけます.
    #[clap(long, default_value = "40")]
    pub limit_of_world: usize,
}

fn main() {
    let args = Args::parse();
    count_of_world(args.limit_of_world).unwrap();
}