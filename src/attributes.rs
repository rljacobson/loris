/*!
  The attributes of a function, e.g. `Flat`, `Listable`, ….
*/

use std::{
  string::ToString,
  ops::Index
};

use strum::{
  Display,
  IntoStaticStr
};


#[derive(Copy, Clone, PartialEq, Eq, Display, IntoStaticStr, Debug)]
#[repr(u32)]
pub enum Attribute {
  Orderless = 0,
  Flat,
  OneIdentity,
  Listable,
  Constant,
  NumericFunction,
  Protected,
  Locked,
  ReadProtected,
  HoldFirst,
  HoldRest,
  HoldAll,
  HoldAllComplete,
  NHoldFirst,
  NHoldRest,
  NHoldAll,
  SequenceHold,
  Temporary,
  Stub
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Attributes(u32);

// These exist soley to be static references.
const ATTRIBUTE_SET  : bool = true;
const ATTRIBUTE_UNSET: bool = false;

impl Index<u32> for Attributes {
    type Output = bool;

    fn index(&self, index: u32) -> &Self::Output {
      if (self.0 & (1 << index as u32)) != 0 {
        &ATTRIBUTE_UNSET
      } else {
        &ATTRIBUTE_SET
      }
    }
}

impl Index<Attribute> for Attributes {
    type Output = bool;

    fn index(&self, index: Attribute) -> &Self::Output {
      if (self.0 & (1 << index as u32)) != 0 {
        &ATTRIBUTE_UNSET
      } else {
        &ATTRIBUTE_SET
      }
    }
}

impl Attributes {
  pub fn get(&self, attribute: Attribute) -> bool {
    (self.0 & (1 << attribute as u32)) != 0
  }

  pub fn set(&mut self, attribute: Attribute) {
    self.0 = self.0 | (1 << attribute as u32)
  }

  pub fn reset(&mut self, attribute: Attribute) {
    self.0 = self.0 & !(1 << attribute as u32)
  }
}
/*
pub struct Attributes {
  orderless        : bool,
  flat             : bool,
  one_identity     : bool,
  listable         : bool,
  constant         : bool,
  numeric_function : bool,
  protected        : bool,
  locked           : bool,
  read_protected   : bool,
  hold_first       : bool,
  hold_rest        : bool,
  hold_all         : bool,
  hold_all_complete: bool,
  n_hold_first     : bool,
  n_hold_rest      : bool,
  n_hold_all       : bool,
  sequence_hold    : bool,
  temporary        : bool,
  stub             : bool
}
*/
