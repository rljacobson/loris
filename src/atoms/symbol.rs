use std::assert_matches::assert_matches;
use std::hash::Hasher;

use fnv::FnvHasher;

use crate::{interfaces::ExpressionRepresentation, formatting::FormattingParameters, IsEqual, expression::Expression};


// Todo: Intern strings.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Symbol(pub String);


impl ExpressionRepresentation for Symbol {
  fn string_form(&self, params: &FormattingParameters) -> String {
    self.0.clone()
  }

  // Though shalt not call `is_equal` with a variant different from `Self`.
  fn is_equal(&self, other: &Expression) -> IsEqual {
    assert_matches!(other, Expression::Symbol(_));

    if let Expression::Symbol(other) = other {
      (self.0 == other.0).into()
    } else {
      unreachable!()
    }
  }

  fn deep_copy(&self) -> Expression {
    Expression::Symbol(self.clone())
  }

  fn copy(&self) -> Expression {
    Expression::Symbol(self.clone())
  }

  fn needs_eval(&self) -> bool {
    false
  }

  fn hash(&self) -> u64 {
    let mut hasher = FnvHasher::default();

    hasher.write(&[107u8, 10u8, 247u8, 23u8, 33u8, 221u8, 163u8, 156u8]);
    hasher.write(self.0.as_str().as_bytes());

    hasher.finish()
  }
}
