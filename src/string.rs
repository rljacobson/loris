/*!
  Traits and structs for interfacing with and converting to strings.

*/

use std::rc::Rc;

use strum::EnumString;

use crate::{
  atoms::expression::Expression,
  expression::ExpressionInterface,
  evaluation::EvalState
};

use super::Def;

pub trait EvalStateForStringer {
  fn get_string_fn(&self, head_str: &str) -> Option<ToStringFnType>;
  // Used by Definition[]
  fn get_defined(&self, name: &str) -> Option<&Def>;
}

pub(crate) type ToStringFnType = fn(&dyn ExpressionInterface, FormattingParameters) -> Option<String>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString)]
pub enum StringForms {
  #[strum(serialize="System`InputForm")]
  InputForm,
  #[strum(serialize="System`FullForm")]
  FullForm,
  #[strum(serialize="System`TraditionalForm")]
  TraditionalForm,
  #[strum(serialize="System`TeXForm")]
  TeXForm,
  #[strum(serialize="System`StandardForm")]
  StandardForm,
  #[strum(serialize="System`OutputForm")]
  OutputForm
}

#[derive(Clone)]
/// Parameters used in methods that transform expressions into strings.
pub struct FormattingParameters {
  pub form            : StringForms,
  pub context         : String,
  pub context_path    : Rc<dyn ExpressionInterface>,
  pub inside_top_level: bool,
  pub esi             : Rc<dyn EvalStateForStringer>,
}

impl Default for FormattingParameters {
    fn default() -> Self {
      let context      = String::from("Global`");
      let context_path = Rc::new(
        Expression {
          correctly_instantiated:  true,
          parts      : Vec:: new(),
          needs_eval : true,
          evaled_hash: 0,
          cached_hash: 0,
        }
      );

      Self {
        context,
        context_path,
        form: StringForms::InputForm,
        inside_top_level: false,
        esi: Rc::new(EvalState::new())
      }
    }
}
