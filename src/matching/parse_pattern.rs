/*!

  We call the pattern a "form", as the form need not be a pattern expression proper.


*/
use std::ops::Range;


use crate::{
  expression::Expression,
  atoms::{symbol::Symbol, mexpression::MExpression},
  // interfaces::{
  //   ExpRep,
  // }
};


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ParsedForm {
  span: Range<u32>,
  form: Expression,
  original_form: Expression,
  is_blank: bool,
  is_implied_blank: bool,
  is_optional: bool,
  default_expression: Expression,
  has_pattern: bool,
  pattern_symbol: Symbol
}

struct ParseRepeatedResult{
  expression: MExpression,
  min: i32,
  max: i32
}

/**
  Parsing `Repeated` patterns.
    `Repeated[p]`
    `Repeated[p, max]`
    `Repeated[p, {min, max}]`
    `Repeated[p, {n}]`
*/
pub fn parse_repeated(e: &MExpression) -> Option<ParseRepeatedResult> {
  let min: i32 = -1;
  let max: i32 = -1;

  if e.len() < 1 {
    return None;

  } else {  // if len >= 1…
    let list = if let Expression::MExpression(m) = e.part(2) {
      m
    } else {
      return None;
    };

    if !list.has_head("System`List") | list.len() != 2 {
      return None;
    }

  }

  None
}
