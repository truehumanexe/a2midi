use regex::Regex;


/// A wrapper around useful regexes for gui text input
/// separated from the app editor for easy testing.
pub struct AppRegex{
    pub tempo: Regex,
    pub time_signature: Regex,
}

impl AppRegex{
    
    pub fn new() -> Self{
        let tempo = Regex::new(r"^(\d{2,3})(\.\d{1,2}){0,1}").expect("Tempo regex not compiled"); 
        
        let time_signature = Regex::new(r"^(\d{1,2})(\/)(\d{1,2})").expect("Time signature regex not compiled"); 

        Self{
            tempo,
            time_signature,
        }
    }

    pub fn validate(re: &Regex, text: &str) -> bool{
        let caps = re.captures(text);

        match caps {
            // first value of a successful match always returns some, so skip it
            Some(c) => c.iter().skip(1).all(|m| m.is_some()),
            None => false,
        }
    }

}
