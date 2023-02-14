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


    #[test]
    fn test_player_resource_insert() {
        let mut p: Player = Player::new();
        let mut pt: Point = Point {
            building: Some((Building::Settlement, p.rank)),
            hex: Hex{
                coord: Coord{x:0,y:0},
                resource: Resource::Desert,
                roll:  0,
                player_rank: None
            },
            pos: 0
        };
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
        let mut pt: Point = Point {
            building: None,
            hex: Hex{
                coord: Coord{x:0,y:0},
                resource: Resource::Desert,
                roll:  0,
                player_rank: None
            },
            pos: 0
        };
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
        let mut pt: Point = Point {
            building: None,
            hex: Hex{
                coord: Coord{x:0,y:0},
                resource: Resource::Desert,
                roll:  0,
                player_rank: None
            },
            pos: 0
        };
        p.resources.insert(Resource::Brick, 1);
        p.resources.insert(Resource::Lumber, 1);
        p.build(&Building::Road, &mut pt); 
        assert_eq!(*p.resources.get(&Resource::Brick).unwrap(), 0);
        assert_eq!(*p.resources.get(&Resource::Lumber).unwrap(), 0);
        assert_eq!(*p.buildings.get(&Building::Road).unwrap(), 14); //consumed a road
    }

    #[test]
    fn test_player_can_upgrade_settlement() {
        let mut p: Player = Player::new();
        p.rank = 1;
        assert_eq!(*p.buildings.get(&Building::Settlement).unwrap(), 5);
        let mut pt: Point = Point {
            building: None,
            hex: Hex{
                coord: Coord{x:0,y:0},
                resource: Resource::Desert,
                roll:  0,
                player_rank: None
            },
            pos: 0
        };
        p.resources.insert(Resource::Ore, 3);
        p.resources.insert(Resource::Grain, 3);
        p.resources.insert(Resource::Brick, 1);
        p.resources.insert(Resource::Lumber, 1);
        p.resources.insert(Resource::Wool, 1);
        p.build(&Building::Settlement, &mut pt); 
        assert_eq!(*p.buildings.get(&Building::Settlement).unwrap(), 4); //consumed a road
        p.build(&Building::City, &mut pt); 
        assert_eq!(pt.building.unwrap(), (Building::City, 1)); //managed to upgrade successfully
        assert_eq!(*p.buildings.get(&Building::Settlement).unwrap(), 5); //got refunded the settlement

    }

    #[test]
    fn test_player_cannot_upgrade_other_player_settlement() {
        let mut p1: Player = Player::new();
        let mut p2: Player = Player::new();
        p1.rank = 1;
        p2.rank = 2;
        let mut pt: Point = Point {
            building: Some((Building::Settlement, 1)),
            hex: Hex{
                coord: Coord{x:0,y:0},
                resource: Resource::Desert,
                roll:  0,
                player_rank: None
            },
            pos: 0
        };
        p2.resources.insert(Resource::Ore, 3);
        p2.resources.insert(Resource::Grain, 2);
        p1.resources.insert(Resource::Grain, 1);
        p1.resources.insert(Resource::Brick, 1);
        p1.resources.insert(Resource::Lumber, 1);
        p1.resources.insert(Resource::Wool, 1);
        p1.build(&Building::Settlement, &mut pt); 
        p2.build(&Building::City, &mut pt); 
        assert_eq!(pt.building.unwrap(), (Building::Settlement, 1)); //managed to upgrade successfully
        assert_eq!(*p2.buildings.get(&Building::City).unwrap(), 4); 
    }

    #[test]
    fn test_player_cant_build_over() {

        let mut p: Player = Player::new();
        p.rank = 1;
        assert_eq!(*p.buildings.get(&Building::Settlement).unwrap(), 5);
        let mut pt: Point = Point {
            building: None,
            hex: Hex{
                coord: Coord{x:0,y:0},
                resource: Resource::Desert,
                roll:  0,
                player_rank: None
            },
            pos: 0
        };
        p.resources.insert(Resource::Grain, 2);
        p.resources.insert(Resource::Brick, 2);
        p.resources.insert(Resource::Lumber, 2);
        p.resources.insert(Resource::Wool, 2);
        p.build(&Building::Settlement, &mut pt); 
        assert_eq!(*p.buildings.get(&Building::Settlement).unwrap(), 4); //consumed a settlement
        assert_eq!(*pt.building.as_ref().unwrap(), (Building::Settlement, 1)); //managed to build
        p.build(&Building::Settlement, &mut pt); 
        assert_eq!(*p.buildings.get(&Building::Settlement).unwrap(), 4); //got situation did not change
        assert_eq!(pt.building.unwrap(), (Building::Settlement, 1)); //managed to build
    }

}
