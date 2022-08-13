use anyhow::Result;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn which_baabaa(n_trials: usize) -> Result<()> {
    let mut n_trials = n_trials;
    if n_trials <= 0 {
        n_trials = 1000;
    }

    let mut rng = rand::thread_rng();
    let keihin = "スーパーカー!!!!!!OMG!!!!!!!!!!!!!!!!!!!!!!";
    let mut doors = ["山羊", "山羊", keihin];
    let doors_range = 0..=(doors.len() - 1);

    // let mut no_change_memory = vec![];
    // let mut change_memory = vec![];
    let mut count_of_no_change = 0;
    let mut count_of_change = 0;

    for _ in 0..n_trials {
        doors.shuffle(&mut rng);
        let first_choice = rng.gen_range(doors_range.clone());

        // 扉を変えなかった場合
        if doors[first_choice] == keihin {
            // no_change_memory.push(true);
            count_of_no_change += 1;
        }

        // 扉を変えた場合
        // もともと正解を選んでいた場合.
        if doors[first_choice] == keihin {
            // 選ばれていない扉番号.
            let monty_choices = get_unselected_numbers(first_choice, 0, doors.len());
            // 選ばれていない扉番号から、一つを残してそれ以外を外れであると、回答者に開示する.
            let remained_choice = *monty_choices.choose(&mut rng).expect("monty choice failed");
            // 残された一つの番号の扉に、回答者は変更する.
            let _second_choice = remained_choice;
            // その扉が正解かどうかを判定する.
            // もともと正解を選んでいた場合、必ず失敗となる.
        }
        // もともと山羊を選んでいた場合.
        else {
            // 選ばれていない扉番号.
            let monty_choices = get_unselected_numbers(first_choice, 0, doors.len());
            // 選ばれていない扉番号から、一つを残してそれ以外を外れであると、回答者に開示する.
            // この場合、残す扉番号は、必ず正解の扉番号である.
            let remained_choice = {
                let mut remained_choice = 0;
                for i in monty_choices.iter() {
                    if doors[*i] == keihin {
                        remained_choice = *i;
                    }
                }
                remained_choice
            };
            let second_choice = remained_choice;
            if doors[second_choice] == keihin {
                count_of_change += 1;
            }
        }
    }

    println!("no change pattern : {}/{}", count_of_no_change, n_trials);
    println!("change pattern: {}/{}", count_of_change, n_trials);
    let rate = count_of_change as f32 / count_of_no_change as f32;
    println!("change pattern / no change pattern = {}", rate);

    Ok(())
}

fn get_unselected_numbers(selected_number: usize, min: usize, max: usize) -> Vec<usize> {
    let mut v = vec![];
    for i in min..max {
        if i == selected_number {
            continue;
        }
        v.push(i);
    }
    v
}
