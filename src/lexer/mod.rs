#[cfg(test)]
mod tests {
    #[test]
    fn test_next_token() {
        assert_eq!(1 + 1, 2)
    }
}

pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: u8,
}

impl Lexer {
    fn new(&mut self) {
        self.input = String::from("");
        self.position = 0;
        self.read_position = 1;
        self.ch = 0;
    }
}
