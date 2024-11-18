// Excessive enumeration, but origins are well defined
enum Origin {
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

struct CoffeBean {
    name: String,
    origin: Origin,
    roastery: String,
}

impl CoffeBean {
    fn new(name: String, origin: Origin, roastery: String) -> Self {
        CoffeBean { name, origin, roastery }
    }
}

