mod setup;
mod map;
mod resource;

use crate::map::*;
use crate::resource::*;
use std::collections::HashMap;

fn main() {
    let m = setup::create_map();
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

    println!("{}", m.to_json());
    println!("{}", m.to_json_with_coords());
    server::server();
    
}
