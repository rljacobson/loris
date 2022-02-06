

use crate::{
  attributes::Attributes,
  atoms::mexpression::MExpression
};


#[derive(Clone, PartialEq)]
pub struct DownValue {
  pub rule        : MExpression,
  pub specificity : i32
}

// pub type EvalFnType =
//     fn(&dyn ExpressionInterface, &dyn EvalStateInterface) -> RcEx;

#[derive(Clone, PartialEq)]
pub struct Definition {
  pub downvalues  : Vec<DownValue>,
  pub attributes  : Attributes,
  // pub default_expr: Expression,

  // A function defined here will override downvalues.
  // pub legacy_eval_fn: EvalFnType
}


pub trait DefinitionMap {
  fn set(&self, key: &str, value: &Definition);
  fn get(&self, key: &str) -> Option<&Definition>;
  fn get_def(&self, key: &str) -> Option<&Definition>;
  fn lock_key(&self, key: &str);
  fn unlock_key(&self, key: &str);
  fn keys(&self) -> Vec<&str>;
  fn copy_defs(&self) -> Box<dyn DefinitionMap>;
}
