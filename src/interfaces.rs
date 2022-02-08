/*!

  The abstract interfaces for expression representations. An expression is a
  variant of the enum `Expression`. Expression _representations_,
  `ExpressionRepresentation`s, are what the variants of `Expression` wrap.

*/

use std::fmt::Display;

use crate::{
  formatting::FormattingParameters,
  IsEqual,
  expression::Expression
};


// pub type RcExpressionRepresentation  = Rc<dyn ExpressionRepresentation>;
// pub type RcMExpressionRepresentation = Rc<dyn MExpressionRepresentation>;

pub trait ExpRep = ExpressionRepresentation;

/// Something is only an `Expression` if it is not an M-expression, i.e. an atom: strings, symbols, numbers and other literals…
pub trait ExpressionRepresentation {
  fn string_form(&self, params: &FormattingParameters) -> String;
  fn is_equal(&self, other: &Expression) -> IsEqual;
  fn deep_copy(&self) -> Expression;
  fn copy(&self) -> Expression;
  fn needs_eval(&self) -> bool;
  fn hash(&self) -> u64;
}

impl Display for dyn ExpressionRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_form(&FormattingParameters::default()))
    }
}

// impl PartialEq for dyn ExpressionRepresentation {
//     fn eq(&self, other: &dyn ExpressionRepresentation) -> bool {
//         self.is_equal(other.into())==IsEqual::True
//     }
// }

/*
// Since `String` is not defined in any internal module, our only option is to put the `impl Ex` in the module that
// defines `Ex`.
impl<String> ExpressionRepresentation<String> for String {
    fn string_form(&self, params: &FormattingParameters) -> String {
        match params.form {

          DisplayForm::Output
          | DisplayForm::Traditional
          | DisplayForm::Standard => {
            self.clone().0
          },

          _ => format!("\"{}\"", self)

        }
    }

    fn is_equal(&self, other: &String) -> IsEqual {
        (self == other).into()
    }

    fn deep_copy(&self) -> String {
        self.copy()
    }

    fn copy(&self) -> String {
        self.clone()
    }

    fn needs_eval(&self) -> bool {
        false
    }

    fn hash(&self) -> u64 {
        let mut hasher = FnvHasher::default();
        hasher.write(&[102u8, 206u8, 57u8, 172u8, 207u8, 100u8, 198u8, 133u8]);
        hasher.write(self.as_str().as_bytes());
        hasher.finish()
    }
}
*/
