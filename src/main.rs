use std::io::prelude::*;

#[derive(Debug)]
pub enum QuotesStates {
    LeftQuote,
    RightQuote,
    Ignore,
}

#[derive(Debug)]
pub struct CollectQuotes {
    pub buffer: String,
    pub saved: Vec<String>,
    pub state: QuotesStates,
}

impl CollectQuotes {
    pub fn is_left_quote(&self, item: &str) -> bool {
        let left_quotes = vec!["\"".to_string(), "“".to_string()];
        let mut result = false;

        for quotes in left_quotes {
            if quotes.eq(item) {
                result = true;
            }
        }

        result
    }

    pub fn is_right_quote(&self, item: &str) -> bool {
        let right_quotes = vec!["\"".to_string(), "”".to_string()];
        let mut result = false;

        for quotes in right_quotes {
            if quotes.eq(item) {
                result = true;
            }
        }

        result
    }

    pub fn new() -> Self {
        CollectQuotes {
            buffer: String::new(),
            saved: vec![],
            state: QuotesStates::Ignore,
        }
    }

    pub fn process(&mut self, item: &str) {
        println!("debug: item: {} -- {:?}", item, &self);

        match self.state {
            QuotesStates::Ignore => match self.is_left_quote(item) {
                true => {
                    self.state = QuotesStates::LeftQuote;
                    self.buffer.push_str(item);
                }
                false => {}
            },
            QuotesStates::LeftQuote => {
                match self.is_right_quote(item) {
                    true => {
                        self.state = QuotesStates::RightQuote;
                        self.buffer.push_str(item);

                        self.saved.push(self.buffer.clone());

                        // clear buffer
                        self.buffer = String::new();
                    }
                    false => {
                        self.buffer.push_str(item);
                    }
                }
            }
            QuotesStates::RightQuote => match self.is_left_quote(item) {
                true => {
                    self.state = QuotesStates::LeftQuote;
                    self.buffer.push_str(item);
                }
                false => {
                    self.state = QuotesStates::Ignore;
                }
            },
        }
    }
}

#[allow(dead_code)]
fn process_string(string: String) -> CollectQuotes {
    // Will process the string as bytes to handle utf8
    let string_as_bytes = string.as_bytes();

    // Initialize my state machine
    let mut dfa = CollectQuotes::new();

    // Temp storage for bytes that are not valid utf8 strings yet
    let mut temp = Vec::new();

    // Processing one byte at a time
    for byte in string_as_bytes.bytes() {
        let item = byte.unwrap();

        temp.push(item);

        match String::from_utf8(temp.clone()) {
            Ok(character) => {
                // Process that one character
                dfa.process(&character);

                // Clear the temp buffer
                temp.clear();
            }
            Err(_) => {
                println!("Bytes list is not a valid utf8 string yet: {:?}", temp);
            }
        }
    }

    return dfa;
}

fn main() {
    let test_string = "Marcus said, \"Yo! Have you eaten? \". I replied, \"Not yet. I am currently looking for food now. How about you?\"".to_string();

    let dfa = process_string(test_string);

    println!("DFA: {:?}", dfa);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_quote() {
        let dfa = CollectQuotes::new();

        assert_eq!(dfa.is_right_quote("\""), true);
        assert_eq!(dfa.is_right_quote("”"), true);
    }

    #[test]
    fn test_left_quote() {
        let dfa = CollectQuotes::new();
        let left_quotes = vec!["\"".to_string(), "“".to_string()];

        for item in left_quotes.iter() {
            assert_eq!(dfa.is_left_quote(item.as_str()), true);
        }
    }

    #[test]
    fn test_process_string_with_english() {
        let test_string = "Marcus said, \"Yo! Have you eaten? \". I replied, \"Not yet. I am currently looking for food now. How about you?\"".to_string();

        let dfa = process_string(test_string);

        assert_eq!("\"Yo! Have you eaten? \"".to_string(), dfa.saved[0]);
        assert_eq!(
            "\"Not yet. I am currently looking for food now. How about you?\"".to_string(),
            dfa.saved[1]
        );
    }

    #[test]
    fn test_process_string_with_chinese() {
        let test_string = "老杜问了我，“你吃了马？”。我回了， “没有阿。你呢？”。".to_string();

        let dfa = process_string(test_string);

        assert_eq!("“你吃了马？”".to_string(), dfa.saved[0]);
        assert_eq!("“没有阿。你呢？”".to_string(), dfa.saved[1]);
    }
}
