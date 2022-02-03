

use crate::{
  expression::{MExpression, ExpressionInterface},
  attributes::Attributes
};


#[derive(Clone)]
pub struct DownValue {
  pub rule        : MExpression,
  pub specificity : i32
}

impl PartialEq for DownValue {
    fn eq(&self, other: &Self) -> bool {
        <dyn ExpressionInterface>::eq(&*self.rule, &*other.rule)
    }
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
