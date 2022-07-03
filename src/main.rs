mod connection;
pub use crate::connection::Connection;
mod color;
pub use crate::color::Color;

// used for storing a train in a Tracktile
pub struct Train {
    color: Color,
    source: u8,
    destination: Option<u8>,
}
pub type BorderState = [Option<Color>; 4];


trait Tile {
    fn accept_trains(&mut self, trains: BorderState) -> bool;
    fn dispatch_trains(&mut self) -> BorderState;
}

pub struct Tracktile {
    active_connection: Option<Connection>,
    passive_connection: Option<Connection>,
    trains: Vec<Train>,
}
impl Tracktile {
    fn has_any_connection(&self, dir: u8) -> bool {
        if let Some(connection) = self.active_connection {
            if connection.contains(dir) {
                return true;
            }
        }
        if let Some(connection) = self.passive_connection {
            if connection.contains(dir) {
                return true;
            }
        }
        false
    }
}


impl Tile for Tracktile {

    fn accept_trains(&mut self, colors: BorderState) -> bool {
        // return true if no trains crash, and return false if trains crashed.
        for dir in 0..4 {
            let possible_color = colors[dir];
            if let Some(color) = possible_color {
                if !self.has_any_connection(dir as u8) {
                    return false;
                }
                self.trains.push(Train {color, source: dir as u8, destination: None })
            }
        }

        true
    }
    fn dispatch_trains(&mut self) -> BorderState {
        // by this point, each Train should have a destination, so we panic if some destination is none.
        // we also panic if two trains have the same destination, since we should have dealt with that already,
        let mut res = [None, None, None, None];
        while let Some(train) = self.trains.pop() {
            let dest = train.destination.unwrap();
            if res[dest as usize] != None {
                panic!();
            }
            res[dest as usize] = Some(train.color);
        }
        res
    }

}




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


}

