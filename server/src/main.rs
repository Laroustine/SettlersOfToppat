mod setup;
mod map;
mod map_impl;
mod resource;
mod player;

mod tests;



fn main() {
    let m = setup::create_map();
    setup::setup(1);
    println!("{}", m.to_json());
    println!("{}", m.to_json_with_coords());
    server::server();
    
}
