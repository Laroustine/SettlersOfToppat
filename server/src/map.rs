//the  coordanates system is defined as fommows:
//     /  \ /  \ / \
//    |0,-1| 2,0|2,1|
//   /  \  / \  / \ / \
//  | 1,-2|1,-1|1,1|1,2|
// /  \ /  \  / \ /  \ / \
//|0, 0|0, 1|0, 2|0, 3|0, 4|
// \  / \ /  \ /  \ / \  /
// |-1,0|-1,1|-1,2|-1,3|
//  \  / \  / \  / \  /
//   |-2,0|-2,1|-2,2|
//     \ /  \ /  \ /
//
// Every intersection point is uniquely defined by 
// the 3 neighbouring hexagons

use std::collections::HashMap;
use crate::resource::*;

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
    pub x: Coord,
    pub y: Coord,
    pub z: Option<Coord>,
}

pub struct Board {
    pub board: HashMap<Coord,Hex>,
}


impl Board {
    pub fn new() -> Self {
        let n = 5; //number of rows (= length of middle row)
        let mut m: Board = Board {board: HashMap::<Coord, Hex>::new()};
        for i in 0..((n+1)/2) { //to keep a hexagon, the shortest row is of length (n+1)/2.
            for j in 0..(n-i) {
                let c1 = Coord{x: i, y:j};
                let c2 = Coord{x:-i, y:j};
                m.board.insert(c1, Hex {
                    coord: Coord{x:i, y:j},
                    resource: Resource::Desert,
                    roll: 0,
                    player_rank: None
                });
                m.board.insert(c2, Hex {
                    coord: Coord{x:-1,y:j},
                    resource: Resource::Desert,
                    roll: 0,
                    player_rank: None
                });
            }
        }
        m
    }

    fn map_f<T>(&self, f: fn(&Hex) -> T) {
        self.board.values().map(|mut h| f(&mut h));
    }

    pub fn to_json(&self) -> String {
        let title: Vec::<&str> = self.board.iter().map(|(_,v)| v.resource.to_str()).collect();
        let value: Vec::<u8>   = self.board.iter().map(|(_,v)| v.roll).collect();
        format!("{{\"title\": {:?}, \"value\": {:?}}}", title, value) 
    }
}

