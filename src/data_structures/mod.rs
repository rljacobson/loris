pub mod hash;
use std::collections::HashMap;


pub type TimeMap<'a>  = HashMap<&'a str, f64>;
pub type CountMap<'a> = HashMap<&'a str, u64>;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeCounter<'a> {
  pub times  : TimeMap<'a>,
  pub counts : CountMap<'a>
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeCounterGroup<'a> {
  pub def_time_counter      : TimeCounter<'a>,
	pub lhs_def_time_counter   : TimeCounter<'a>,
	pub eval_time_counter     : TimeCounter<'a>,
	pub head_eval_time_counter : TimeCounter<'a>
}
