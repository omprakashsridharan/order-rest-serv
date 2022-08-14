use std::fmt::{Display, Formatter, Result};

use sea_orm::strum::EnumString;
use serde::{Deserialize, Serialize};

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum ROLES {
    ADMIN,
    USER,
}

impl Display for ROLES {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
