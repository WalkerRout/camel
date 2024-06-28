use std::iter::Peekable;
use std::str::Chars;

use crate::token::{Token, TokenKind};

pub struct Lexer<'inp> {
  chars: Peekable<Chars<'inp>>,
  buffer: &'inp str,
  pos: usize,
  start: usize,
}

impl<'inp> Lexer<'inp> {
  pub fn new(input: &'inp str) -> Self {
    Lexer {
      chars: input.chars().peekable(),
      buffer: input,
      pos: 0,
      start: 0,
    }
  }

  fn advance(&mut self) -> Option<char> {
    let next_char = self.chars.next()?;
    self.pos += next_char.len_utf8();
    Some(next_char)
  }

  fn peek(&mut self) -> Option<&char> {
    self.chars.peek()
  }

  pub fn next_token(&mut self) -> Option<Token<'inp>> {
    self.skip_whitespace();
    self.start = self.pos;
    if let Some(&c) = self.peek() {
      let tok = match c {
        '(' => self.create_token(TokenKind::LeftParen),
        ')' => self.create_token(TokenKind::RightParen),
        'λ' | '\\' => self.create_token(TokenKind::Lambda),
        '.' => self.create_token(TokenKind::Dot),
        'a'..='z' => self.read_lcid(), // could add additional logic inside here to return an Unknown
        _ => self.create_token(TokenKind::Unknown),
      };
      Some(tok)
    } else {
      None
    }
  }

  fn create_token(&mut self, kind: TokenKind) -> Token<'inp> {
    self.advance();
    Token {
      kind,
      text: &self.buffer[self.start..self.pos],
    }
  }

  fn skip_whitespace(&mut self) {
    while let Some(&c) = self.peek() {
      if c.is_whitespace() {
        self.advance();
      } else {
        break;
      }
    }
  }

  fn read_lcid(&mut self) -> Token<'inp> {
    while let Some(&c) = self.peek() {
      if c.is_ascii_alphanumeric() {
        self.advance();
      } else {
        break;
      }
    }
    Token {
      kind: TokenKind::LowercaseId,
      text: &self.buffer[self.start..self.pos],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  #[rstest]
  #[case("(", Some(Token { kind: TokenKind::LeftParen, text: "(" }))]
  #[case(")", Some(Token { kind: TokenKind::RightParen, text: ")" }))]
  #[case("λ", Some(Token { kind: TokenKind::Lambda, text: "λ" }))]
  #[case("\\", Some(Token { kind: TokenKind::Lambda, text: "\\" }))]
  #[case(".", Some(Token { kind: TokenKind::Dot, text: "." }))]
  #[case("x", Some(Token { kind: TokenKind::LowercaseId, text: "x" }))]
  #[case("xyz", Some(Token { kind: TokenKind::LowercaseId, text: "xyz" }))]
  #[case("  (", Some(Token { kind: TokenKind::LeftParen, text: "(" }))]
  #[case("", None)]
  fn next_token(#[case] input: &str, #[case] expected_token: Option<Token>) {
    let mut lexer = Lexer::new(input);
    let token = lexer.next_token();
    assert_eq!(token, expected_token);
  }

  #[rstest]
  #[case("(λx.x)", vec![
    Token { kind: TokenKind::LeftParen, text: "(" },
    Token { kind: TokenKind::Lambda, text: "λ" },
    Token { kind: TokenKind::LowercaseId, text: "x" },
    Token { kind: TokenKind::Dot, text: "." },
    Token { kind: TokenKind::LowercaseId, text: "x" },
    Token { kind: TokenKind::RightParen, text: ")" }
  ])]
  #[case("\\x.x", vec![
    Token { kind: TokenKind::Lambda, text: "\\" },
    Token { kind: TokenKind::LowercaseId, text: "x" },
    Token { kind: TokenKind::Dot, text: "." },
    Token { kind: TokenKind::LowercaseId, text: "x" }
  ])]
  fn tokenize_all(#[case] input: &str, #[case] expected_tokens: Vec<Token>) {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
      tokens.push(token);
    }
    assert_eq!(tokens, expected_tokens);
  }
}
