pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn from_string(characters: impl Into<String>) -> Self {
        Self {
            cursor: 0,
            characters: characters.into().chars().collect(),
        }
    }
    pub fn cursor(&self) -> usize {
        self.cursor
    }
    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }
    pub fn is_done(&self) -> bool {
        self.cursor >= self.characters.len()
    }
    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(value) => {
                self.cursor += 1;
                Some(value)
            }
            None => None,
        }
    }
    pub fn take(&mut self, target: &char) -> bool {
        if let Some(c) = self.peek() {
            if c == target {
                self.cursor += 1;
                return true;
            }
        }
        return false;
    }
    pub fn take_if(&mut self, predicate: fn(&char) -> bool) -> bool {
        if let Some(c) = self.peek() {
            if predicate(c) {
                return true;
            }
        }
        false
    }
    pub fn transform<T>(&mut self, cb: impl FnOnce(&char) -> Option<T>) -> Option<T> {
        match self.peek() {
            Some(value) => match cb(&value) {
                Some(value) => {
                    self.cursor += 1;
                    Some(value)
                }
                None => None,
            },
            None => None,
        }
    }
    pub fn skip_n(&mut self, n: usize) {
        self.cursor += n;
    }
    pub fn check_and_skip(&mut self, str: &str) -> bool {
        let content = &self.characters[self.cursor..];
        let char_array: Vec<char> = str.chars().collect();
        let char_slice: &[char] = &char_array;
        if content.starts_with(char_slice) {
            self.skip_n(str.len());
            return true;
        }
        false
    }

    pub fn skip(&mut self, target: &char) {
        while let Some(value) = self.peek() {
            if value == target {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }
    pub fn skip_white_space(&mut self) {
        self.skip(&' ');
    }
    pub fn take_strict(&mut self, target: &str) -> bool {
        let mut target = target.chars();
        while let Some(value) = target.next() {
            if let Some(c) = self.peek() {
                if value == *c {
                    self.cursor += 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn skip_to_next_string(&mut self, target: &str) {
        while let Some(c) = self.peek() {
            if *c == target.chars().next().unwrap() {
                if self.take_strict(target) {
                    break;
                }
            }
            self.cursor += 1;
        }
    }

    pub fn take_while(&mut self, mut predicate: impl FnMut(&char) -> bool) -> String {
        let mut result = String::new();
        while let Some(value) = self.peek() {
            if predicate(value) {
                result.push(*value);
                self.cursor += 1;
            } else {
                break;
            }
        }
        result
    }
    pub fn parse_number(&mut self) -> Option<f64> {
        let str =
            self.take_while(|c| c.is_digit(10) || *c == '.' || *c == '-' || c.is_alphabetic());
        if str.chars().any(|c| c.is_alphabetic()) {
            None
        } else {
            str.parse().ok()
        }
    }
}