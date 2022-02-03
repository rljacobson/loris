/*!

  The abstract interfaces for expressions. Note the naming convention: the trait
  is called `WidgetInterface`, while the reference counted copy-on-write
  pointer to a `WidgetInterface` is a `Widget`. For copy-on-write operations,
  be sure to use `widget.make_mut()` rather than `widget.get_mut()`.

*/

use std::{
  hash::Hasher,
  fmt::Display,
  rc::Rc
};

use crate::{
  formatting::{
    FormattingParameters,
    DisplayForm
  },
  IsEqual,
  data_structures::hash::{
    FnvHasher
  }
};

/// Strings and Symbols are special cases for formatting purposes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionKind {
  String,
  Symbol,
  OtherExpression,
  Other
}

pub type Expression  = Rc<dyn ExpressionInterface>;
pub type MExpression = Rc<dyn MExpressionInterface>;

/// Something is only an `Expression` if it is not an M-expression, i.e. an atom: strings, symbols, numbers and other literals…
pub trait ExpressionInterface {
  fn string_form(&self, params: &FormattingParameters) -> String;
  fn is_equal(&self, other: &dyn ExpressionInterface) -> IsEqual;
  fn deep_copy(&self) -> Expression;
  fn copy(&self) -> Expression;
  fn needs_eval(&self) -> bool;
  fn hash(&self) -> u64;
  fn kind(&self) -> ExpressionKind;
}

impl Display for dyn ExpressionInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_form(&FormattingParameters::default()))
    }
}

impl PartialEq for dyn ExpressionInterface {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)==IsEqual::True
    }
}

/// An `MExpression` is an expression of the form `head[exp1, exp2, …]`. It is any expression that has parts.
pub trait MExpressionInterface: ExpressionInterface {
  fn get_parts(&self) -> &Vec<Expression>;
  fn get_part(&self, i: usize) -> Expression;
  fn set_parts(&self, new_parts: &Vec<Expression>);
  fn clear_hashes(&self);

  fn len(&self) -> usize;
  fn less(&self, i: i32, j: i32) -> bool;
  fn swap(&self, i: i32, j: i32);
  fn append_ex(&self, e: &dyn ExpressionInterface);
  fn append_ex_array(&self, e: &[Expression]);
  fn head_str(&self) -> String;
}


// Since `String` is not defined in any internal module, our only option is to put the `impl Ex` in the module that
// defines `Ex`.
impl ExpressionInterface for String {
    fn string_form(&self, params: &FormattingParameters) -> String {
        match params.form {

          DisplayForm::Output
          | DisplayForm::Traditional
          | DisplayForm::Standard => {
            self.clone()
          },

          _ => format!("\"{}\"", self)

        }
    }

    fn is_equal(&self, other: &dyn ExpressionInterface) -> IsEqual {
        match other.kind() {
            ExpressionKind::String => {
              let params = FormattingParameters::standard();
              (self.string_form(&params) == other.string_form(&params)).into()
            },
            _ => IsEqual::False,
        }
    }

    fn deep_copy(&self) -> Expression {
        self.copy()
    }

    fn copy(&self) -> Expression {
        Rc::new(self.clone())
    }

    fn needs_eval(&self) -> bool {
        false
    }

    fn hash(&self) -> u64 {
        let mut hasher = FnvHasher::default();
        hasher.write(self.as_str().as_bytes());
        hasher.finish()
    }

    fn kind(&self) -> ExpressionKind {
        ExpressionKind::String
    }

}
