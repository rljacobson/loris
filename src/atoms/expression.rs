/*!

  The concrete atom expression. For the abstract expression interface, see
  `crate::expression`.

*/

use std::str::FromStr;

use crate::{expression::{Ex, ExpressionInterface}, log::LoggingInterface, string::{FormattingParameters, StringForms}, evaluation::EvalStateInterface};

use super::symbol::Symbol;


pub struct Expression<'e> {
	parts                 : Vec<&'e dyn Ex>,
	needs_eval            : bool,
	correctly_instantiated: bool,
	evaled_hash           : u64,
	cached_hash           : u64,
}

impl<'e> Expression<'e> {
  pub fn head_assertion(&self, head: &dyn ExpressionInterface, logger: &dyn LoggingInterface) -> bool {
    is_sameq(head, self.get_part(0))
  }
}

impl Ex for Expression<'_> {

  fn string_form(&self, params: &FormattingParameters) -> String {
    let head = self.get_part(0);
    let head_str = head.string_form(&params);

    /* Apparently never used?
    // If the head is a symbol with a `to_string_fn`.
    if head.is_symbol() {
      let head_str = head.string_value();

      if let Some(to_string_fn) = params.esi.get_string_fn(&head_str) {
        return to_string_fn(self, params);
      }
    }
    */

    // Check if the head is `System\`*Form`, which changes the string form.
    if self.get_parts().len() == 2 {
      match StringForms::from_str(head_str.as_str()) {

        Ok(form) => {
          let mut new_params = params.clone();
          new_params.form = form;
          return self.get_part(1).string_form(&mut new_params);
        }

        Err(_) => { /* pass */ }

      } // end match on `StringForms`
    } // end if two parts

    let sequence = self.get_parts()[1..]
                       .iter()
                       .map(|p| p.string_form(&params))
                       .collect::<Vec<_>>()
                       .join(", ");

    format!("{}[{}]", head_str, sequence)
  }

  fn is_equal(&self, b: &dyn Ex) -> String {
      todo!()
  }

  fn deep_copy(&self) -> Box<dyn Ex> {
      todo!()
  }

  fn copy(&self) -> Box<dyn Ex> {
      todo!()
  }

  fn needs_eval(&self) -> bool {
      todo!()
  }

  fn hash(&self) -> u64 {
      todo!()
  }

  fn string_value(&self) -> Option<String> {
      todo!()
  }
}

impl ExpressionInterface for Expression<'_>{
    fn get_parts(&self) -> Vec<&dyn Ex> {
        self.parts.clone()
    }

    fn get_part(&self, i: usize) -> &dyn Ex {
        self.parts[i]
    }

    fn set_parts(&self, new_parts: &Vec<&dyn Ex>) {
        todo!()
    }

    fn clear_hashes(&self) {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn less(&self, i: i32, j: i32) -> bool {
        todo!()
    }

    fn swap(&self, i: i32, j: i32) {
        todo!()
    }

    fn append_ex(&self, e: &dyn Ex) {
        todo!()
    }

    fn append_ex_array(&self, e: &[&dyn Ex]) {
        todo!()
    }

    fn head_str(&self) -> String {
        todo!()
    }

    fn get_value(&self) -> String {
        todo!()
    }
}
