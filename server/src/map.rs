//the  coordanates system is defined as fommows:
//     /  \ /  \ /  \
//    |-2,0|-2,1|*2,2|
//   / \  / \  / \  /  \
//  |-1,0|-1,1|-1,2|-1,3|
// / \ /  \  / \  / \  /
//|0, 0|0,1|0,2|0,3|0,4|
// \ / \ / \ / \ / \  /
//  |1,0|1,1|1,2|1,3|
//  \ / \ / \ / \ /
//   |2,0|2,1|2,2|
//    \ / \ / \ /
//
// Every intersection point is uniquely defined by 
// the 3 neighbouring hexagons

use std::collections::HashMap;
use crate::resource::*;
use crate::map_impl::*;

#[derive(Eq, Hash, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

pub struct Hex {
    pub coord: Coord,
    pub resource: Resource,
    pub roll: u8,
    pub player_rank: Option<Vec<i32>>
}

pub struct Point {
    pub building: Option<(Building, u8)>, //the building, and the player it belongs to
    pub x: Coord,
    pub y: Coord,
    pub z: Option<Coord>,
}

pub struct Board {
    pub board: HashMap<Coord,Hex>,
}
