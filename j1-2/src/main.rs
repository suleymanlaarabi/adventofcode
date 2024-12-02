use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let content = fs::read_to_string("input.txt")?;
    let lignes: Vec<&str> = content.lines().collect();

    let mut liste1 = Vec::new();
    let mut liste2 = Vec::new();

    for ligne in lignes {
        let nombres: Vec<&str> = ligne.split_whitespace().collect();
        if nombres.len() == 2 {
            liste1.push(nombres[0].parse::<i32>().unwrap());
            liste2.push(nombres[1].parse::<i32>().unwrap());
        }
    }
    let mut result = 0;
    for el in liste1 {
        let factor = el;
        for value in &liste2 {
            if *value == el {
                result += factor;
            }
        }
    }
    println!("{}", result);
    Ok(())
}
