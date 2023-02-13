mod setup;
mod map;
mod map_impl;
mod resource;
mod player;

use std::collections::HashMap;

use crate::map::*;
use crate::resource::*;
use crate::player::Player;

fn main() {
    let m = setup::create_map();
    let mut m2: Board = Board {board: HashMap::<Coord, Hex>::new()};
    m2.board.insert(
        Coord{x:0,y:0}, 
        Hex {
            coord: Coord{x:0,y:0},
            resource: Resource::Lumber,
            roll: 0,
            player_rank: None
        });
    m2.board.insert(
        Coord{x:0,y:1},
        Hex{
            coord: Coord{x:0,y:0},
            resource: Resource::Brick,
            roll: 1,
            player_rank: Some(vec![0])
        });

    println!("{}", m.to_json());
    println!("{}", m.to_json_with_coords());
    server::server();
    
    let mut p = Player::new();
    p.resources.insert(Resource::Brick, 1);
    p.resources.insert(Resource::Lumber, 1);
    println!("p has enough for road: {}", p.has_enough(Building::Road));
    println!("p has enough for settlement: {}", p.has_enough(Building::Settlement));
}
