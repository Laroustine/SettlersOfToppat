extern crate rand;
use rand::{random, thread_rng, seq::SliceRandom};
use crate::map::*;
use crate::resource::*;
use crate::player::*;
use std::collections::HashMap;

/* @args: none
 * @requires: nothing
 * @ensures: returns the sum of a double 6 sided die roll
 */
pub fn dice_roll() -> u8 {
    let d1 = random::<u8>() % 6 + 1;
    let d2 = random::<u8>() % 6 + 1;
    d1 + d2
}


/* @args: none
 * @requires: nothing
 * @ensures: returns a randomized starting game board
 */
pub fn create_map() -> Board {
    //all the resources to place on the map (except the desert)
    let mut resources = vec![
        Resource::Wool, Resource::Wool, Resource::Wool, Resource::Wool,
        Resource::Grain, Resource::Grain, Resource::Grain, Resource::Grain,
        Resource::Lumber, Resource::Lumber, Resource::Lumber, Resource::Lumber,
        Resource::Ore, Resource::Ore, Resource::Ore,
        Resource::Brick, Resource::Brick, Resource::Brick,];
    resources.shuffle(&mut thread_rng());

    let mut resources2 = resources.clone(); //duplicate in same order to generate the Points, avoiding move problems

    //all the possible dice rolls to place on the map (except for the desert, marked with 0)
    let mut nums =  Vec::<u8>::new();
    for i in 3..7 {
        nums.push(i);
        nums.push(i);
    }
    for i in 8..12 {
        nums.push(i);
        nums.push(i);
    }
    nums.push(2);
    nums.push(12);
    nums.shuffle(&mut thread_rng());

    let mut nums2 = nums.clone();

    let mut m: Board =
        Board {
            board: HashMap::<Coord, Hex>::new(),
            points: HashMap::<Point, Option<(Building, u8)>>::new()
        };

    let pts = HashMap::<Point, Option<(Building, u8)>>::new();

    //place the desert first in a random spot
    let a: i32 = random::<u8>() as i32 % 5 -2;
    let b: i32 = random::<u8>() as i32 % (5 - a.abs());

    m.board.insert(Coord{x:a,y:b},
                   Hex {
                       coord: Coord{x:a, y:b},
                       resource: Resource::Desert,
                       roll: 0,
                       player_rank: Some(vec![0]) //the robber
                   });

    m.points.insert(
        Point {
            building: None,
            hex: Hex {
                coord: Coord{x:a, y:b},
                resource: Resource::Desert,
                roll: 0,
                player_rank: Some(vec![0])
            },
            pos: 0
        }, None);

    for i in -2..3 { //to keep a hexagon, the shortest row is of length (n+1)/2.
        for j in 0..(5-i32::abs(i)) {
            // insert a random resource, with a randomly attached roll to every possible coordanate, that isn't occupied by the desert
            // we have to match, to avoid poping the arrays when encountering the desert
            match m.board.get(&Coord{x:i, y:j}) {
                None => m.board.insert( Coord{x:i,y:j},
                                        Hex {
                                            coord: Coord{x:i, y:j},
                                            resource: resources.pop().unwrap(),
                                            roll: nums.pop().unwrap(),
                                            player_rank: None
                                        }),
                _ => None,
            };
            //do the same thing with the points in the same order to keep the correct coord <->
            //resource+roll association

            let res = resources2.pop().unwrap();
            let r = nums2.pop().unwrap();
            for i in 0..6 {
                m.points.insert(
                    Point {
                        building: None,
                        hex: Hex {
                            coord: Coord{x:i, y:j},
                            resource: res.clone(),
                            roll: r.clone(),
                            player_rank: None
                        },
                        pos: i as u8
                    }, None);
            }
        }
    }
    m.points = pts;
    m
}

pub fn setup(n_players: u8) {
    let mut m = create_map();
    let mut players: Vec::<Player> = Vec::<Player>::new();
    for i in 0..(n_players as i32) {
        let mut plyr = Player::new();
        plyr.rank = (i+1) as u8;
        let mut start_pt =
            Point {
                building: Some((Building::Settlement, plyr.rank)),
                hex: Hex {
                    coord: Coord{x:0, y:i},
                    resource: m.board.get(&Coord{x:0, y:i}).unwrap().resource,
                    roll: m.board.get(&Coord{x:0, y:i}).unwrap().roll,
                    player_rank: None
                },
                pos: (i%6) as u8
            };
        plyr.build(&Building::Settlement, &mut start_pt);
    }
}



