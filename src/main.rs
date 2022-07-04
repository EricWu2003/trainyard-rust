pub mod connection;
pub use crate::connection::Connection;
pub mod color;
pub use crate::color::Color;
pub mod tile;
pub use crate::tile::tracktile::Tracktile;
pub use crate::tile::Tile;
pub mod edge;
pub mod yard;
pub use crate::yard::Yard;

fn main() {
    let mut yard = Yard::new();
    yard.display();
}
