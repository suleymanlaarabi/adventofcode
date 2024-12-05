use std::{fs::read_to_string, io};
mod scanner;

fn get_string(grid: &Vec<Vec<char>>, direction: (i32, i32), x: usize, y: usize) -> String {
    let mut str = String::new();
    for i in 0..4 {
        let x = x as i32 + (direction.0 * i);
        let y = y as i32 + (direction.1 * i);
        match grid.get(y as usize) {
            Some(result) => match result.get(x as usize) {
                Some(c) => str.push(*c),
                None => return String::new(),
            },
            None => return String::new(),
        }
    }
    str
}

fn check_direction(grid: &Vec<Vec<char>>, direction: (i32, i32), x: usize, y: usize) -> bool {
    let str = get_string(grid, (direction.0, direction.1), x, y);
    if str.starts_with("XMAS") {
        return true;
    }
    return false;
}

fn main() -> io::Result<()> {
    let content = read_to_string("input.txt")?;
    let mut result: usize = 0;

    let grid: Vec<&str> = content.lines().collect();
    let grid: Vec<Vec<char>> = grid.iter().map(|str| str.chars().collect()).collect();

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if check_direction(&grid, (0, 1), x, y) {
                result += 1;
            }
            if check_direction(&grid, (1, 1), x, y) {
                result += 1;
            }
            if check_direction(&grid, (1, 0), x, y) {
                result += 1;
            }
            if check_direction(&grid, (-1, 1), x, y) {
                result += 1;
            }
            if check_direction(&grid, (1, -1), x, y) {
                result += 1;
            }

            if check_direction(&grid, (0, -1), x, y) {
                result += 1;
            }
            if check_direction(&grid, (-1, -1), x, y) {
                result += 1;
            }
            if check_direction(&grid, (-1, 0), x, y) {
                result += 1;
            }
        }
    }
    println!("{}", result);
    Ok(())
}
