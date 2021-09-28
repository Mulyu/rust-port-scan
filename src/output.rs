use std::fmt::{self};

pub struct Message {
	pub ports: Vec<u16>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut formatted = Vec::new();
		for port in &self.ports {
			formatted.push(port.to_string());
		}
		write!(f, "{}", formatted.join(", "))
    }
} 