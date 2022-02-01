/*!

  The concrete atom expression. For the abstract expression interface, see
  `crate::expression`.

*/

use crate::{
  expression::{
    Ex,
    ExpressionInterface, ExpressionKind, RcEx
  },
  log::LoggingInterface,
  IsEqual, string::FormattingParameters
};




pub struct Expression {
	parts                 : Vec<RcEx>,
	needs_eval            : bool,
	correctly_instantiated: bool,
	evaled_hash           : u64,
	cached_hash           : u64,
}

impl Expression {
  pub fn head_assertion(&self, head: &dyn ExpressionInterface, logger: &dyn LoggingInterface) -> bool {
    is_sameq(head, self.get_part(0))
  }
}

impl Ex for Expression {

  fn string_form(&self, params: &FormattingParameters) -> String {
    let head     = self.get_part(0);
    let head_str = head.string_form(params);

    // If the head is a symbol with a `to_string_fn`, use that function.
    if head.kind() == ExpressionKind::Symbol {

      if let Some(to_string_fn) = params.esi.get_string_fn(&head_str) {
        return to_string_fn(self, params);
      }

      // If the head is `System\`*Form`, adjust the formatting parameters accordingly.
      if self.get_parts().len() == 2 {
          match head_str.as_str() {

            "System`InputForm`"
            | "System`InputForm"
            | "System`FullForm"
            | "System`TraditionalForm"
            | "System`TeXForm"
            | "System`StandardForm"
            | "System`OutputForm" => {
              let mut new_params = params.clone();
              return self.get_part(1).string_form(&mut new_params);
            }

            _ => {
              // pass
            }

          } // end match on head.name()
        } // end if the head is `System\`*Form`
    } // end if head is symbol

    // todo: Why are we not respecting `params.form`?
    // todo: Add limits on the length of the output.
    // Default print as M-expression.
    let sequence =
      self.parts[1..]
          .iter()
          .map(|v| v.string_form(params))
          .collect()
          .join(", ");

    format!("{}[{}]", head_str, sequence)
  }

  fn is_equal(&self, b: &dyn Ex) -> IsEqual {
      todo!()
  }

  fn deep_copy(&self) -> RcEx {
      todo!()
  }

  fn copy(&self) -> RcEx {
      todo!()
  }

  fn needs_eval(&self) -> bool {
      todo!()
  }

  fn hash(&self) -> u64 {
      todo!()
  }

  fn kind(&self) -> ExpressionKind {
    ExpressionKind::OtherExpression
  }
}

impl ExpressionInterface for Expression {
    fn get_parts(&self) -> &Vec<RcEx> {
        &self.parts
    }

    fn get_part(&self, i: usize) -> RcEx {
        self.parts[i].clone()
    }

    fn set_parts(&self, new_parts: &Vec<RcEx>) {
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

    fn append_ex(&self, e: RcEx) {
        todo!()
    }

    fn append_ex_array(&self, e: &[RcEx]) {
        todo!()
    }

    fn head_str(&self) -> String {
        todo!()
    }
}
