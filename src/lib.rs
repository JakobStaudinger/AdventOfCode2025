pub mod cli {
    use std::env;
    use std::fs;
    use std::str::FromStr;

    pub enum Level {
        Level1,
        Level2,
    }

    impl FromStr for Level {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "level1" => Ok(Level::Level1),
                "level2" => Ok(Level::Level2),
                _ => Err(format!("{} is not a valid level number", s)),
            }
        }
    }

    pub fn run<F>(input_file: &str, f: F)
    where
        F: Fn(Level, &str),
    {
        let mut args = env::args();
        args.next().expect("Unexpected end of arguments");
        let level = args
            .next()
            .expect("Please provide a level number (e.g. level1)")
            .parse::<Level>()
            .unwrap();

        assert!(args.next().is_none(), "Unexpected extra arguments provided");

        let input = fs::read_to_string(input_file)
            .unwrap_or_else(|_| panic!("Could not read file {input_file}"));
        f(level, &input);
    }
}
