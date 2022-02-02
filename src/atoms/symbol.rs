
use std::fmt::Display;
use std::hash::{
  Hash,
  Hasher
};
use std::ops::Index;
use std::rc::Rc;

use fnv::FnvHasher;

use crate::IsEqual;
use crate::expression::{
  Ex,
  ExpressionInterface,
  RcEx, ExpressionKind
};
use crate::string::FormattingParameters;


pub type RcSymbol = Rc<Symbol>;

// Todo: Should strings be interned?
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Symbol {
  // This awkward construction is due to the need to give out owned references to
  // a vector containing `name.`
  pub name: Vec<Rc<String>>,
  pub hash: u64
}

impl Display for Symbol {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl Symbol {

  pub fn new_rc(name: String) -> RcSymbol {
    Rc::new(
      Symbol {
        name: vec![Rc::new(name)],
        hash: 0
      }
    )
  }

  fn name(&self) -> &String {
    self.name.index(0)
  }

  // Private to force a new hash even if `self.hash != 0`.
  fn _hash(&self) -> u64 {
    let hasher = FnvHasher::default();

    hasher.write(self.name().as_bytes());
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

}


impl Ex for Symbol {

  fn string_form(&self, params: &FormattingParameters) -> String {
    if self.name().len() == 0 {
      return "<EMPTYSYM>".to_string();
    }

    if self.name().starts_with(&params.context) {
      return self.name()
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
        let path = path_part.string_form(params);

        // Don't print the `Global\`whatever` part if it's in scope.
        if self.name().starts_with(&path) {
          return self.name()
                    .strip_prefix(
                          &params.context
                        )
                    .unwrap()
                    .to_string();
        }
      }
    }
    return format_sym_name(self.name(), params)
  }

  fn is_equal(&self, other: &dyn Ex) -> IsEqual {
    match other.kind() {

      self.kind() => {
        let params = FormattingParameters::standard();
        (self.string_form(&params) == other.string_form(&params)).into()
      }

      _ => IsEqual::False

    }
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
    ExpressionKind::Symbol
  }

}


// Free Functions

/// Formats the given symbol name according to `params.Form`.
/// (TeXForm is not implemented.)
pub fn format_sym_name(name: &str, params: &FormattingParameters) -> String {
  // Todo: Implement TeXForm
  name.to_string()
}
