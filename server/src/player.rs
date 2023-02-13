use std::collections::HashMap;
use crate::map::*;
use crate::resource::*;
use crate::resource::Resource::*;

pub struct Player {
    points: u8,
    pub resources: HashMap<Resource, u8>,
    rank: u8
}

impl Player {
    pub fn new() -> Player {

        let mut r: HashMap<Resource, u8> = HashMap::new();
        r.insert(Brick,0);
        r.insert(Lumber,0);
        r.insert(Wool,0);
        r.insert(Grain,0);
        r.insert(Ore,0);
        Player {
            points: 0,
            resources: r,
            rank: 0
        }
    }

    pub fn has_enough(&self, build_type: Building) -> bool {
        match build_type {
            Building::Road => self.resources.get(&Brick).unwrap() > &0 && self.resources.get(&Lumber).unwrap() > &0,
            Building::Settlement => 
                self.resources.get(&Brick).unwrap()  > &0 &&
                self.resources.get(&Lumber).unwrap()  > &0 && 
                self.resources.get(&Wool).unwrap()  > &0 &&
                self.resources.get(&Grain).unwrap()  > &0,
            Building::City => self.resources.get(&Ore).unwrap() > &2 && self.resources.get(&Grain).unwrap() > &1
        }
    }
    pub fn build(&self, build_type: Building, point: &mut Point) {

    }
}
