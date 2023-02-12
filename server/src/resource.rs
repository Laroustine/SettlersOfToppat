pub enum Resource {
    Sheep,
    Hay,
    Wood,
    Rock,
    Clay,
    Desert
} 


impl Resource {
    pub fn to_str(&self) -> &str {
         match self {
            Resource::Sheep   => "sheep",
            Resource::Hay     => "hay", 
            Resource::Wood    => "wood",
            Resource::Rock    => "rock",
            Resource::Clay    => "clay",
            Resource::Desert  => ""
        } 
    }
}

