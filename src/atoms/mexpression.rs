use std::cmp::Ordering;
use std::hash::Hasher;

use fnv::FnvHasher;

use crate::{
  interfaces::ExpressionRepresentation,
  expression::Expression, IsEqual, formatting::FormattingParameters
};

use super::symbol::Symbol;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Default, Hash)]
pub struct MExpression {
  parts:                  Vec<Expression>,
  evaled_hash:            u64,
  cached_hash:            u64,
  needs_eval:             bool,
  correctly_instantiated: bool,
}

impl MExpression {
  pub fn parts(&self) -> &Vec<Expression> {
    &self.parts
  }

  pub fn part(&self, i: usize) -> Expression {
    self.parts[i]
  }

  pub fn set_parts(&self, new_parts: Vec<Expression>) {
    self.parts = new_parts;
  }

  pub fn clear_hashes(&self) {
    self.evaled_hash = 0;
    self.cached_hash = 0;
  }

  pub fn len(&self) -> usize {
    self.parts.len() - 1
  }

  pub fn less(&self, i: usize, j: usize) -> Option<Ordering> {
    self.parts[i+1].partial_cmp(&self.parts[j+1])
  }

  pub fn swap(&mut self, i: usize, j: usize) {
    std::mem::swap(&mut self.parts[i+1], &mut self.parts[j+1]);
  }

  pub fn append_ex(&self, e: Expression) {
    self.parts.push(e);
  }

  pub fn append_ex_array(&mut self, mut e: Vec<Expression>) {
    self.parts.extend(e);
  }

  pub fn head_str(&self) -> String {
    match self.parts[0] {

      Expression::Symbol(Symbol(name)) => name.clone(),

      _ => "".to_string()

    }
  }

  pub fn has_head(&self, head: &str) -> bool {
    if let Expression::Symbol(Symbol(name)) = self.parts[0] {
      name == head
    } else {
      false
    }
  }

  pub fn has_head_expression(&self, other_expression: &Expression) -> bool {
    let head_expression = self.parts[0];
    head_expression.same_q(other_expression)
  }
}


impl ExpressionRepresentation for MExpression {
  fn string_form(&self, params: &FormattingParameters) -> String {
    // let head      = self.get_part(0);
    // let full_form = false;

    // if let Expression::Symbol(s) = head {
    //   if !full_form & params.esi.is_some() {
    //     let head_str = head.string_form(params);
    //     let formatting_function = params.esi.as_ref().unwrap();
    //   }
    // }
    todo!()
  }

  /// This differs from `same_q` in that it recursively tests equality, whereas `same_q` only compares the hash values.
  fn is_equal(&self, other: &Expression) -> IsEqual {
    if let Expression::MExpression(other) = other {

      if self.len() != other.len() {
        return IsEqual::False;
      }

      for (self_part, other_part) in
        self.parts.iter().zip(other.parts.iter()) {
        match self_part.is_equal(other_part) {

          IsEqual::Unknown => return IsEqual::Unknown,

          IsEqual::False   => return IsEqual::False,

          IsEqual::True    => { /* pass */ }

        }
      }

      // We only reach here if every part `IsEqual::True`.
      return IsEqual::True;

    } else { unreachable!() }
  }

  fn deep_copy(&self) -> Expression {
    let mut new_expression = MExpression::default();
    new_expression.parts.reserve(self.len());
    new_expression.evaled_hash = self.evaled_hash;
    new_expression.cached_hash = self.cached_hash;
    new_expression.needs_eval  = self.needs_eval;
    new_expression.correctly_instantiated
    = self.correctly_instantiated;

    for part in self.parts{
      new_expression.parts.push(part.deep_copy());
    }

    Expression::MExpression(new_expression)
  }

  fn copy(&self) -> Expression {
    Expression::MExpression(self.clone())
  }

  fn needs_eval(&self) -> bool {
    self.needs_eval
  }

  fn hash(&self) -> u64 {
    if self.cached_hash != 0 {
      return self.cached_hash;
    }

    let mut hasher = FnvHasher::default();

    hasher.write(&[72u8, 5u8, 244u8, 86u8, 5u8, 210u8, 69u8, 30u8]);
    for part in self.parts {
      hasher.write_u64(part.hash());
    }

    let result       = hasher.finish();
    self.cached_hash = result;

    result
  }
}
