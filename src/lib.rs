/*!

  Defines the fundamental traits and structs used throughout the library.

*/

use attributes::Attributes;
use evaluation::EvalStateInterface;
use expression::{ExpressionInterface, Ex};
pub mod atoms;
pub mod log;
pub mod attributes;
pub mod data_structures;
pub mod evaluation;
pub mod expression;
pub mod string;


pub trait DefinitionMap {
  fn set(&self, key: &str, value: &Def);
  fn get(&self, key: &str) -> Option<&Def>;
  fn get_def(&self, key: &str) -> Option<&Def>;
  fn lock_key(&self, key: &str);
  fn unlock_key(&self, key: &str);
  fn keys(&self) -> Vec<&str>;
  fn copy_defs(&self) -> Box<dyn DefinitionMap>;
}

pub trait StreamManager {
  fn write_string(
    &self,
    stream_name : &str,
    stream_index: i64,
    to_write    : &str
  ) -> bool;
  fn as_expr(&self) -> Box<dyn Ex>;
}


pub struct DownValue {
  pub rule        : Box<dyn ExpressionInterface>,
  pub specificity : i32
}

pub type EvalFnType =
    fn(&dyn ExpressionInterface, &dyn EvalStateInterface) -> Box<dyn Ex>;

pub struct Def {
  pub downvalues  : Vec<DownValue>,
  pub attributes  : Attributes,
  pub default_expr: Box<dyn Ex>,

  // A function defined here will override downvalues.
  pub legacy_eval_fn: EvalFnType
}
