use std::{fs::read_to_string, io};
mod scanner;

fn check_index(grid: &Vec<Vec<char>>, x: i32, y: i32, c: char) -> bool {
    let x = x as usize;
    let y = y as usize;
    if grid.get(y).is_some() && grid[y].get(x).is_some() {
        if grid[y][x] == c {
            return true;
        }
    }
    return false;
}

fn check_direction(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    let mut valid = 0;
    if check_index(grid, x, y, 'A') {
        if check_index(grid, x + 1, y + 1, 'S') && check_index(grid, x - 1, y - 1, 'M') {
            valid += 1;
        }
        if check_index(grid, x - 1, y + 1, 'S') && check_index(grid, x + 1, y - 1, 'M') {
            valid += 1;
        }
        if check_index(grid, x + 1, y + 1, 'M') && check_index(grid, x - 1, y - 1, 'S') {
            valid += 1;
        }
        if check_index(grid, x - 1, y + 1, 'M') && check_index(grid, x + 1, y - 1, 'S') {
            valid += 1;
        }
    }
    if valid >= 2 {
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
            if check_direction(&grid, x as i32, y as i32) {
                result += 1;
            }
        }
    }
    println!("{}", result);
    Ok(())
}
