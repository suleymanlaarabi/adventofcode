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
    liste1.sort();
    liste2.sort();
    let total: i32 = liste1
        .iter()
        .zip(liste2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    println!("{}", total);
    Ok(())
}
