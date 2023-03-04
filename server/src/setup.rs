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

    let mut m: Board =
        Board {
            board: HashMap::<Coord, Hex>::new(),
            points: HashMap::<Point, Option<(Building, u8)>>::new()
        };

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


    for i in -2..3 { //to keep a hexagon, the shortest row is of length (n+1)/2.
        for j in 0..(5-i32::abs(i)) {
            // insert a random resource, with a randomly attached roll to every possible coordanate, that isn't occupied by the desert
            // we have to match, to avoid poping the arrays when encountering the desert
            match m.board.get(&Coord{x:i, y:j}) {
                None => {
                    let res  = resources.pop().unwrap();
                    let r = nums.pop().unwrap();

                    //insert the resource in a hex
                    m.board.insert( Coord{x:i,y:j},
                                    Hex {
                                        coord: Coord{x:i, y:j},
                                        resource: res.clone(),
                                        roll: r.clone(),
                                        player_rank: None
                                    });
                    //complete the vertexes of the hex
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
                },

                //something is there, so it's the desert
                _ => {
                    for i in 0..6 {
                        m.points.insert(
                            Point {
                                building: None,
                                hex: Hex {
                                    coord: Coord{x:a, y:b},
                                    resource: Resource::Desert,
                                    roll: 0,
                                    player_rank: Some(vec![0])
                                },
                                pos: i
                            }, None);
                    }
                },
            };
        }
    }
    m
}

pub fn setup(mut players: Vec::<Player>, map: &mut Board) {
    for plyr in players.iter_mut() {
        let i = plyr.rank as i32;
        let mut start_pt =
            Point {
                building: Some((Building::Settlement, plyr.rank)),
                hex: Hex {
                    coord: Coord{x:0, y:i},
                    resource: map.board.get(&Coord{x:0, y:i}).unwrap().resource,
                    roll: map.board.get(&Coord{x:0, y:i}).unwrap().roll,
                    player_rank: None
                },
                pos: (i%6) as u8
            };
        plyr.build(&Building::Settlement, &mut start_pt, map);
    }
}



