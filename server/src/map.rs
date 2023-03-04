//the  coordanates system is defined as fommows:
//     /  \ /  \ /  \
//    |-2,0|-2,1|-2,2|
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


//a point is defined as the number of the vertex of a hexagon, strating at the top, and counting
//clockwise
//
//     0
//    /\
// 5 /  \ 1
//  | i,j|
//  |    |
// 4 \  / 2
//    \/
//     3
//
// Some points are thus defined multiple times, but by have immediate access the hex, and thus the
// coordanates attached, and we can easilly identify if 2 points are the same using the coordanates
// of the hex and their position.
// this does mean we must do some point operations up to 3 times
// we can access a hex's vertices in constant time
// hopefully this system won't bring any problems down the line
//
use std::collections::HashMap;
use crate::resource::*;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Hex {
    pub coord: Coord,
    pub resource: Resource,
    pub roll: u8,
    pub player_rank: Option<Vec<i32>>
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Point {
    pub building: Option<(Building, u8)>, //the building, and the player it belongs to
    pub hex: Hex,
    pub pos: u8
}

#[derive(Clone)]
pub struct Board {
    pub board: HashMap<Coord,Hex>,
    pub points: HashMap<Point, Option<(Building, u8)>>
}
