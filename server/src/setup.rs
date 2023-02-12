extern crate rand;
use rand::random;
use crate::map::*;


pub fn dice_roll() -> u8 {
    let d1 = random::<u8>() % 6 + 1;
    let d2 = random::<u8>() % 6 + 1;
    d1 + d2
}

pub fn create_map() -> Board {
   Board::new() 
   }



