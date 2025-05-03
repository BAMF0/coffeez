use std::hash::Hash;
use std::fmt;
use serde::{Serialize, Deserialize};

// Excessive enumeration, but origins are well defined
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Origin {
    Brazil,
    Colombia,
    ElSalvador,
    Ethiopia,
    Kenya,
}

impl Origin {
    fn as_str(&self) -> &str  {
        match self{
            Self::Brazil => "Brazil",
            Self::Colombia => "Colombia",
            Self::ElSalvador => "El Salvador",
            Self::Ethiopia => "Ethiopia",
            Self::Kenya => "Kenya",
        }
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct CoffeeBean {
    name: String,
    origin: Origin,
    roastery: String,
}


impl CoffeeBean {
    pub fn new(name: String, origin: Origin, roastery: String) -> Self {
        CoffeeBean { name, origin, roastery }
    }
}