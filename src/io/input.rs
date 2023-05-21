pub trait Input {
    fn read(&self) -> std::io::Result<String>;
}

pub struct StdinInput;

impl Input for StdinInput {
    fn read(&self) -> std::io::Result<String> {
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;
        Ok(user_input)
    }
}

pub struct MockInput {
    pub input: String,
}

impl Input for MockInput {
    fn read(&self) -> std::io::Result<String> {
        Ok(self.input.clone())
    }
}
