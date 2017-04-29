use std::collections::HashMap;
use std::fmt::{Display,Formatter,Result};


pub struct HTTPHeaders(pub HashMap<&'static str,&'static str>);


impl Display for HTTPHeaders    {

    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut r: Result = Ok(());
        for (k,v) in self.0.iter() {
            r = write!(f,"{}: {}\n",k,v);
        }

        r

    }
}
