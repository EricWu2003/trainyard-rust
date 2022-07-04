use crate::color::Color;
use crate::connection::Connection;
use crate::tile::BorderState;

// used for storing a train in a Tracktile
#[derive(Debug)]
pub struct Train {
    color: Color,
    source: u8,
    destination: u8,
}

#[derive(Debug)]
pub struct Tracktile {
    active_connection: Option<Connection>,
    passive_connection: Option<Connection>,
    trains: Vec<Train>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConnectionType {
    None,
    S,
    B,
    H,
    Z,
    M,
    J,
}
impl ConnectionType {
    pub fn get_char(&self) -> char {
        match self {
            ConnectionType::None => '_',
            ConnectionType::S => 'S',
            ConnectionType::B => 'B',
            ConnectionType::H => 'H',
            ConnectionType::Z => 'Z',
            ConnectionType::M => 'M',
            ConnectionType::J => 'J',
        }
    }
}

impl Tracktile {
    pub fn new(
        active_connection: Option<Connection>,
        passive_connection: Option<Connection>,
    ) -> Tracktile {
        Tracktile {
            active_connection,
            passive_connection,
            trains: Vec::new(),
        }
    }

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
        let c = self.connection_type();
        if c == ConnectionType::M || c == ConnectionType::J {
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

    pub fn has_connection_up_to_rot(&self, c: Connection) -> i8 {
        // returns -1 if there is no connection, otherwise returns the rotation amount
        if let Some(my_c) = self.active_connection {
            for rot_amt in 0..4 {
                if c.rot(rot_amt) == my_c {
                    return rot_amt as i8;
                }
            }
        }
        if let Some(my_c) = self.passive_connection {
            for rot_amt in 0..4 {
                if c.rot(rot_amt) == my_c {
                    return rot_amt as i8;
                }
            }
        }
        -1
    }

    pub fn has_active_connection_up_to_rot(&self, c: Connection) -> i8 {
        // returns -1 if there is no connection, otherwise returns the rotation amount
        if let Some(my_c) = self.active_connection {
            for rot_amt in 0..4 {
                if c.rot(rot_amt) == my_c {
                    return rot_amt as i8;
                }
            }
        }
        -1
    }

    pub fn has_connections_up_to_rot(&self, c1: Connection, c2: Connection) -> i8 {
        // returns true iff self has both an active and passive connection,
        // and the connections match c1 and c2 (regardless of active/passive)
        // after being rotated a fixed amount
        if let Some(my_c1) = self.active_connection {
            if let Some(my_c2) = self.passive_connection {
                for rot_amt in 0..4 {
                    let rot_c1 = c1.rot(rot_amt);
                    let rot_c2 = c2.rot(rot_amt);
                    if (my_c1 == rot_c1 && my_c2 == rot_c2) || (my_c1 == rot_c2 && my_c2 == rot_c1)
                    {
                        return rot_amt as i8;
                    }
                }
            }
        }
        -1
    }
    pub fn has_active_passive_connections_up_to_rot(
        &self,
        active: Connection,
        passive: Connection,
    ) -> i8 {
        if let Some(my_active) = self.active_connection {
            if let Some(my_passive) = self.passive_connection {
                for rot_amt in 0..4 {
                    let rot_active = active.rot(rot_amt);
                    let rot_passive = passive.rot(rot_amt);
                    if my_active == rot_active && my_passive == rot_passive {
                        return rot_amt as i8;
                    }
                }
            }
        }
        -1
    }

    fn indices_of_trains_along(&self, c: Connection, i1: &mut usize, i2: &mut usize) -> bool {
        // returns true iff there is a train colliding along the connection c.
        for index1 in 0..self.trains.len() {
            for index2 in 0..self.trains.len() {
                if self.trains[index1].source == c.dir1
                    && self.trains[index1].destination == c.dir2
                    && self.trains[index2].source == c.dir2
                    && self.trains[index2].destination == c.dir1
                {
                    *i1 = index1;
                    *i2 = index2;
                    return true;
                }
            }
        }
        false
    }

    pub fn connection_type(&self) -> ConnectionType {
        if self.active_connection == None {
            return ConnectionType::None;
        }
        if self.passive_connection == None {
            if self.has_connection_up_to_rot(Connection { dir1: 0, dir2: 2 }) != -1 {
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
    pub fn interact_trains(&mut self) {
        let my_type = self.connection_type();

        if self.trains.len() >= 2 {
            if my_type == ConnectionType::H
                || my_type == ConnectionType::S
                || my_type == ConnectionType::B
            {
                // simply mix all the trains in these connection types
                let new_color =
                    Color::mix_many(self.trains.iter().map(|train| train.color).collect());
                for i in 0..self.trains.len() {
                    self.trains[i].color = new_color;
                }
                return;
            }

            if my_type == ConnectionType::Z {
                let mut i1: usize = 0;
                let mut i2: usize = 0;
                // first do mixing on Active Connection
                if self.indices_of_trains_along(self.active_connection.unwrap(), &mut i1, &mut i2) {
                    let new_color =
                        Color::mix_many(vec![self.trains[i1].color, self.trains[i2].color]);
                    self.trains[i1].color = new_color;
                    self.trains[i2].color = new_color;
                }
                // then do mixing on Passive Connection
                if self.indices_of_trains_along(self.passive_connection.unwrap(), &mut i1, &mut i2)
                {
                    let new_color =
                        Color::mix_many(vec![self.trains[i1].color, self.trains[i2].color]);
                    self.trains[i1].color = new_color;
                    self.trains[i2].color = new_color;
                }
                return;
            }

            // At this point, we know we either have a J or M type connection.

            // In either case, we will first check to see if there are trains colliding along the active connection.
            // If there are, we mix those.
            // Then, we merge the trains (if there are any going to the same destination)

            let mut i1: usize = 0;
            let mut i2: usize = 0;
            if self.indices_of_trains_along(self.active_connection.unwrap(), &mut i1, &mut i2) {
                let new_color = Color::mix_many(vec![self.trains[i1].color, self.trains[i2].color]);
                self.trains[i1].color = new_color;
                self.trains[i2].color = new_color;
            }

            'outer: for i1 in 0..self.trains.len() {
                for i2 in 0..self.trains.len() {
                    if i1 == i2 {
                        continue;
                    }

                    if self.trains[i1].destination == self.trains[i2].destination {
                        let new_color =
                            Color::mix_many(vec![self.trains[i1].color, self.trains[i2].color]);
                        self.trains[i1].color = new_color;
                        self.trains.remove(i2);
                        break 'outer;
                    }
                }
            }
        }

        if self.trains.len() % 2 == 1 {
            self.switch_active_passive();
        }
    }

    pub fn accept_trains(&mut self, colors: BorderState) -> bool {
        // return true if no trains crash, and return false if trains crashed.
        for dir in 0..4 {
            let possible_color = colors[dir as usize];

            if let Some(color) = possible_color {
                if !self.has_any_connection(dir) {
                    return false;
                }

                //now we have to determine the train's destination based on it's source direction (dir).
                if let Some(active_conn) = self.active_connection {
                    if active_conn.contains(dir) {
                        let other_dir = if active_conn.dir1 == dir {
                            active_conn.dir2
                        } else {
                            active_conn.dir1
                        };
                        self.trains.push(Train {
                            color,
                            source: dir,
                            destination: other_dir,
                        });
                        continue;
                    }
                }
                if let Some(passive_conn) = self.passive_connection {
                    if passive_conn.contains(dir) {
                        let other_dir = if passive_conn.dir1 == dir {
                            passive_conn.dir2
                        } else {
                            passive_conn.dir1
                        };
                        self.trains.push(Train {
                            color,
                            source: dir,
                            destination: other_dir,
                        });
                        continue;
                    }
                }
                unreachable!()
            }
        }

        true
    }

    pub fn dispatch_trains(&mut self) -> BorderState {
        // we panic if two trains have the same destination, since we should have dealt with that already,
        let mut res = [None, None, None, None];
        while let Some(train) = self.trains.pop() {
            let dest = train.destination;
            if res[dest as usize] != None {
                panic!();
            }
            res[dest as usize] = Some(train.color);
        }
        res
    }

    pub fn add_connection(&mut self, conn: Connection) {
        self.passive_connection = self.active_connection;
        self.active_connection = Some(conn);
    }
}
