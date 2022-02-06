use std::hash::Hasher;

use fnv::FnvHasher;

use crate::{
  expression::{
    Expression
  },
  formatting::{
    FormattingParameters,
    DisplayForm
  },
  IsEqual,
  interfaces::{
    ExpressionRepresentation
  }
};

/// Wrapper type for strings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringExpression(String);


// Since `String` is not defined in any internal module, our only option is to put the `impl Ex` in the module that
// defines `Ex`.
impl ExpressionRepresentation for StringExpression {

  fn string_form(&self, params: &FormattingParameters) -> String {
    match params.form {

      DisplayForm::Output
      | DisplayForm::Traditional
      | DisplayForm::Standard => {
        self.0.clone()
      },

      _ => format!("\"{}\"", self.0)

    }
  }

  fn is_equal(&self, other: &Expression) -> IsEqual {
    match other {

      Expression::StringExpression(other_string_expression) => {
        (self.0 == other_string_expression.0).into()
      },

      _ => IsEqual::False,
    }
  }

  fn deep_copy(&self) -> Expression {
    self.copy()
  }

  fn copy(&self) -> Expression {
    Expression::StringExpression(self.clone())
  }

  fn needs_eval(&self) -> bool {
    false
  }

  fn hash(&self) -> u64 {
    let mut hasher = FnvHasher::default();
    hasher.write(&[102u8, 206u8, 57u8, 172u8, 207u8, 100u8, 198u8, 133u8]);
    hasher.write(self.0.as_str().as_bytes());
    hasher.finish()
  }


}
