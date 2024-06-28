#[derive(Debug, Clone, PartialEq)]
pub struct Token<'inp> {
  pub kind: TokenKind,
  pub text: &'inp str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TokenKind {
  LeftParen,
  RightParen,
  Lambda,
  Dot,
  LowercaseId,
  Unknown,
}

#[derive(Debug, PartialEq)]
pub struct TokenError {
  pub kind: TokenKind,
  pub text: String,
}

impl From<Token<'_>> for TokenError {
  fn from(token: Token) -> Self {
    TokenError {
      kind: token.kind,
      text: token.text.to_string(),
    }
  }
}
