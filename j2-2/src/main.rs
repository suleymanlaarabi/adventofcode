use std::{fs, io::Result};

enum Status {
    Increase,
    Diminue,
    Error,
}

fn check_status(values: &Vec<f64>) -> (Status, f64) {
    let v1 = values.get(0).unwrap();
    let v2 = values.get(1).unwrap();

    if v1 == v2 {
        return (Status::Error, *v1);
    }
    if v2 > v1 {
        return (Status::Increase, *v1);
    }
    if v2 < v1 {
        return (Status::Diminue, *v1);
    }
    return (Status::Error, *v1);
}

fn check_line(values: &Vec<f64>) -> bool {
    let (status, mut last) = check_status(&values);
    for number in values.iter().skip(1) {
        let number = number.clone();
        match status {
            Status::Increase => {
                if number > last && number - last <= 3. {
                    last = number;
                    continue;
                }
                return false;
            }
            Status::Diminue => {
                if number < last && last - number <= 3. {
                    last = number;
                    continue;
                }
                return false;
            }
            Status::Error => {
                return false;
            }
        }
    }
    true
}

fn main() -> Result<()> {
    let content = fs::read_to_string("input.txt")?;
    let lignes: Vec<&str> = content.lines().collect();

    let mut safe = 0;
    for line in lignes {
        let values: Vec<f64> = line
            .split(" ")
            .map(|number_str| number_str.parse::<f64>().unwrap())
            .collect();
        let len = values.len();
        for i in 0..len {
            let mut new = values.clone();
            new.remove(i);
            if check_line(&new) {
                safe += 1;
                break;
            }
        }
    }
    println!("{}", safe);
    Ok(())
}
