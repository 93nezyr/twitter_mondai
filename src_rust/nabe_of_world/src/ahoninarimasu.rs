use anyhow::Result;

/// # About
/// 
/// 世界の愚者を数えます.
/// 
/// ## Args
/// 
/// - `limit_of_world` - 世界の愚者を何人まで数えるかを指定できます.
/// 
/// ## ???
/// 
/// オモローーーーーーーーーーーーーーーーーーーーーーーーーーーーーーーーー！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！
pub fn count_of_world(limit_of_world: usize) -> Result<()> {
    for i in 1..=limit_of_world {
        if (i % 3 == 0) || i.to_string().contains("3") {
            println!("{}ぁぁぁぁぁん！！！！？？？？", i);
        } else {
            println!("{}", i);
        }
    }

    println!("オモローーーーーーーーーーーー！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！");

    Ok(())
}