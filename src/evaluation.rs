/*

  Traits and structs related to the evaluation of expressions.


*/

use crate::{
  string::{
    ToStringFnType
  },
  expression::{
    Ex,
    ExpressionInterface, RcEx
  },
  Def, DefinitionMap, log::LoggingInterface, data_structures::TimeCounterGroup, StreamManager,

};


pub trait EvalStateForStringer {
  fn get_string_fn(&self, head_str: &str) -> Option<ToStringFnType>;
  // Used by Definition[]
  fn get_defined(&self, name: &str) -> Option<&Def>;
}

pub trait EvalStateInterface: EvalStateForStringer {
  fn	eval(&self, expr: &dyn Ex) -> RcEx;

  fn	set_defined(&self, name: &str, def: &Def);
  fn	init(&self, load_all_defs: bool);
  fn	is_def(&self, name: &str) -> bool;
  fn	get_def(&self, name: &str, lhs: &dyn Ex) ->
      (RcEx, bool, Box<dyn ExpressionInterface>);
  fn	get_sym_def(&self, name: &str) -> Option<RcEx>;
  fn	mark_seen(&self, name: &str);
  fn	define(&self, lhs: &dyn Ex, rhs: &dyn Ex);
  fn	clear_all(&self);
  fn	clear(&self, name: &str);
  fn	get_defined_snapshot(&self) -> Box<&dyn DefinitionMap>;
  fn	is_frozen(&self) -> bool;
  fn	set_frozen(&self, frozen: bool);
  fn	is_interrupted(&self) -> bool;
  fn	get_string_def(&self, name: &str, default_val: String) -> String;
  fn	get_list_def(&self, name: &str) -> Box<dyn ExpressionInterface>;
  fn	throw(&self, e: Box<dyn ExpressionInterface>);
  fn	has_thrown(&self) -> bool;
  fn	thrown(&self) -> Box<dyn ExpressionInterface>;
  fn	process_top_level_result(&self, input: &dyn Ex, output: &dyn Ex) -> RcEx;

  fn	get_logger(&self) -> Box<dyn LoggingInterface>;
  fn	get_trace(&self) -> Box<dyn ExpressionInterface>;
  fn	set_trace(&self, new_trace: Box<dyn ExpressionInterface>);
  fn	get_defined_map(&self) -> Box<&dyn DefinitionMap>;
  fn	get_reap_sown(&self) -> Box<dyn ExpressionInterface>;
  fn	set_reap_sown(&self, ex: Box<dyn ExpressionInterface>);

  fn	get_time_counter(&self) -> TimeCounterGroup;
  fn	get_stream_manager(&self) -> Box<&dyn StreamManager>;
}


// Todo: Implement `EvalState`
/// A dummy implementation
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EvalState {
  // pass
}

impl EvalState {
  pub fn new() -> EvalState {
    EvalState {}
  }
}

// Todo: impl EvalStateInterface for EvalState

impl EvalStateForStringer for EvalState {
  fn get_string_fn(&self, head_str: &str) -> Option<ToStringFnType>{None}

  fn get_defined(&self, name: &str) -> Option<&Def> {None}
}
