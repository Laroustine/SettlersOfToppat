extern crate rand;
use rand::{random, thread_rng, seq::SliceRandom};
use crate::map::*;
use crate::resource::*;
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
    let mut board = Board::new();

    //all the resources to place on the map (except the desert)
    let mut resources = vec![
        Resource::Sheep, Resource::Sheep, Resource::Sheep, Resource::Sheep,
        Resource::Hay, Resource::Hay, Resource::Hay, Resource::Hay,
        Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood,
        Resource::Rock, Resource::Rock, Resource::Rock,
        Resource::Clay, Resource::Clay, Resource::Clay,];
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

    let n = 5; //number of rows (= length of middle row)
    let mut m: Board = Board {board: HashMap::<Coord, Hex>::new()};

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
                None => m.board.insert( Coord{x:i,y:j},
                Hex {
                    coord: Coord{x:i, y:j},
                    resource: resources.pop().unwrap(),
                    roll: nums.pop().unwrap(),
                    player_rank: None
                }),
                _ => None,
            };
        }
    }
    m
}



