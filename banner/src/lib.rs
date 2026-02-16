use std::{ collections::HashMap, num::ParseFloatError };
pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let short = format!("-{}", name.chars().next().unwrap());
        let big = format!("--{}", name);
        Flag {
            short_hand: short,
            long_hand: big,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(func) = self.flags.get(input) {
            return match func(argv[0], argv[1]) {
                Ok(num) => Ok(num),
                Err(e) => Err(e.to_string()),
            };
        } else {
            Err("hhhhh".to_string())
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1: f64 = a.parse()?;
    let num2: f64 = b.parse()?;
    let result = num1 / num2;
    Ok(result.to_string())
    
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
   let num1: f64 = a.parse()?;
    let num2: f64 = b.parse()?;
    let result = num1  % num2;
    Ok(result.to_string())
    
    
}