use std::collections::HashMap;
use std::fmt::{Display,Formatter,Result};

pub struct HTTPHeaders(HashMap<&'static str,String>);

impl HTTPHeaders {
    pub fn other(&mut self, identifier: &'static str, setting:  &'static str){
        self.0.insert(identifier,setting.to_string());

    }

    pub fn content_type(&mut self, setting: &'static str, charset: &'static str){
        let s = format!("{}; charset = {}",setting, charset);
        self.0.insert("Content-Type",s);

    }

    pub fn content_encoding(&mut self, setting: &'static str){
        self.0.insert("Content-Encoding",setting.to_string());

    }
    pub fn connection(&mut self, setting: &'static str){
        self.0.insert("Connection",setting.to_string());

    }


    pub fn new() -> Self{HTTPHeaders(HashMap::new())}
}


impl Display for HTTPHeaders    {

    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut r: Result = Ok(());
        for (k,v) in self.0.iter() {
            r = write!(f,"{}: {}\n",k,v);
        }

        r

    }
}
