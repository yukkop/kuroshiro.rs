use kuromoji::Tokenizer;

pub struct Kuroshiro;

impl Kuroshiro {
    pub fn output_ruby(text: &str) -> String {
        let tokens = parse(text);
        let ruby_tokens = tokens
            .as_slice()
            .iter()
            .map(|token| token.clone().to_ruby())
            .collect::<Vec<String>>();
        let ruby_output = ruby_tokens.join("");
        ruby_output
    }
}

#[derive(Clone)]
struct Token {
    text: String,
    reading: String,
}

impl Token {
    fn new(text: &str, reading: &String) -> Token {
        Token {
            text: String::from(text),
            reading: reading.to_owned(),
        }
    }

    fn alphabet(&self) -> Alphabet {
        match to_char(self.text.as_str()) {
            '\u{4E00}'..='\u{9FCF}' => Alphabet::Kanji,
            '\u{F900}'..='\u{FAFF}' => Alphabet::Kanji,
            '\u{3400}'..='\u{4DBF}' => Alphabet::Kanji,
            '\u{3040}'..='\u{309F}' => Alphabet::Hiragana,
            '\u{30A0}'..='\u{30FF}' => Alphabet::Katakana,
            _ => Alphabet::Other,
        }
    }

    fn to_ruby(&self) -> String {
        match self.alphabet() {
            Alphabet::Kanji => format!("<ruby>{}<rt>{}</rt></ruby>", self.text, self.reading),
            _ => format!("{}", self.text),
        }
    }
}

#[derive(Debug)]
enum Alphabet {
    Kanji,
    Hiragana,
    Katakana,
    Other,
}

fn to_char(text: &str) -> char {
    text.chars().next().unwrap()
}

fn parse(text: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::normal();
    let tokens = tokenizer.tokenize(text);
    let parsed_tokens = tokens
        .as_slice()
        .iter()
        .map(|token| Token::new(token.clone().text, &token.clone().detail.reading))
        .collect::<Vec<Token>>();
    parsed_tokens
}