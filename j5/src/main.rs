use std::{fs::read_to_string, io};

use scanner::Rule;
mod scanner;

fn check_not_has(page: &[i32], not: i32) -> bool {
    for el in page {
        if *el == not {
            return false;
        }
    }
    return true;
}

fn valide_rule(page: &Vec<i32>, rule: &Rule) -> bool {
    for (i, el) in page.iter().enumerate() {
        if *el == rule.before && !check_not_has(&page[..i], rule.after) {
            return false;
        }
        if *el == rule.after && !check_not_has(&page[i..], rule.before) {
            return false;
        }
    }
    true
}

fn valide_page(page: &Vec<i32>, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if !valide_rule(page, rule) {
            return false;
        }
    }
    true
}

fn get_middle(page: &Vec<i32>) -> i32 {
    let len = page.len();
    let mid_index = len / 2;
    page[mid_index]
}

fn main() -> io::Result<()> {
    let content = read_to_string("input.txt")?;
    let content: Vec<&str> = content.split("\n\n").collect();
    let mut result: i32 = 0;
    let rules = Rule::from_str(content[0]);
    let pages: Vec<Vec<i32>> = content[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|el| el.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    for page in pages {
        if valide_page(&page, &rules) {
            result += get_middle(&page);
        }
    }
    println!("{}", result);
    Ok(())
}
