pub trait Res {
    fn to_str(&self) -> &str;
}

impl Res for Resource {
    fn to_str(&self) -> &str {
         match self {
            Resource::Sheep   => "sheep",
            Resource::Hay     => "hay", 
            Resource::Wood    => "wood",
            Resource::Rock    => "rock",
            Resource::Clay    => "clay",
            Resource::Desert  => ""
        } 
    }
}

pub trait Map {
    fn new() -> Self;
    fn to_json(&self) -> String;
    fn map_f<T>(&self, f: fn(&Hex) -> T);
}

impl Map for Board {
    fn new() -> Self {
        let n = 5; //number of rows (= length of middle row)
        let mut m: Board = Board {board: HashMap::<Coord, Hex>::new()};
        for i in 0..((n+1)/2) { //to keep a hexagon, the shortest row is of length (n+1)/2.
            for j in 0..(n-i) {
                let c1 = Coord{x: i, y:j};
                let c2 = Coord{x:-i, y:j};
                m.board.insert(c1, Hex {
                    coord: Coord{x:i, y:j},
                    resource: Resource::Desert,
                    roll: 0,
                    player_rank: None
                });
                m.board.insert(c2, Hex {
                    coord: Coord{x:-1,y:j},
                    resource: Resource::Desert,
                    roll: 0,
                    player_rank: None
                });
            }
        }
        m
    }

    fn map_f<T>(&self, f: fn(&Hex) -> T) {
        self.board.values().map(|h| f(h));
    }

    fn to_json(&self) -> String {
        let mut title = Vec::<Resource>::new();
        let mut value = Vec::<u8>::new();
        //self.board.values().map(|hex| title.push(hex.resource));
        //self.board.values().map(|hex| value.push(hex.roll)); //TODO 2 iterations over values. find a way to do both at once
        title.push(Resource::Desert);
        value.push(0);
        let mut str = String::from("{ ");
        str.push_str("title\": [");
        str.push_str(title.iter().map(|r| String::from(r.to_str())).collect::<String>().as_str());
        str.push_str("], \"value\": [");
        str.push_str(value.iter().map(|r| r.to_string()).collect::<String>().as_str()); 
        str.push_str("] }}");
        String::from(str)
    }
}

