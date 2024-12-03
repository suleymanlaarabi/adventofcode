use std::{fs::read_to_string, io};
mod scanner;
use scanner::Scanner;

fn main() -> io::Result<()> {
    let content = read_to_string("input.txt")?;
    let mut result: f64 = 0.0;
    let mut scanner = Scanner::from_string(content);
    let mut enabled = true;

    loop {
        if scanner.check_and_skip("do()") {
            enabled = true;
        } else if scanner.check_and_skip("don't()") {
            enabled = false;
        } else if scanner.check_and_skip("mul(") {
            if let Some(value) = scanner.parse_number() {
                if let Some(c) = scanner.pop() {
                    if *c == ',' {
                        if let Some(value2) = scanner.parse_number() {
                            if let Some(l) = scanner.pop() {
                                if *l == ')' && enabled {
                                    result += value * value2;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            scanner.pop();
        }
        if scanner.is_done() {
            break;
        }
    }
    println!("{}", result);
    Ok(())
}
