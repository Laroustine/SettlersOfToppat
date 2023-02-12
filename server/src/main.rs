mod setup;
mod map;
mod resource;

use crate::map::*;
use crate::resource::*;
use std::collections::HashMap;

fn main() {
    let roll =  setup::dice_roll();
    let m = map::Board::new();
    let mut m2: Board = Board {board: HashMap::<Coord, Hex>::new()};
    m2.board.insert(
        Coord{x:0,y:0}, 
        Hex {
            coord: Coord{x:0,y:0},
            resource: Resource::Wood,
            roll: 0,
            player_rank: None
        });
    m2.board.insert(
        Coord{x:0,y:1},
        Hex{
            coord: Coord{x:0,y:0},
            resource: Resource::Clay,
            roll: 1,
            player_rank: Some(vec![0])
        });

    println!("roll: {}", roll);
    println!("{}", Resource::Wood.to_str());
    println!("{}",m2.to_json());
    //println!("{}", m.to_json());
    let mut vec: Vec::<i32> = Vec::<i32>::new();
    vec.push(0);
    vec.push(1);
    println!("{}",format!("{:?}", vec));
    server::server();
    
}
