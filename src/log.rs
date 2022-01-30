/*!

  Logging. I want to be a lumberjack.


*/


use strum::{
  Display,
  IntoStaticStr
};

#[derive(Hash, Display, Eq, PartialEq, Clone, Copy, IntoStaticStr)]
pub enum LogLevel {
	Critical,
	Error,
	Warning,
	Notice,
	Info,
	Debug,
}


pub trait LoggingInterface {
  fn log_debug(&self, msg: &str);
  fn log_info(&self, msg: &str);
  fn log_error(&self, msg: &str);
  fn debug_on(&self, level: LogLevel);
  fn debug_off(&self);
  fn set_debug_state(&self, new_state: bool);
  fn is_profiling(&self) -> bool;
  fn set_profiling(&self, profiling: bool);
  fn set_up_logging(&self);
}
