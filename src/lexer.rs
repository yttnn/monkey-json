#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  String(String),
  Number(f64),
  Bool(bool),
  Null,
  WhiteSpace,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,
  Commma,
  Colon,
}

pub struct Lexer<'a> {
  chars : std::iter::Peekable<std::str::Chars<'a>>,
}

#[derive(Debug)]
pub struct LexerError {
  pub msg: String,
}

impl LexerError {
  fn new(msg: &str) -> LexerError {
    LexerError {
      msg: msg.to_string(),
    }
  }
}

impl<'a> Lexer<'a> {
  pub fn new(input: &str) -> Lexer {
    Lexer {
      chars: input.chars().peekable(),
    }
  }

  // 文字列を分割
  pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
    let mut tokens = vec![];
    while let Some(token) = self.next_token()? {
      match token {
        Token::WhiteSpace => {}
        _ => {
          tokens.push(token);
        }
      }
    }
    Ok(tokens)
  }
}

fn next_return_token(&mut self, token: Token) -> Option<Token> {
  self.chars.next();
  Some(token)
}

fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
  unimplemented!()
}

fn parse_null_token(&mut self) -> Result<Option<Token>, LexerError> {
  unimplemented!()
}

fn parse_bool_token(&mut self, b: bool) -> Result<Option<Token>, LexerError> {
  unimplemented!()
}

fn parse_number_token(&mut self) -> Result<Option<Token>, LexerError> {
  unimplemented!()
}

fn parse_string_token(&mut self) -> Result<Option<Token>, LexerError> {
  unimplemented!()
}

fn push_utf16(result: &mut String, utf16: &mut Vec<u16>) -> Result<(), LexerError> {
  unimplemented!()
}