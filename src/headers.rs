use std::collections::HashMap;
use std::fmt::{Display,Formatter,Result};

pub struct HTTPHeaders(HashMap<&'static str,&'static str>);

impl HTTPHeaders {
    pub fn other(&mut self, identifier: &'static str, setting:  &'static str){
        self.0.insert(identifier,setting);

    }

    pub fn content_type(&mut self, setting: &'static str){
        self.0.insert("Content-Type",setting);

    }
    pub fn content_encoding(&mut self, setting: &'static str){
        self.0.insert("Content-Encoding",setting);

    }
    pub fn connection(&mut self, setting: &'static str){
        self.0.insert("Connection",setting);

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
