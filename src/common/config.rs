pub struct Config {
    pub(crate) file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let file_path: String = args[1].clone();
        Ok(Config { file_path })
    }
}
