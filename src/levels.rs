use crate::color::Color::{Blue, Green, Orange, Purple, Red, Yellow};
use crate::tile::trainsink::Trainsink;
use crate::tile::trainsource::Trainsource;
use crate::tile::Tile;

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

impl LevelManager<'_> {
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

pub static mut LEVEL_MANAGER: LevelManager = LevelManager(Vec::new());
pub fn initialize() {
    let mut abbotsford: City = ("Abbotsford", Vec::new());
    let mut brampton: City = ("Brampton", Vec::new());
    let mut calgary: City = ("Calgary", Vec::new());
    let delson: City = ("Delson", Vec::new());
    let edmonton: City = ("Edmonton", Vec::new());

    {
        abbotsford.1.push(Level {
            level_info: vec![
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Red], 1)),
                    x: 1,
                    y: 3,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(vec![Red], [false, false, false, true])),
                    x: 5,
                    y: 3,
                },
            ],
            name: "Red Line",
            num_stars: 1,
        });
        abbotsford.1.push(Level {
            level_info: vec![
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Green], 2)),
                    x: 1,
                    y: 1,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(vec![Green], [true, false, false, false])),
                    x: 1,
                    y: 5,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Orange], 1)),
                    x: 2,
                    y: 5,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(
                        vec![Orange],
                        [false, false, false, true],
                    )),
                    x: 5,
                    y: 5,
                },
            ],
            name: "Grorange Lines",
            num_stars: 1,
        });
        abbotsford.1.push(Level {
            level_info: vec![
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Purple], 3)),
                    x: 6,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(
                        vec![Purple],
                        [false, true, false, false],
                    )),
                    x: 0,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Purple], 1)),
                    x: 0,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(
                        vec![Purple],
                        [false, false, false, true],
                    )),
                    x: 6,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Yellow], 2)),
                    x: 1,
                    y: 2,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(
                        vec![Yellow],
                        [true, false, false, false],
                    )),
                    x: 1,
                    y: 4,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Yellow], 0)),
                    x: 5,
                    y: 4,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(
                        vec![Yellow],
                        [false, false, true, false],
                    )),
                    x: 5,
                    y: 2,
                },
            ],
            name: "Yorple Lines",
            num_stars: 1,
        });
        // TODO: 3 more levels for Abbotsford
    }
    {
        brampton.1.push(Level {
            level_info: vec![
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Green], 0)),
                    x: 3,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(vec![Green], [false, false, true, false])),
                    x: 3,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Rock,
                    x: 3,
                    y: 3,
                },
            ],
            name: "A Rock in the Way",
            num_stars: 1,
        });
        brampton.1.push(Level {
            level_info: vec![
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Green], 2)),
                    x: 5,
                    y: 1,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(vec![Green], [false, false, true, false])),
                    x: 1,
                    y: 1,
                },
                PositionedTile {
                    tile: Tile::Rock,
                    x: 3,
                    y: 1,
                },
                PositionedTile {
                    tile: Tile::Rock,
                    x: 3,
                    y: 2,
                },
                PositionedTile {
                    tile: Tile::Rock,
                    x: 3,
                    y: 3,
                },
                PositionedTile {
                    tile: Tile::Rock,
                    x: 3,
                    y: 4,
                },
                PositionedTile {
                    tile: Tile::Rock,
                    x: 3,
                    y: 5,
                },
                PositionedTile {
                    tile: Tile::Rock,
                    x: 3,
                    y: 6,
                },
            ],
            name: "Green Wally",
            num_stars: 1,
        });
        // TODO: 3 more levels for Brampton
    }
    {
        calgary.1.push(Level {
            level_info: vec![
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Red], 0)),
                    x: 0,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(vec![Red], [false, false, true, false])),
                    x: 0,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Blue], 0)),
                    x: 2,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(vec![Blue], [false, false, true, false])),
                    x: 2,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Yellow], 0)),
                    x: 4,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(
                        vec![Yellow],
                        [false, false, true, false],
                    )),
                    x: 4,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Red], 0)),
                    x: 6,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(vec![Red], [false, false, true, false])),
                    x: 6,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Purple], 2)),
                    x: 1,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(
                        vec![Purple],
                        [true, false, false, false],
                    )),
                    x: 1,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Green], 2)),
                    x: 3,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(vec![Green], [true, false, false, false])),
                    x: 3,
                    y: 6,
                },
                PositionedTile {
                    tile: Tile::Trainsource(Trainsource::new(vec![Orange], 2)),
                    x: 5,
                    y: 0,
                },
                PositionedTile {
                    tile: Tile::Trainsink(Trainsink::new(
                        vec![Orange],
                        [true, false, false, false],
                    )),
                    x: 5,
                    y: 6,
                },
            ],
            name: "Rainbow",
            num_stars: 1,
        });
    }

    unsafe {
        LEVEL_MANAGER.0.push(abbotsford);
        LEVEL_MANAGER.0.push(brampton);
        LEVEL_MANAGER.0.push(calgary);
        LEVEL_MANAGER.0.push(delson);
        LEVEL_MANAGER.0.push(edmonton);
    }
}
