use crate::color::Color::{self, Blue, Brown, Green, Orange, Purple, Red, Yellow};
use crate::connection::Connection;
use crate::tile::painter::Painter;
use crate::tile::splitter::Splitter;
use crate::tile::trainsink::Trainsink;
use crate::tile::trainsource::Trainsource;
use crate::tile::Tile;

use std::str;

#[derive(Clone)]
pub struct PositionedTile {
    pub tile: Tile,
    pub x: u8,
    pub y: u8,
}
pub type LevelInfo = Vec<PositionedTile>;

pub struct Level<'a> {
    pub level_info: LevelInfo,
    pub name: &'a str,
    pub num_stars: u32,
}

pub type City<'a> = (&'a str, Vec<Level<'a>>);

pub struct LevelManager<'a>(Vec<City<'a>>);

fn convert_string_to_color(s: &str) -> Color {
    match s {
        "red" => Red,
        "blue" => Blue,
        "yellow" => Yellow,
        "purple" => Purple,
        "green" => Green,
        "orange" => Orange,
        "brown" => Brown,
        invalid => panic!("invalid color: {}", invalid),
    }
}

fn convert_string_to_dir(s: &str) -> usize {
    match s {
        "up" => 0,
        "right" => 1,
        "down" => 2,
        "left" => 3,
        invalid => panic!("invalid direction: {}", invalid),
    }
}

impl LevelManager<'_> {
    pub fn new() -> LevelManager<'static> {
        let mut lm = LevelManager(vec![]);
        let info_str = str::from_utf8(include_bytes!("../assets/levels.txt")).unwrap();
        let arr: Vec<&str> = info_str
            .split("\n")
            .filter(|line| !(line.starts_with("//") || line.is_empty()))
            .collect();
        let max_index = arr.len() - 1;
        let mut index = 0;
        'outer: loop {
            assert!(arr[index].starts_with("CITY:"));
            let city_name = &arr[index][5..];
            let mut city: City = (city_name, vec![]);
            index += 1;

            loop {
                // load all levels within a city
                let fields: Vec<&str> = arr[index].split(":").collect();
                let level_name = fields[0];
                let num_stars: u32 = fields[1].parse().unwrap();
                index += 1;

                let mut level = Level {
                    level_info: vec![],
                    name: level_name,
                    num_stars,
                };
                loop {
                    println!("{}", arr[index]);
                    let x: u8 = arr[index][2..3].parse().unwrap();
                    let y: u8 = arr[index][4..5].parse().unwrap();

                    if arr[index].starts_with("+ ") {
                        // handle a new trainsource
                        let fields: Vec<&str> = arr[index].split(" ").collect();
                        let colors: Vec<Color> =
                            fields[2].split(",").map(convert_string_to_color).collect();
                        let dir = convert_string_to_dir(fields[3]);
                        level.level_info.push(PositionedTile {
                            tile: Tile::Trainsource(Trainsource::new(colors, dir)),
                            x,
                            y,
                        });
                    } else if arr[index].starts_with("o ") {
                        // handle a new trainsink
                        let fields: Vec<&str> = arr[index].split(" ").collect();
                        let colors: Vec<Color> =
                            fields[2].split(",").map(convert_string_to_color).collect();
                        let dirs = fields[3].split(",").map(convert_string_to_dir);
                        let mut border_state = [false, false, false, false];
                        for dir in dirs {
                            border_state[dir] = true;
                        }
                        level.level_info.push(PositionedTile {
                            tile: Tile::Trainsink(Trainsink::new(colors, border_state)),
                            x,
                            y,
                        });
                    } else if arr[index].starts_with("* ") {
                        // handle a new rock
                        let mut positions = arr[index][2..].split(" ");
                        while let Some(position) = positions.next() {
                            let x: u8 = position[0..1].parse().unwrap();
                            let y: u8 = position[2..3].parse().unwrap();
                            level.level_info.push(PositionedTile {
                                tile: Tile::Rock,
                                x,
                                y,
                            });
                        }
                    } else if arr[index].starts_with("p ") {
                        // handle a new painter
                        let fields: Vec<&str> = arr[index].split(" ").collect();
                        let color = convert_string_to_color(fields[2]);
                        let dirs: Vec<usize> =
                            fields[3].split(",").map(convert_string_to_dir).collect();

                        level.level_info.push(PositionedTile {
                            tile: Tile::Painter(Painter::new(
                                Connection {
                                    dir1: dirs[0] as u8,
                                    dir2: dirs[1] as u8,
                                },
                                color,
                            )),
                            x,
                            y,
                        });
                    } else if arr[index].starts_with("s ") {
                        // handle a new splitter
                        let fields: Vec<&str> = arr[index].split(" ").collect();
                        let dir = convert_string_to_dir(fields[2]);
                        level.level_info.push(PositionedTile {
                            tile: Tile::Splitter(Splitter::new(dir)),
                            x,
                            y,
                        });
                    } else {
                        panic!("line begins with an invalid character {}", arr[index]);
                    }

                    index += 1;
                    if arr[index] == "---" {
                        break;
                    }
                }
                city.1.push(level);

                index += 1;
                if index > max_index {
                    lm.0.push(city);
                    break 'outer;
                }
                if arr[index].starts_with("CITY:") {
                    break;
                }
            }

            lm.0.push(city);

            // if index > max_index {
            //     break 'outer;
            // }
        }

        lm
    }

    pub fn get_city_names(&self) -> Vec<String> {
        self.0.iter().map(|city| city.0.to_owned()).collect()
    }
    pub fn get_names_of_city(&self, city_name: &str) -> Vec<String> {
        let city = self.0.iter().find(|city| city.0 == city_name).unwrap();
        city.1.iter().map(|level| level.name.to_owned()).collect()
    }
    pub fn get_level(&self, city_name: &str, level_name: &str) -> &LevelInfo {
        let city = self.0.iter().find(|city| city.0 == city_name).unwrap();
        &city
            .1
            .iter()
            .find(|level| level.name == level_name)
            .unwrap()
            .level_info
    }
}
