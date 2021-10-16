pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap();
    }
    fn starts_wth(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
    // Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }
    fn consume_while<F>(&mut self, test:F)->String
      Where F: Fn(char)->bool{
        let mut result = String::new();
        while !self.eof() && test(self.next_char()){
          result.push(self.consume_char());
        }
        result
      }
}
