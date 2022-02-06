#![feature(assert_matches)]
/*!

 Defines the fundamental traits and structs used throughout the library.

*/
//#![feature(trait_upcasting)]
pub mod atoms;
pub mod attributes;
pub mod data_structures;
pub mod definition;
pub mod evaluation;
pub mod expression;
pub mod interfaces;
pub mod formatting;
pub mod log;

use std::rc::Rc;


use expression::Expression;
use strum::Display;


pub type StreamManager = Rc<dyn StreamManagerInterface>;

pub trait StreamManagerInterface {
  fn write_string(
    &self,
    stream_name : &str,
    stream_index: i64,
    to_write    : &str
  ) -> bool;
  fn as_expr(&self) -> Expression;
}


/// This is not the same as `PartialEq`, because `PartialEq` uses `false` for incomparable.
#[derive(Copy, Clone, PartialEq, Eq, Display, Hash, Debug)]
#[repr(i8)]
pub enum IsEqual {
  Unknown = -1,
  False,
  True,
}

impl From<bool> for IsEqual {
    fn from(value: bool) -> Self {
        if value {
          Self::True
        } else {
          Self::False
        }
    }
}
