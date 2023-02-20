use std::collections::HashMap;
use crate::map::*;
use crate::resource::*;

impl Board {
    pub fn new() -> Self {
        let mut m: Board = 
            Board {
                board: HashMap::<Coord, Hex>::new(),
                points: HashMap::<Point, Option<(Building, u8)>>::new()
            };
        for i in 0..3 { //to keep a hexagon, the shortest row is of length (n+1)/2.
            for j in 0..(5-i) {
                m.board.insert(Coord{x:i, y:j},
                               Hex {
                                   coord: Coord{x:i, y:j},
                                   resource: Resource::Desert,
                                   roll: 0,
                                   player_rank: None
                               });
                m.board.insert(Coord{x:-i, y:j},
                               Hex {
                                   coord: Coord{x:-i, y:j},
                                   resource: Resource::Desert,
                                   roll: 0,
                                   player_rank: None
                               });
            }
        }
        m
    }

    fn map_f<T>(&self, f: fn(&Hex) -> T) {
        self.board.values().map(|mut h| f(&mut h));
    }

    pub fn to_json(&self) -> String {
        let iter = self.board.values();
        let mut t: Vec::<&str> = Vec::<&str>::new();
        let mut v: Vec::<u8> = Vec::<u8>::new();
        for h in iter {
            t.push(h.resource.to_str());
            v.push(h.roll);
        }

        format!("{{\"title\": {:?}, \"value\": {:?}}}", t, v) 
    }
    pub fn to_json_with_coords(&self) -> String {
        let iter = self.board.values();
        let mut t: Vec::<&str> = Vec::<&str>::new();
        let mut v: Vec::<u8> = Vec::<u8>::new();
        let mut c: Vec::<(i32,i32)> = Vec::<(i32,i32)>::new();
        for h in iter {
            t.push(h.resource.to_str());
            v.push(h.roll);
            let (a,b) = (h.coord.x, h.coord.y);
            c.push((a,b));
        }

        format!("{{\"title\": {:?}, \"value\": {:?}, \"coords\": {:?}}}", t, v,c) 
    }

}

impl Hex {
    pub fn neighbour(&self, vertex: u8) -> Hex {
        match vertex {
            0 => Hex {
                    coord: Coord {x:self.coord.x-1,y:self.coord.y+1},
                    resource:self.resource,
                    roll:self.roll,
                    player_rank: self.player_rank.clone()
            },
            1 => Hex {
                    coord: Coord {x:self.coord.x,y:self.coord.y+1},
                    resource:self.resource,
                    roll:self.roll,
                    player_rank: self.player_rank.clone()
            },
            2 => Hex {
                    coord: Coord {x:self.coord.x+1,y:self.coord.y},
                    resource:self.resource,
                    roll:self.roll,
                    player_rank: self.player_rank.clone()
            },
            3 => Hex {
                    coord: Coord {x:self.coord.x+1,y:self.coord.y-1},
                    resource:self.resource,
                    roll:self.roll,
                    player_rank: self.player_rank.clone()
            },
            4 => Hex {
                    coord: Coord {x:self.coord.x,y:self.coord.y-1},
                    resource:self.resource,
                    roll:self.roll,
                    player_rank: self.player_rank.clone()
            },
            _ => Hex {
                    coord: Coord {x:self.coord.x-1,y:self.coord.y},
                    resource:self.resource,
                    roll:self.roll,
                    player_rank: self.player_rank.clone()
            } 
        }
    }
}

impl Point {
    pub fn twins(&self) -> Vec<Point> {
        let mut twins: Vec<Point> = Vec::new();
        twins.push(Point {
            building: self.building.clone(),
            hex: self.hex.neighbour((self.pos - 1)%6),
            pos: (self.pos + 4)%6,
        });
        twins.push(Point {
            building: self.building.clone(),
            hex: self.hex.neighbour(self.pos),
            pos: (self.pos + 2)%6,
        });
        twins
    }
}
