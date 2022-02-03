#![feature(trait_upcasting)]
/*!

 Defines the fundamental traits and structs used throughout the library.

*/
pub mod atoms;
pub mod attributes;
pub mod data_structures;
pub mod definition;
pub mod evaluation;
pub mod expression;
pub mod formatting;
pub mod log;

use std::rc::Rc;


use strum::Display;


use expression::Expression;


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
