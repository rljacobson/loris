
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use fnv::FnvHasher;

use crate::expression::{Ex, ExpressionInterface};
use crate::string::FormattingParameters;



// Todo: Should strings be interned?
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Symbol<'s> {
  pub name: &'s str,
  pub hash: u64
}

impl<'s> Display for Symbol<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'s> Symbol<'s> {

  pub fn new(name: &str) -> Symbol {
    Symbol {
      name,
      hash: 0
    }
  }

  // Private to force a new hash even if `self.hash != 0`.
  fn _hash(&self) -> u64 {
    let hasher = FnvHasher::default();

    hasher.write(self.name.as_bytes());
    self.hash = hasher.finish();

    return self.hash;
  }

  /// Returns the hash of this symbol.
  pub fn hash(self) -> u64 {
    if self.hash == 0 {
      self._hash();
    }

    return self.hash;
  }

  // Formatting-oriented Methods

  pub fn string_form(&self, params: FormattingParameters) -> String {
    if self.name.len() == 0 {
      return "<EMPTYSYM>".to_string();
    }

    if self.name.starts_with(&params.context) {
      return self.name
                  .strip_prefix(
                    &params.context
                  )
                  .unwrap()
                  .to_string();
    }

    { // Scope for part_iter
      let mut part_iter = params.context_path.get_parts().into_iter();
      part_iter.next(); // Skip the head
      for path_part in part_iter {
        let path = path_part.string_value().unwrap();
        
        if self.name.starts_with(&path) {
          return self.name
                      .strip_prefix(
                        &params.context
                      )
                      .unwrap()
                      .to_string();
        }
      }
    }
    return format_sym_name(&self.name, &params)
  }

}


impl<'s> Hash for Symbol<'s> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
  }
}


impl<'s> Ex for Symbol<'s> {
    fn string_form(&self, params: &FormattingParameters) -> String {
        todo!()
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

impl<'s> ExpressionInterface for Symbol<'s> {
    fn get_parts(&self) -> Vec<&dyn Ex> {
        vec![self]
    }

    fn get_part(&self, i: usize) -> &dyn Ex {
        todo!()
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
        self.name.to_string()
    }

    fn get_value(&self) -> String {
        self.head_str()
    }
}


// Free Functions

/// Formats the given symbol name according to `params.Form`.
/// (TeXForm is not implemented.)
pub fn format_sym_name(name: &str, params: &FormattingParameters) -> String {
  // Todo: Implement TeXForm
  name.to_string()
}
