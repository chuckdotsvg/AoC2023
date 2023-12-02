use std::fs::read_to_string;

fn main() {
    let mut total = 0;
    let filename = String::from("/home/chuck/projects/aoc2023/day1/input");

    for line in read_to_string(filename).unwrap().lines() {
        let mut mynum = 0;
        let mut lastnum = 0;
        let mut count = 0;

        for c in line.chars() {
            if c.is_numeric() {
                count += 1;
                lastnum = c.to_digit(10).unwrap();

                if 1 == count {
                    mynum += lastnum * 10;
                }
            }
        }

        // append last number
        mynum += lastnum;
        total += mynum;
    }

    println!("{}", total);
}
