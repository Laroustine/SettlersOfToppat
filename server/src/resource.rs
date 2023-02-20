use crate::map::*;
use crate::player::Player;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum Resource {
    Wool,
    Grain,
    Lumber,
    Ore,
    Brick,
    Desert
} 


impl Resource {
    pub fn to_str(&self) -> &str {
         match self {
            Resource::Wool   => "sheep",
            Resource::Grain  => "hay", 
            Resource::Lumber => "wood",
            Resource::Ore    => "rock",
            Resource::Brick  => "clay",
            Resource::Desert => ""
        } 
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Building {
    Road,
    Settlement,
    City
}
