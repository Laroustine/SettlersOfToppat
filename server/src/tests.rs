#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::*;
    use crate::resource::*;
    use crate::setup;
    use crate::player::*;
    use std::collections::HashMap;



    #[test]
    fn test_map_insert() {
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
        assert_eq!(m2.board.get(&Coord{x:0,y:1}).unwrap().resource, Resource::Brick);
        assert_eq!(m2.board.get(&Coord{x:0,y:1}).unwrap().roll, 1);
        assert_eq!(m2.board.get(&Coord{x:0,y:1}).unwrap().player_rank, Some(vec![0]));
    }

    #[test]
    fn test_map_to_json() {
        let mut m2: Board = Board {board: HashMap::<Coord, Hex>::new()};
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
        let mut m2: Board = Board {board: HashMap::<Coord, Hex>::new()};
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


    #[test]
    fn test_player_resource_insert() {
        let mut p: Player = Player::new();
        let mut pt: Point = Point{building: None, x:Coord{x:0,y:0}, y: Coord{x:0,y:1}, z: None};
        p.resources.insert(Resource::Brick, 1);
        p.resources.insert(Resource::Lumber, 1);
        assert_eq!(*p.resources.get(&Resource::Brick).unwrap(), 1);
        assert_eq!(*p.resources.get(&Resource::Lumber).unwrap(), 1);
        assert_eq!(*p.resources.get(&Resource::Wool).unwrap(), 0);
    }


    #[test]
    fn test_player_has_enough() {
        let mut p: Player = Player::new();
        p.resources.insert(Resource::Brick, 1);
        p.resources.insert(Resource::Lumber, 1);
        assert!(!p.has_enough(Building::Settlement));
        assert!(p.has_enough(Building::Road));
        *p.buildings.get_mut(&Building::City).unwrap() = 0;
        assert!(!p.has_enough(Building::City));
    }

    #[test]
    fn test_player_build_changes_point() {
        let mut p: Player = Player::new();
        let mut pt: Point = Point{building: None, x:Coord{x:0,y:0}, y: Coord{x:0,y:1}, z: None};
        p.resources.insert(Resource::Brick, 1);
        p.resources.insert(Resource::Lumber, 1);
        p.build(&Building::Settlement, &mut pt);
        assert_eq!(pt.building, None);
        p.build(&Building::Road, &mut pt); 
        assert_eq!(pt.building.unwrap(), (Building::Road, 0));
    }

    #[test]
    fn test_player_build_consumes_resources() {
        let mut p: Player = Player::new();
        let mut pt: Point = Point{building: None, x:Coord{x:0,y:0}, y: Coord{x:0,y:1}, z: None};
        p.resources.insert(Resource::Brick, 1);
        p.resources.insert(Resource::Lumber, 1);
        p.build(&Building::Road, &mut pt); 
        assert_eq!(*p.resources.get(&Resource::Brick).unwrap(), 0);
        assert_eq!(*p.resources.get(&Resource::Lumber).unwrap(), 0);
    }
}
