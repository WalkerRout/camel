use std::fmt;

/// Nodes in the Abstract Syntax Tree
///
/// Application: t1 t2
/// Abstraction: λx. t1
/// Identifier:  x
#[derive(Debug, PartialEq)]
pub enum Node<'inp> {
  Abstraction(Box<Abstraction<'inp>>),
  Application(Box<Application<'inp>>),
  Identifier(Identifier<'inp>),
}

/// An abstraction of a lambda function, containing a parameter and a body
#[derive(Debug, PartialEq)]
pub struct Abstraction<'inp> {
  pub param: &'inp str,
  pub body: Node<'inp>,
}

#[derive(Debug, PartialEq)]
pub struct Application<'inp> {
  pub lhs: Node<'inp>,
  pub rhs: Node<'inp>,
}

#[derive(Debug, PartialEq)]
pub struct Identifier<'inp> {
  pub name: &'inp str,
}

impl fmt::Display for Node<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Node::Abstraction(abs) => write!(f, "(λ{}. {})", abs.param, abs.body),
      Node::Application(app) => write!(f, "{} {}", app.lhs, app.rhs),
      Node::Identifier(id) => write!(f, "{}", id.name),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;

  #[rstest]
  #[case(
    Node::Application(Box::new(Application {
      lhs: Node::Abstraction(Box::new(Abstraction {
        param: "x",
        body: Node::Identifier(Identifier {
          name: "x",
        }),
      })),
      rhs: Node::Abstraction(Box::new(Abstraction {
        param: "y",
        body: Node::Identifier(Identifier {
          name: "y",
        }),
      })),
    })),
    "(λx. x) (λy. y)"
  )]
  fn simple_ast(#[case] ast: Node, #[case] expected_str: &str) {
    let expected_ast = Node::Application(Box::new(Application {
      lhs: Node::Abstraction(Box::new(Abstraction {
        param: "x",
        body: Node::Identifier(Identifier { name: "x" }),
      })),
      rhs: Node::Abstraction(Box::new(Abstraction {
        param: "y",
        body: Node::Identifier(Identifier { name: "y" }),
      })),
    }));

    assert_eq!(ast, expected_ast);
    assert_eq!(ast.to_string(), expected_str);
  }
}
