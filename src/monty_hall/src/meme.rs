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

    let mut no_change_memory = vec![];
    let mut change_memory = vec![];

    for _ in 0..n_trials {
        doors.shuffle(&mut rng);
        let first_choice = rng.gen_range(doors_range);
        
        // 扉を変えなかった場合
        if doors[first_choice] == keihin {
            no_change_memory.push(true);
        }

        // 扉を変えた場合
        // もともと正解を選んでいた場合.
        if doors[first_choice] == keihin {
            let mut monty_choices = get_unselected_numbers(first_choice, 0, doors.len());
            let monty_choice = *monty_choices.choose(&mut rng).expect("monty choice failed");
            
        }
        // もともと山羊を選んでいた場合.
        else {
            
        }

    }



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
