use crate::map::*;
use crate::resource::*;
use std::collections::HashMap;


#[test]
fn test_map_insert() {
    let mut m2: Board =
        Board {
            board: HashMap::<Coord, Hex>::new(),
            points: HashMap::<Point, Option<(Building, u8)>>::new()};
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
    assert_eq!(m2.board.get(&Coord{x:0,y:1}).unwrap().resource, Resource::Brick);
    assert_eq!(m2.board.get(&Coord{x:0,y:1}).unwrap().roll, 1);
    assert_eq!(m2.board.get(&Coord{x:0,y:1}).unwrap().player_rank, Some(vec![0]));
}

#[test]
fn test_map_to_json() {
    let mut m2: Board =
        Board {
            board: HashMap::<Coord, Hex>::new(),
            points: HashMap::<Point, Option<(Building, u8)>>::new()};
    m2.board.insert(
        Coord{x:0,y:1}, 
        Hex {
            coord: Coord{x:0,y:1},
            resource: Resource::Brick,
            roll: 1,
            player_rank: Some(vec![0])
        });
    assert_eq!(m2.to_json(), "{\"title\": [\"clay\"], \"value\": [1]}");
}

#[test]
fn test_map_to_json_with_coords() {
    let mut m2: Board =
        Board {
            board: HashMap::<Coord, Hex>::new(),
            points: HashMap::<Point, Option<(Building, u8)>>::new()};
    m2.board.insert(
        Coord{x:0,y:1}, 
        Hex {
            coord: Coord{x:0,y:1},
            resource: Resource::Brick,
            roll: 1,
            player_rank: Some(vec![0])
        });
    assert_eq!(m2.to_json_with_coords(), "{\"title\": [\"clay\"], \"value\": [1], \"coords\": [(0, 1)]}");
}
