use unicode_general_category::{get_general_category, GeneralCategory};

#[derive(PartialEq, Debug, Clone)]
pub enum IntLit {
    DecimalLit(String),
    SiLit(String, String),
    OctalLit(String),
    BinaryLit(String),
    HexLit(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum FloatLit {
    WithExp(String, String),
    WithoutExp(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Null,
    BConst(bool),
    Package,
    Import,
    For,
    In,
    If,
    Let,
    Add,
    Sub,
    Mul,
    Div,
    BolAnd,
    BolOr,
    And,
    Or,
    Eq,
    NotEq,
    PatternEq,
    PatternNotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Assign,
    Colon,
    Question,
    Exlamation,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Bottom,
    Top,
    Ellipsis,
    Comma,
    Dot,
    IntLit(IntLit),
    FloatLit(FloatLit),
    StringLit(String),
    ByteLit(Vec<u8>),
}

fn is_newline(c: char) -> bool {
    c == '\n'
}

fn is_unicode_char(c: char) -> bool {
    !is_newline(c)
}

fn is_unicode_letter(c: char) -> bool {
    get_general_category(c) == GeneralCategory::UppercaseLetter
        || get_general_category(c) == GeneralCategory::LowercaseLetter
        || get_general_category(c) == GeneralCategory::TitlecaseLetter
        || get_general_category(c) == GeneralCategory::ModifierLetter
        || get_general_category(c) == GeneralCategory::OtherLetter
}

fn is_unicode_digit(c: char) -> bool {
    get_general_category(c) == GeneralCategory::DecimalNumber
}

fn is_letter(c: char) -> bool {
    is_unicode_letter(c) || c == '_' || c == '$'
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_binary_digit(c: char) -> bool {
    c == '0' || c == '1'
}

fn is_octal_digit(c: char) -> bool {
    c.is_ascii_digit() && c != '8' && c != '9'
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn is_multiplier(c: char) -> bool {
    c == 'K' || c == 'M' || c == 'G' || c == 'T' || c == 'P'
}

pub struct Lexer {
    input: Vec<char>,
    current_position: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Self {
            input: input.chars().collect(),
            current_position: 0,
            tokens: Vec::new(),
        }
    }

    fn get(&self, i: usize) -> Option<char> {
        self.input.get(i).cloned()
    }

    fn getstr(&self, i: usize, j: usize) -> String {
        self.input[i..j].iter().collect::<String>()
    }

    fn exhausted(&self) -> bool {
        self.check_pos(self.current_position)
    }

    fn check_pos(&self, i: usize) -> bool {
        i >= self.input.len()
    }

    fn verify(&self, i: usize, c: char) -> bool {
        !self.check_pos(i) && self.get(i).unwrap() == c
    }

    fn verify_predicat(&self, i: usize, f: fn(char) -> bool) -> bool {
        !self.check_pos(i) && f(self.get(i).unwrap())
    }

    fn lex_identifier(&mut self, mut i: usize) {
        while self.verify_predicat(i, is_letter) || self.verify_predicat(i, is_unicode_digit) {
            i += 1;
        }
        let binding = self.getstr(self.current_position, i);
        match binding.as_str() {
            "null" => self.tokens.push(Token::Null),
            "true" => self.tokens.push(Token::BConst(true)),
            "false" => self.tokens.push(Token::BConst(false)),
            "package" => self.tokens.push(Token::Package),
            "import" => self.tokens.push(Token::Import),
            "for" => self.tokens.push(Token::For),
            "in" => self.tokens.push(Token::In),
            "if" => self.tokens.push(Token::If),
            "let" => self.tokens.push(Token::Let),
            keyword if keyword.starts_with("__") => {
                self.tokens.push(Token::Keyword(keyword.to_string()))
            }
            ident => self.tokens.push(Token::Identifier(ident.to_string())),
        };
        self.current_position = i - 1;
    }

    fn lex_binary(&mut self, mut i: usize) {
        loop {
            if self.verify_predicat(i, is_binary_digit) {
                i += 1;
            } else if self.verify(i, '_') && self.verify_predicat(i + 1, is_binary_digit) {
                i += 2;
            } else {
                break;
            }
        }
        let binding = self.getstr(self.current_position, i);
        self.tokens.push(Token::IntLit(IntLit::BinaryLit(binding)));
        self.current_position = i - 1;
    }

    fn lex_octal(&mut self, mut i: usize) {
        loop {
            if self.verify_predicat(i, is_octal_digit) {
                i += 1;
            } else if self.verify(i, '_') && self.verify_predicat(i + 1, is_octal_digit) {
                i += 2;
            } else {
                break;
            }
        }
        let binding = self.getstr(self.current_position, i);
        self.tokens.push(Token::IntLit(IntLit::OctalLit(binding)));
        self.current_position = i - 1;
    }

    fn lex_hex(&mut self, mut i: usize) {
        loop {
            if self.verify_predicat(i, is_hex_digit) {
                i += 1;
            } else if self.verify(i, '_') && self.verify_predicat(i + 1, is_hex_digit) {
                i += 2;
            } else {
                break;
            }
        }
        let binding = self.getstr(self.current_position, i);
        self.tokens.push(Token::IntLit(IntLit::HexLit(binding)));
        self.current_position = i - 1;
    }

    fn lex_decimal(&mut self, mut i: usize) {
        loop {
            if self.verify_predicat(i, is_digit) {
                i += 1;
            } else if self.verify(i, '_') && self.verify_predicat(i + 1, is_digit) {
                i += 2;
            } else if self.verify(i, '.') {
                self.lex_point_decimal(i + 1);
                return;
            } else if self.verify(i, 'e') || self.verify(i, 'E') {
                self.lex_float(i + 1);
                return;
            } else if self.verify_predicat(i, is_multiplier) {
                self.lex_si(i + 1);
                return;
            } else {
                break;
            }
        }
        let binding = self.getstr(self.current_position, i);
        match binding.as_str() {
            "0" => self.tokens.push(Token::IntLit(IntLit::DecimalLit(binding))),
            _ if self.verify_predicat(self.current_position, |x| x != '0') => {
                self.tokens.push(Token::IntLit(IntLit::DecimalLit(binding)))
            }
            _ => panic!(
                "Decimals may not start with '0': {} (pos: {}-{})",
                binding, self.current_position, i
            ),
        }
        self.current_position = i - 1;
    }

    fn lex_point_decimal(&mut self, mut i: usize) {
        loop {
            if self.verify_predicat(i, is_digit) && self.verify_predicat(i + 1, is_multiplier) {
                self.lex_si(i + 2);
                return;
            } else if self.verify_predicat(i, is_digit)
                && self.verify(i + 1, '_')
                && self.verify_predicat(i + 2, is_digit)
            {
                i += 2;
            } else if self.verify(i, 'e') || self.verify(i, 'E') {
                self.lex_float(i + 1);
                return;
            } else if self.verify_predicat(i, is_digit) {
                i += 1;
            } else {
                break;
            }
        }
        let binding = self.getstr(self.current_position, i);
        self.tokens
            .push(Token::FloatLit(FloatLit::WithoutExp(binding)));
        self.current_position = i - 1;
    }

    fn lex_si(&mut self, mut i: usize) {
        let binding = self.getstr(self.current_position, i - 1);
        let multiplier;
        if self.verify(i, 'i') {
            multiplier = self.getstr(i - 1, i + 1);
            i += 1;
        } else {
            multiplier = self.getstr(i - 1, i);
        }
        self.tokens
            .push(Token::IntLit(IntLit::SiLit(binding, multiplier)));
        self.current_position = i - 1;
    }

    fn lex_float(&mut self, mut i: usize) {
        let mut exp_start = i - 1;
        if self.verify(i, '+') || self.verify(i, '-') {
            i += 1;
        }

        loop {
            if self.verify_predicat(i, is_digit) {
                i += 1;
            } else if self.verify(i, '_') && self.verify_predicat(i + 1, is_digit) {
                i += 2;
            } else {
                break;
            }
        }
        let exp = self.getstr(exp_start, i);
        let binding = self.getstr(self.current_position, exp_start);
        self.tokens
            .push(Token::FloatLit(FloatLit::WithExp(binding, exp)));
        self.current_position = i - 1;
    }

    fn lexer(&mut self) {
        while !self.exhausted() {
            let i = self.current_position;

            match self.get(i).unwrap() {
                '0' if self.verify(i + 1, 'b') => self.lex_binary(i + 2),
                '0' if self.verify(i + 1, 'o') => self.lex_octal(i + 2),
                '0' if self.verify(i + 1, 'x') || self.verify(i + 1, 'X') => self.lex_hex(i + 2),
                c if is_digit(c) => self.lex_decimal(i + 1),
                '.' if self.verify_predicat(i + 1, is_digit) => self.lex_point_decimal(i + 2),
                '+' => self.tokens.push(Token::Add),
                '-' => self.tokens.push(Token::Sub),
                '*' => self.tokens.push(Token::Mul),
                '/' => self.tokens.push(Token::Div),
                '&' if self.verify(i + 1, '&') => {
                    self.tokens.push(Token::BolAnd);
                    self.current_position += 1;
                }
                '&' => self.tokens.push(Token::And),
                '|' if self.verify(i + 1, '|') => {
                    self.tokens.push(Token::BolOr);
                    self.current_position += 1;
                }
                '|' => self.tokens.push(Token::Or),
                '=' if self.verify(i + 1, '=') => {
                    self.tokens.push(Token::Eq);
                    self.current_position += 1;
                }
                '=' if self.verify(i + 1, '~') => {
                    self.tokens.push(Token::PatternEq);
                    self.current_position += 1;
                }
                '=' => self.tokens.push(Token::Assign),
                '!' if self.verify(i + 1, '=') => {
                    self.tokens.push(Token::NotEq);
                    self.current_position += 1;
                }
                '!' if self.verify(i + 1, '~') => {
                    self.tokens.push(Token::PatternNotEq);
                    self.current_position += 1;
                }
                '!' => self.tokens.push(Token::Exlamation),
                '<' if self.verify(i + 1, '=') => {
                    self.tokens.push(Token::LessEq);
                    self.current_position += 1;
                }
                '<' => self.tokens.push(Token::Less),
                '>' if self.verify(i + 1, '=') => {
                    self.tokens.push(Token::GreaterEq);
                    self.current_position += 1;
                }
                '>' => self.tokens.push(Token::Greater),
                ':' => self.tokens.push(Token::Colon),
                '?' => self.tokens.push(Token::Question),
                '(' => self.tokens.push(Token::LParen),
                ')' => self.tokens.push(Token::RParen),
                '{' => self.tokens.push(Token::LBrace),
                '}' => self.tokens.push(Token::RBrace),
                '[' => self.tokens.push(Token::LBracket),
                ']' => self.tokens.push(Token::RBracket),
                ',' => self.tokens.push(Token::Comma),
                '.' if self.verify(i + 1, '.') && self.verify(i + 2, '.') => {
                    self.tokens.push(Token::Ellipsis);
                    self.current_position += 2;
                }
                '.' => self.tokens.push(Token::Dot),
                '_' if self.verify(i + 1, '|') && self.verify(i + 2, '_') => {
                    self.tokens.push(Token::Bottom);
                    self.current_position += 2;
                }
                '#' if self.verify_predicat(i + 1, is_letter) => self.lex_identifier(i + 2),
                '_' if self.verify(i + 1, '#') && self.verify_predicat(i + 2, is_letter) => {
                    self.lex_identifier(i + 3)
                }
                c if is_letter(c) => self.lex_identifier(i + 1),
                c if c.is_whitespace() => (),
                c => panic!(
                    "Unexpected character: {} (pos: {})",
                    c, self.current_position
                ),
            }

            self.current_position += 1;
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        self.lexer();
        // return copy of tokens
        self.tokens.clone()
    }
}
