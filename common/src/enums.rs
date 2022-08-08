use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ROLES {
    ADMIN,
    USER,
}

impl Display for ROLES {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
