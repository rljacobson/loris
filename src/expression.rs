/*!

The abstract interfaces for expressions. Note the naming convention: the trait
is called `WidgetInterface`, while the reference counted copy-on-write
pointer to a `WidgetInterface` is a `Widget`. For copy-on-write operations,
be sure to use `widget.make_mut()` rather than `widget.get_mut()`.

*/

use std::hash::Hasher;

use strum::EnumDiscriminants;

use crate::{
  formatting::FormattingParameters,
  IsEqual,
  interfaces::ExpressionRepresentation,
  atoms::{
    symbol::Symbol,
    string::StringExpression,
    mexpression::MExpression
  }
};

/// Wrapper around `atoms` and `MExpression`s the primary function of which is to
/// provide virtual dispatch based on expression type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumDiscriminants)]
pub enum Expression {
  StringExpression(StringExpression),
  Symbol(Symbol),
  MExpression(MExpression),
  // Other(Expression)
}

type Ex = Expression;

macro_rules! forward_call {
  ($func_name:ident, $ret_type:ty) => {
    pub fn $func_name(&self) -> $ret_type {
      match self {
        Expression::StringExpression(e) => e.$func_name(),
        Expression::Symbol(e)           => e.$func_name(),
        Expression::MExpression(e)      => e.$func_name(),
      }
    }
  }
}

impl Expression {


  forward_call!(deep_copy, Expression);
  forward_call!(copy, Expression);
  forward_call!(needs_eval, bool);
  forward_call!(hash, u64);


  pub fn string_form(&self, params: &FormattingParameters) -> String {
    match self {
      Expression::StringExpression(e) => e.string_form(params),
      Expression::Symbol(e)           => e.string_form(params),
      Expression::MExpression(e)      => e.string_form(params),
    }
  }

  /// This differs from `same_q` in that it recursively tests equality, whereas `same_q` only compares the hash values.
  pub fn is_equal(&self, other: &Expression) -> IsEqual {
    match (*self, *other) {

      (Ex::StringExpression(u), Ex::StringExpression(v)) => u.is_equal(other),

      (Ex::Symbol(u), Ex::Symbol(v)) => u.is_equal(other),

      (Ex::MExpression(u), Ex::MExpression(v)) => u.is_equal(other),

      _ => IsEqual::False
    }
  }

  pub fn same_q(&self, other: &Expression) -> bool {
    // todo: Handle special case of floats.
    self.hash() == other.hash()
  }

  // Checks if the head of this expression is a symbol with name `other`. If this
  // expression is not an `MExpression`, returns false.
  pub fn has_head(&self, other: &str) -> bool {
    if let Expression::MExpression(e) = *self {
      if let Expression::Symbol(Symbol(name)) = e.part(0) {
        return name == other
      }
    }
    return false;
  }

  // Checks if the head of this expression is the expression `other`. If this
  // expression is not an `MExpression`, returns false.
  pub fn has_head_expression(&self, other: &Expression) -> bool {
    if let Expression::MExpression(e) = *self {
      e.has_head_expression(other)
    } else {
      false
    }
  }

}

// todo: Should `PartialEq` be implemented in terms of `is_same`?
impl PartialOrd for Expression {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {

      (Ex::StringExpression(u), Ex::StringExpression(v)) => u.partial_cmp(v),

      (Ex::Symbol(u), Ex::Symbol(v)) => u.partial_cmp(v),

      (Ex::MExpression(u), Ex::MExpression(v)) => u.partial_cmp(v),

      _ => None

    }
  }
}



macro_rules! upcast_repr_to_expr {
  ($repr: ty) => {
    impl From<$repr> for Expression {
      fn from(representation: $repr) -> Self {
        Expression::$repr(representation)
      }
    }
  };
}

upcast_repr_to_expr!(StringExpression);
upcast_repr_to_expr!(Symbol);
upcast_repr_to_expr!(MExpression);
