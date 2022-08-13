use clap::Parser;
use monty_hall::which_baabaa;

#[derive(Debug, Parser)]
/// # About
///
/// モンティホール問題の設定です.
///
/// おもに、シミュレーション実験における試行回数などを設定します.
struct Args {
    /// シミュレーション実験の試行回数を設定します.
    #[clap(long, default_value = "10000")]
    pub n_trials: usize,
}

fn main() {
    let args = Args::parse();
    which_baabaa(args.n_trials).unwrap();
}
