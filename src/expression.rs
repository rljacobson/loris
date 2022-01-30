/*!
  The abstract interfaces for expressions are defined here. For the concrete atom
  of the same name, see `crate::atom::expression`.
*/

use crate::string::FormattingParameters;

pub trait Ex {
  fn string_form(&self, params: &FormattingParameters) -> String;
  fn is_equal(&self, b: &dyn Ex) -> String;
  fn deep_copy(&self) -> Box<dyn Ex>;
  fn copy(&self) -> Box<dyn Ex>;
  fn needs_eval(&self) -> bool;
  fn hash(&self) -> u64;

  // Attempt to assert that this Ex is a String and get its value.
  fn string_value(&self) -> Option<String>;
}


pub trait ExpressionInterface: Ex {
  fn get_parts(&self) -> Vec<&dyn Ex>;
  fn get_part(&self, i: usize) -> &dyn Ex;
  fn set_parts(&self, new_parts: &Vec<&dyn Ex>);
  fn clear_hashes(&self);

  fn len(&self) -> usize;
  fn less(&self, i: i32, j: i32) -> bool;
  fn swap(&self, i: i32, j: i32);
  fn append_ex(&self, e: &dyn Ex);
  fn append_ex_array(&self, e: &[&dyn Ex]);
  fn head_str(&self) -> String;

  // fn is_symbol(&self) -> bool;
  // From "StringInterface"
  fn get_value(&self) -> String;
}
