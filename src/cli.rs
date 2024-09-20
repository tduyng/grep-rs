pub struct Config {
    pub pattern: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip the program name

        let flag = args.next().ok_or("Expected flag")?;
        if flag != "-E" {
            return Err("Expected flag to be '-E'");
        }
        let pattern = args.next().ok_or("Expected pattern")?;

        Ok(Config { pattern })
    }
}
