use std::fs;

struct Rgb(u32, u32, u32);

fn main() {
    let path = "/home/chuck/projects/aoc2023/day2/input.txt";
    let contents = fs::read_to_string(path);

    let my_balls = Rgb(12, 13, 14);
    let mut count = 0;

    for lines in contents.unwrap_or("".to_owned()).lines() {
        let mut word_iterator = lines.split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace());

        let mut cur_game = word_iterator.nth(1).to_owned().unwrap_or("").parse::<u32>().unwrap_or(0);
        let mut num_balls = 0;

        while let Some(word) = word_iterator.next() {
            match word {
                "red" => {
                    if num_balls > my_balls.0 {
                        cur_game = 0;
                        while word_iterator.next().is_some() {}
                    }
                },
                "green" => {
                    if num_balls > my_balls.1 {
                        cur_game = 0;
                        while word_iterator.next().is_some() {}
                    }
                },
                "blue" => {
                    if num_balls > my_balls.2 {
                        cur_game = 0;
                        while word_iterator.next().is_some() {}
                    }
                },
                "Game" => (),
                _ => {
                    num_balls = word.to_owned().parse::<u32>().unwrap_or(0);
                }
            }
        }

        count += cur_game;
    }

    println!("{}", count);
}
