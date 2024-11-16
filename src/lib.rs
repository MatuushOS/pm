#[cfg(test)]
mod tests {
	use std::process::Command;
	#[test]
	fn build_hello() {
	    Command::new("target/debug/pm").args(["build", "examples/hello"]).status().unwrap();
	}
	
}