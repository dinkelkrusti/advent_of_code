use std::fs;

pub fn calories_on_top_elves(top_n: usize, filepath: &str) -> usize {
    let input = fs::read_to_string(filepath).unwrap();
    let meals = input.lines();

    let mut high_caloric_elves = vec![0; top_n];

    let mut calories_on_elf: usize = 0;
    for meal in meals {
        match meal.parse::<usize>() {
            Err(_) => {
                high_caloric_elves.sort();
                if calories_on_elf > high_caloric_elves[0] {
                    high_caloric_elves[0] = calories_on_elf;
                }
                calories_on_elf = 0
            }

            Ok(i) => calories_on_elf += i
        }
    }

    let total_calories = high_caloric_elves.into_iter().sum::<usize>();
    println!("The {} most high-caloric elves are carrying a total of {} calories.", top_n, total_calories);

    return total_calories;
}

#[deprecated(note = "Initial solution, but use the generalized version tbqh")]
fn part_one() {
    let input = fs::read_to_string("01.txt").unwrap();
    let meals = input.lines();

    let mut max_calories = 0;
    let mut calories_on_elf = 0;

    for meal in meals {
        match meal.parse::<i32>() {
            Err(_) => {
                if calories_on_elf > max_calories {
                    max_calories = calories_on_elf;
                }
                calories_on_elf = 0
            }

            Ok(i) => calories_on_elf += i
        }
    }

    println!("Maximum calories on any elf was {}.", max_calories);
}

#[deprecated(note = "Initial solution, but use the generalized version tbqh")]
fn part_two() {
    let input = fs::read_to_string("01.txt").unwrap();
    let meals = input.lines();

    let mut high_caloric_elves = vec!(0, 0, 0);
    let mut calories_on_elf = 0;

    for meal in meals {
        match meal.parse::<i32>() {
            Err(_) => {
                high_caloric_elves.sort();

                if calories_on_elf > high_caloric_elves[0] {
                    high_caloric_elves[0] = calories_on_elf;
                }
                calories_on_elf = 0
            }

            Ok(i) => calories_on_elf += i
        }
    }

    println!("The three most high-caloric elves are carrying a total of {} calories.", high_caloric_elves.into_iter().sum::<i32>())
}