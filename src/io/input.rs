///  Trait for reading some generic input
pub trait Input {
    fn read(&self) -> std::io::Result<String>;
}

/// Represents a file input source
pub struct StdinInput;

impl Input for StdinInput {
    /// Concrete implementation of the read method for StdinInput
    fn read(&self) -> std::io::Result<String> {
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;
        Ok(user_input)
    }
}

/// Represents a mock input source
pub struct MockInput {
    pub input: String,
}

impl Input for MockInput {
    /// Concrete implementation of the read method for MockInput
    fn read(&self) -> std::io::Result<String> {
        Ok(self.input.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdin_input() {
        let stdin_input = StdinInput {};
        let result = stdin_input.read();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_input() {
        let mock_input = MockInput {
            input: String::from("Hello, world!"),
        };
        let result = mock_input.read();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }
}
