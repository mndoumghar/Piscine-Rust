pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}


pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    
            match server {
                Result::Ok(s) =>  match security_level{
                        Security::Warning => s.to_string(),
                        Security::Message => s.to_string(),
                        Security::NotFound =>s.to_string(),
                        Security::Unknown => s.to_string(),
                        Security::UnexpectedUrl => panic!("{}",s),
                    
                },
                Result::Err(e) => match security_level {
                        Security::Warning => "WARNING: check the server".to_string(),
                        Security::Message => panic!("ERROR: program stops"),
                        Security::NotFound => format!("Not found: {}",e),
                        Security::Unknown => server.unwrap().to_string(),
                        Security::UnexpectedUrl => panic!("{}",e),
                },
            }

    }


