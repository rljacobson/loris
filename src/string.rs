/*!
Traits and structs for interfacing with and converting to strings. For the concrete atom string, see `atom/string.rs`.

*/

use std::rc::Rc;

use strum::EnumString;

use crate::{
  atoms::{expression::Expression, symbol::Symbol},
  evaluation::EvalState,
  expression::{ExpressionInterface, RcExpressionInterface},
};

use super::Def;

pub(crate) type ToStringFnType =
fn(&dyn ExpressionInterface, FormattingParameters) -> Option<String>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString)]
pub enum StringForms {
  #[strum(serialize = "System`InputForm")]
  Input,
  #[strum(serialize = "System`FullForm")]
  Full,
  #[strum(serialize = "System`TraditionalForm")]
  Traditional,
  #[strum(serialize = "System`TeXForm")]
  TeX,
  #[strum(serialize = "System`StandardForm")]
  Standard,
  #[strum(serialize = "System`OutputForm")]
  Output,
}

#[derive(Clone)]
/// Parameters used in methods that transform expressions into strings.
pub struct FormattingParameters {
  pub form            : StringForms,
  pub context         : String,
  pub context_path    : RcExpressionInterface,
  pub inside_top_level: bool,
  pub esi             : Rc<EvalStateForStringer>,
}

impl FormattingParameters {
  pub fn standard() -> FormattingParameters {
    let context      = String::from("Global`");
    let context_path = Rc::new(Expression {
      parts: vec![Symbol::new_rc(String::from("System`List"))],
      correctly_instantiated: true,
      needs_eval            : true,
      evaled_hash           : 0,
      cached_hash           : 0,
    });

    Self {
      context,
      context_path,
      form: StringForms::Input,
      inside_top_level: false,
      esi: Rc::new(EvalState::new()),
    }
  }
}

impl Default for FormattingParameters {
  fn default() -> Self {
    let context      = String::from("Global`");
    let context_path = Rc::new(Expression {
      parts      : Vec:: new(),
      correctly_instantiated: true,
      needs_eval : true,
      evaled_hash: 0,
      cached_hash: 0,
    });

    Self {
      context,
      context_path,
      form: StringForms::Input,
      inside_top_level: false,
      esi : Rc::new(EvalState::new()),
    }
  }
}
