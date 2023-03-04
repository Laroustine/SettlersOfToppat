mod setup;
mod map;
mod map_impl;
mod resource;
mod player;

mod tests;



fn main() {
    let mut players: Vec<player::Player> = Vec::new();
    let mut m = setup::create_map();
    let mut plyr = player::Player::new();
    players.push(plyr);
    setup::setup(players, &mut m);
    println!("{}", m.to_json());
    println!("{}", m.to_json_with_coords());
    server::server();
}
