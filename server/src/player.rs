use std::collections::HashMap;
use crate::map::*;
use crate::resource::*;
use crate::resource::Resource::*;

pub struct Player {
    pub rank: u8,
    pub points: u8,
    pub buildings: HashMap<Building, u8>,
    pub resources: HashMap<Resource, u8>
}

impl Player {
    pub fn new() -> Player {

        let mut r: HashMap<Resource, u8> = HashMap::new();
        r.insert(Brick,0);
        r.insert(Lumber,0);
        r.insert(Wool,0);
        r.insert(Grain,0);
        r.insert(Ore,0);

        let mut b: HashMap<Building, u8> = HashMap::new();
        b.insert(Building::Road, 15);
        b.insert(Building::Settlement, 5);
        b.insert(Building::City, 4);
        Player {
            rank: 0,
            points: 0,
            buildings: b,
            resources: r
        }
    }

    pub fn has_enough(&self, build_type: Building) -> bool {
        match build_type {
            Building::Road => 
                *self.resources.get(&Brick).unwrap()  > 0 && 
                *self.resources.get(&Lumber).unwrap() > 0 &&
                *self.buildings.get(&Building::Road).unwrap() > 0,
            Building::Settlement => 
                *self.resources.get(&Brick).unwrap()  > 0 &&
                *self.resources.get(&Lumber).unwrap() > 0 && 
                *self.resources.get(&Wool).unwrap()   > 0 &&
                *self.resources.get(&Grain).unwrap()  > 0 &&
                *self.buildings.get(&Building::Settlement).unwrap() > 0,
            Building::City => 
                *self.resources.get(&Ore).unwrap()   > 2 && 
                *self.resources.get(&Grain).unwrap() > 1 &&
                *self.buildings.get(&Building::City).unwrap() > 0,
        }
    }

    // this indicates to the point "point" that it is occupied by a building.
    // to create a road between two points, this must be called twice
    pub fn build(&mut self, build_type: &Building, point: &mut Point, map: &mut Board) {

        // look if the point is already occupied
        match &point.building {
            Some((b,r)) => match (b,r) {
                // if it is occupied, and it is my settelment, and i'm trying to make a city,
                // proceed  
                // if not, do nothing
                (&Building::Settlement, x) if *x==self.rank => {
                    match build_type {
                        Building::City => (),
                        _ => {
                            println!("Point already occupied");
                            return ();
                        },
                    }}

                _ => {
                    println!("Point already occupied");
                    return ();
                }
            },
            None => (),
        }
        match build_type {
            Building::Road => {
                //can probably change into an option return and throw error
                if !self.has_enough(Building::Road) {
                    println!("Not enough resources to build!");
                    return ()
                }
                point.building = Some((Building::Road, self.rank));
                *self.resources.get_mut(&Brick).unwrap()  -= 1;
                *self.resources.get_mut(&Lumber).unwrap() -= 1;
            },
            Building::Settlement => {
                if !self.has_enough(Building::Settlement) {
                    println!("Not enough resources to build!");
                    return ()
                }
                point.building = Some((Building::Settlement, self.rank));
                *self.resources.get_mut(&Brick).unwrap()  -= 1;
                *self.resources.get_mut(&Lumber).unwrap() -= 1;
                *self.resources.get_mut(&Wool).unwrap()   -= 1;
                *self.resources.get_mut(&Grain).unwrap()  -= 1;
            },
            Building::City => {
                if !self.has_enough(Building::City) {
                    println!("Not enough resources to build!");
                    return ()
                }
                point.building = Some((Building::City, self.rank));
                *self.resources.get_mut(&Ore).unwrap()   -= 3;
                *self.resources.get_mut(&Grain).unwrap() -= 2;

                //refund the upraded Settlement
                *self.buildings.get_mut(&Building::Settlement).unwrap() += 1;
            }
        };
        map.points.insert(point.clone(), Some((build_type.clone(), self.rank)));
        *self.buildings.get_mut(&build_type).unwrap() -= 1;
    }
}
