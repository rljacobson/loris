/*!
  The abstract interfaces for expressions are defined here. For the concrete atom
  of the same name, see `crate::atom::expression`.
*/

use std::{fmt::Display, rc::Rc};

use crate::{string::{FormattingParameters, StringForms}, IsEqual};

// Wrapper types for types defined below.
// Todo: Study replacing Rc with COW.
pub type RcEx = Rc<dyn Ex>;
pub type RcExpressionInterface = Rc<dyn ExpressionInterface>;


/// Strings and Symbols are special cases for formatting purposes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionKind {
  String,
  Symbol,
  OtherExpression,
  Other
}

pub trait Ex {
  fn string_form(&self, params: &FormattingParameters) -> String;
  fn is_equal(&self, other: &dyn Ex) -> IsEqual;
  fn deep_copy(&self) -> RcEx;
  fn copy(&self) -> RcEx;
  fn needs_eval(&self) -> bool;
  fn hash(&self) -> u64;

  /// A cheat to determine if this Ex is a String or Symbol or 'other'.
  fn kind(&self) -> ExpressionKind;
}


impl Display for Ex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_form(&FormattingParameters::default()))
    }
}

pub trait ExpressionInterface: Ex {
  fn get_parts(&self) -> &Vec<RcEx>;
  fn get_part(&self, i: usize) -> RcEx;
  fn set_parts(&self, new_parts: &Vec<RcEx>);
  fn clear_hashes(&self);

  fn len(&self) -> usize;
  fn less(&self, i: i32, j: i32) -> bool;
  fn swap(&self, i: i32, j: i32);
  fn append_ex(&self, e: RcEx);
  fn append_ex_array(&self, e: &[RcEx]);
  fn head_str(&self) -> String;

  // fn is_symbol(&self) -> bool;
  // From "StringInterface"
  // fn get_value(&self) -> String;
}


// Since `String` is not defined in any internal module, our only option is to put the `impl Ex` in the module that
// defines `Ex`.
impl Ex for String {
    fn string_form(&self, params: &FormattingParameters) -> String {
        match params.form {
          StringForms::Output
          | StringForms::Traditional
          | StringForms::Standard => {
            self.clone()
          },

          _ => format!("\"{}\"", self)
        }
    }

    fn is_equal(&self, other: &dyn Ex) -> IsEqual {
        match other.kind() {
            ExpressionKind::String => {
              (self.string_form(&FormattingParameters::standard()) == other.string_form(&FormattingParameters::standard())).into()
            },
            _ => IsEqual::False,
        }
    }

    fn deep_copy(&self) -> RcEx {
        self.copy()
    }

    fn copy(&self) -> RcEx {
        Rc::new(self.clone())
    }

    fn needs_eval(&self) -> bool {
        todo!()
    }

    fn hash(&self) -> u64 {
        todo!()
    }

    fn kind(&self) -> ExpressionKind {
        ExpressionKind::String
    }
}
