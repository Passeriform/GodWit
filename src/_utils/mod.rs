//! Utility functions and macros
#![macro_use]
macro_rules! str_to_command {
	($command_str:expr, $args_vec:expr) => {{
		let exec_vec = $command_str.split_whitespace();
		$args_vec.extend(exec_vec.clone().skip(1).collect::<Vec<_>>());
		exec_vec.collect::<Vec<_>>()[0].to_string()
		}};
}
