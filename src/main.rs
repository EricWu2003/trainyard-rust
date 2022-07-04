pub mod connection;
pub use crate::connection::Connection;
pub mod color;
pub use crate::color::Color;
pub mod tile;
pub use crate::tile::Tile;
pub use crate::tile::tracktile::Tracktile;
pub mod edge;



// pub struct Yard {
//     tiles : (types like Tracktile)
//     borders: (border objects that lie between the Tracktiles)
// }

fn main() {
    // let t1 = Color::Blue;
    // let t2 = Color:: Red;
    // let t3 = Color::Green;
    // println!("{:?}", t1.mix_with(t2));
    // println!("{:?}", t1.mix_with(t3));
    // println!("{:?}", Color::mix_many(&[t1, t2]));
    // println!("{:?}", Color::mix_many(&[t1, t2, t2]));
    let mut my_tile = Tracktile::new( 
        Some(Connection{dir1: 0, dir2: 1}),
        Some(Connection{dir1: 0, dir2: 2}),
    );
    let res = my_tile.accept_trains([None, Some(Color::Red), Some(Color::Yellow), None]);
    println!("{}", res);
    println!("{:?}", my_tile);
    my_tile.interact_trains();
    println!("{:?}", my_tile);
}
