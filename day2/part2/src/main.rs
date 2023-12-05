use std::fs;

struct Rgb(u32, u32, u32);

fn main() {
    let path = "/home/chuck/projects/aoc2023/day2/input.txt";
    let contents = fs::read_to_string(path);

    let mut power_sum = 0;

    for lines in contents.unwrap().lines() {
        let mut word_iterator =
            lines.split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace());

        // skip Game x part
        word_iterator.nth(2);

        let mut num_balls = 0;
        let mut min_balls = Rgb(0, 0, 0);

        for word in word_iterator {
            match word {
                "red" => {
                    if num_balls > min_balls.0 {
                        min_balls.0 = num_balls;
                    }
                }
                "green" => {
                    if num_balls > min_balls.1 {
                        min_balls.1 = num_balls;
                    }
                }
                "blue" => {
                    if num_balls > min_balls.2 {
                        min_balls.2 = num_balls;
                    }
                }
                _ => {
                    num_balls = word.to_owned().parse::<u32>().unwrap_or(0);
                }
            }
        }

        let cur_power = min_balls.0 * min_balls.1 * min_balls.2;
        power_sum += cur_power;
    }

    println!("{}", power_sum);
}
