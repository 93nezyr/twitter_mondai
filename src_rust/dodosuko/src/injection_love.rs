use anyhow::Result;
use rand::seq::SliceRandom;

/// # About
/// 
/// 楽しんごがドドスコしたりしなかったりします.
/// 
/// 楽しんごがドドスコスコスコ×3に成功すると、愛が注入されます.
/// 
/// DIならぬLIです(激寒).
/// 
/// ## Args
/// 
/// - `is_new_line` - 楽しんごを一行ごとにドドスコさせるか、一度に全ドドスコさせるかのフラグです. trueなら一行ごとにドドスコします.
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
