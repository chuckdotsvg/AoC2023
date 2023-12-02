use std::{fs::read_to_string, collections::HashMap};

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Default for TrieNode {
    fn default() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }


    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }

        current_node.is_end_of_word
    }
}

fn main() {
    let mut total = 0;
    let filename = String::from("/home/chuck/projects/aoc2023/day1/part2/test");

    for line in read_to_string(filename).unwrap().lines() {
        let mut mytrie = Trie::new();

        for c in iterator {
            if c.is_ascii_digit() {
                // new number found
                count += 1;
                chardig = c.to_digit(10).unwrap();

                //buffer cleared
                buffer.clear();
            } else if c.is_alphabetic() {
                buffer.push(c)
            }

            match buffer.as_str() {
                "one" => strtoval = 1,
                "two" => strtoval = 2,
                "three" => strtoval = 3,
                "four" => strtoval = 4,
                "five" => strtoval = 5,
                "six" => strtoval = 6,
                "seven" => strtoval = 7,
                "eight" => strtoval = 8,
                "nine" => strtoval = 9,
                _ => strtoval = 0,
            }

            if strtoval != 0 {
                // assign to the variable to be added
                chardig = strtoval;
                count += 1;

                // reset buffer
                buffer.clear();
            }

            if 1 == count && buffer.is_empty() {
                total += chardig * 10;
                print!("{}", chardig);
            }
        }

        total += chardig;
        println!("{}", chardig);
    }

    println!("{}", total);
}
