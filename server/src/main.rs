mod setup;
mod map;

use crate::map::*;
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
    server::server();
    
}