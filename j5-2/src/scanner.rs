#[derive(Debug)]
pub struct Rule {
    pub before: i32,
    pub after: i32,
}

impl Rule {
    pub fn from_line(str: &str) -> Self {
        let array: Vec<i32> = str
            .split("|")
            .map(|el| el.parse::<i32>().unwrap())
            .collect();
        Self {
            before: array[0],
            after: array[1],
        }
    }
    pub fn from_str(str: &str) -> Vec<Self> {
        let mut values: Vec<Self> = Vec::new();
        for line in str.lines() {
            values.push(Self::from_line(line));
        }
        values
    }
}
