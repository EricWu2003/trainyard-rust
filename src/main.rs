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

pub enum ConnectionType {
    None,
    S,
    B,
    H,
    Z,
    M,
    J,
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
    fn switch_active_passive(&mut self) {
        // this function is called whenever an odd number of trains rolls through a tracktile
        // if there is no passive connection, then we do nothing
        if self.passive_connection != None {
            let temp = self.passive_connection;
            self.passive_connection = self.active_connection;
            self.active_connection = temp;
        }
    }

    fn has_connections(&self, c1: Connection, c2: Connection) -> bool {
        // returns true iff self has both an active and passive connection,
        // and the connections match c1 and c2 (regardless of active/passive)
        if let Some(my_c1) = self.active_connection {
            if let Some(my_c2) = self.passive_connection {
                return (my_c1 == c1 && my_c2 == c2) || (my_c1 == c2 && my_c2 == c1);
            }
        }
        false
    }

    fn has_connection_up_to_rot(&self, c: Connection) -> i8 {
        // returns -1 if there is no connection, otherwise returns the rotation amount
        if let Some(my_c) = self.active_connection {
            for rot_amt in 0..4 {
                if my_c.rot(rot_amt) == c {
                    return rot_amt as i8;
                }
            }
        }
        if let Some(my_c) = self.passive_connection {
            for rot_amt in 0..4 {
                if my_c.rot(rot_amt) == c {
                    return rot_amt as i8;
                }
            }
        }
        -1
    }

    fn has_connections_up_to_rot(&self, c1: Connection, c2: Connection) -> i8 {
        // returns true iff self has both an active and passive connection,
        // and the connections match c1 and c2 (regardless of active/passive)
        // after being rotated a fixed amount
        if let Some(my_c1) = self.active_connection {
            if let Some(my_c2) = self.passive_connection {
                for rot_amt in 0..4 {
                    let rot_my_c1 = my_c1.rot(rot_amt);
                    let rot_my_c2 = my_c2.rot(rot_amt);
                    if (rot_my_c1 == c1 && rot_my_c2 == c2) || (rot_my_c1 == c2 && rot_my_c2 == c1)
                    {
                        return rot_amt as i8;
                    }
                }
            }
        }
        -1
    }

    fn connection_type(&self) -> ConnectionType {
        if self.active_connection == None {
            return ConnectionType::None;
        }
        if self.passive_connection == None {
            if (self.has_connection_up_to_rot(Connection { dir1: 0, dir2: 2 }) != -1) {
                return ConnectionType::S;
            }
            return ConnectionType::B;
        }
        // now we can assume that there is both an active and passive connection
        if self.has_connections(
            Connection { dir1: 0, dir2: 2 },
            Connection { dir1: 1, dir2: 3 },
        ) {
            return ConnectionType::H;
        }
        if self.has_connections_up_to_rot(
            Connection { dir1: 0, dir2: 1 },
            Connection { dir1: 2, dir2: 3 },
        ) != -1
        {
            return ConnectionType::Z;
        }

        if self.has_connections_up_to_rot(
            Connection { dir1: 0, dir2: 1 },
            Connection { dir1: 1, dir2: 2 },
        ) != -1
        {
            return ConnectionType::M;
        }

        if self.has_connections_up_to_rot(
            Connection { dir1: 0, dir2: 1 },
            Connection { dir1: 0, dir2: 2 },
        ) != -1
            || self.has_connections_up_to_rot(
                Connection { dir1: 0, dir2: 3 },
                Connection { dir1: 0, dir2: 2 },
            ) != -1
        {
            return ConnectionType::J;
        }

        unreachable!()
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
                self.trains.push(Train {
                    color,
                    source: dir as u8,
                    destination: None,
                })
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
