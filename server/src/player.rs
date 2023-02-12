use std::collections::HashMap;

struct Player {
    resources: HashMap<Resource, u8>,
    map: Board,
    rank: i32
}
