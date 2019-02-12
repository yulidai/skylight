// CSS Data Struct

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector)
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color)
}

#[derive(Debug, Clone)]
enum Unit {
    Px,
}

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();

        return (a, b, c);
    }
}

// Parsing
pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser { pos: 0, input: source };
    Stylesheet { rules: parser.parse_rules() }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() { break }
            rules.push(self.parse_rule());
        }

        return rules;
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => { self.consume_char(); self.consume_whitespace(); },
                '{' => break,
                c => panic!("Unexpected character {:?} in selector list", c),
            }
        }

        return selectors;
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector { tag_name: None, id: None, class: Vec::new() };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identify());
                },
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identify())
                },
                '*' => {
                    self.consume_char();
                },
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identify());
                },
                _ => break
            }
        }

        return selector;
    }

    fn parse_identify(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }

        return declarations;
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.parse_identify();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ';');

        Declaration {
            name,
            value,
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'...'9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identify()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| match c {
            '0'...'9' | '.' => true,
            _ => false,
        });

        s.parse().unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identify().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit")
        }
    }

    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255
        })
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos .. self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    // 通用函数
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let ( _, crt_char ) = iter.next().unwrap();
        let ( next_pos, _ ) = iter.next().unwrap_or((1, ' '));

        self.pos += next_pos;

        crt_char
    }

    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' | '-' | '_' => true,
        _ => false,
    }
}