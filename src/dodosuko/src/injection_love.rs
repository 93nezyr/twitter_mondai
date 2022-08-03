use anyhow::Result;
use rand::seq::SliceRandom;

pub fn tano_singo(is_new_line: bool) -> Result<()> {
    let mut rng = rand::thread_rng();
    let dodosuko = ["ドド", "スコ"];
    let tanosingo_judge = "ドドスコスコスコドドスコスコスコドドスコスコスコ";
    let love = "ラブ注入♡";

    let mut tanosingo = "".to_string();
    loop {
        let ds = *dodosuko.choose(&mut rng).expect("dodosuko choice failed.");
        if is_new_line {
            println!("{}", ds);
        }
        // print!("{}", ds);
        tanosingo = tanosingo + ds;

        if tanosingo.ends_with(tanosingo_judge) {
            tanosingo = tanosingo + love;
            if is_new_line {
                println!("{}", love);
            }
            break;
        }
    }
    if !is_new_line {
        println!("{}", tanosingo);
    }

    Ok(())
}
