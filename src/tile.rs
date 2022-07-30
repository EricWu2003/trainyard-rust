use macroquad::prelude::*;
pub mod painter;
pub mod splitter;
pub mod tracktile;
pub mod trainsink;
pub mod trainsource;

use crate::color::Color;
use crate::tile::painter::Painter;
use crate::tile::splitter::Splitter;
use crate::tile::tracktile::Tracktile;
use crate::tile::trainsink::Trainsink;
use crate::tile::trainsource::Trainsource;
use crate::sprites::GameSprites;
use crate::particle::ParticleList;
use crate::particle::smoke::Smoke;


pub type BorderState = [Option<Color>; 4];

#[derive(Clone)]
pub enum Tile {
    Tracktile(Tracktile),
    Trainsource(Trainsource),
    Trainsink(Trainsink),
    Rock(Option<Rect>),
    Painter(Painter),
    Splitter(Splitter),
}

impl Tile {
    pub fn accept_trains(&mut self, trains: BorderState, p: &mut ParticleList) -> bool {
        let border_state: BorderState;
        match self {
            Tile::Tracktile(tracktile) => {
                border_state = tracktile.accept_trains(trains);
            }
            Tile::Trainsource(trainsource) => {
                border_state = trainsource.accept_trains(trains);
            }
            Tile::Trainsink(trainsink) => {
                border_state = trainsink.accept_trains(trains);
            }
            Tile::Rock(_) => {
                border_state = trains;
            }
            Tile::Painter(painter) => {
                border_state = painter.accept_trains(trains);
            }
            Tile::Splitter(splitter) => {
                border_state = splitter.accept_trains(trains);
            }
        }
        let mut no_crashes = true;
        let rect = self.get_rect();
        for (dir, train) in border_state.iter().enumerate() {
            if let Some(color) = train {
                no_crashes = false;
                match dir {
                    0 => p.push(Box::new(Smoke::new(
                        rect.x + rect.w / 2.,
                        rect.y,
                        *color,
                    ))),
                    1 => p.push(Box::new(Smoke::new(
                        rect.x + rect.w,
                        rect.y + rect.h / 2.,
                        *color,
                    ))),
                    2 => p.push(Box::new(Smoke::new(
                        rect.x + rect.w / 2.,
                        rect.y + rect.h,
                        *color,
                    ))),
                    3 => p.push(Box::new(Smoke::new(
                        rect.x,
                        rect.y + rect.h / 2.,
                        *color,
                    ))),
                    _ => unreachable!(),
                }
            }
        }


        return no_crashes;
    }
    pub fn dispatch_trains(&mut self) -> BorderState {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.dispatch_trains()
            }
            Tile::Trainsource(trainsource) => {
                trainsource.dispatch_trains()
            }
            Tile::Trainsink(trainsink) => {
                trainsink.dispatch_trains()
            }
            Tile::Rock(_) => {
                [None, None, None, None]
            }
            Tile::Painter(painter) => {
                painter.dispatch_trains()
            }
            Tile::Splitter(splitter) => {
                splitter.dispatch_trains()
            }
        }
    }
    pub fn process_end_of_tick(&mut self, gs: &mut GameSprites, p: &mut ParticleList) {
        // the tracktile Tile type is the only one which needs to process things at the end of each tick (merging trains)
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.interact_trains(gs, p);
            }
            _ => {}
        }
    }

    pub fn process_tick(&mut self, gs: &mut GameSprites, p: &mut ParticleList) {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.process_tick(gs, p);
            }
            Tile::Trainsource(trainsource) => {
                trainsource.process_tick(p);
            }
            Tile::Trainsink(trainsink) => {
                trainsink.process_tick(gs, p);
            }
            Tile::Painter(painter) => {
                painter.process_tick(gs, p)
            }
            Tile::Splitter(splitter) => {
                splitter.process_tick(gs, p)
            }
            Tile::Rock(_) => {}
        }
    }
    
    pub fn get_char(&self) -> char {
        // a deprecated method for rendering a tile to the console screen.
        match self {
            Tile::Tracktile(tracktile) => tracktile.connection_type().get_char(),
            Tile::Trainsource(_) => 'S',
            Tile::Trainsink(_) => 'S',
            Tile::Rock(_) => 'R',
            Tile::Painter(_) => 'P',
            Tile::Splitter(_) => 'C',
        }
    }

    pub fn render_trains(&self, gs: &GameSprites, progress: f32) {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.render_trains(gs, progress)
            }
            Tile::Trainsink(trainsink) => {
                trainsink.render_trains(gs, progress)
            }
            Tile::Trainsource(trainsource) => {
                trainsource.render_trains(gs, progress)
            }
            Tile::Painter(painter) => {
                painter.render_trains(gs, progress)
            }
            Tile::Splitter(splitter) => {
                splitter.render_trains(gs, progress)
            }
            Tile::Rock(_) => {}
        }
    }

    pub fn set_rect(&mut self, rect:Rect) {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.set_rect(rect);
            }
            Tile::Trainsink(trainsink) => {
                trainsink.set_rect(rect);
            }
            Tile::Trainsource(trainsource) => {
                trainsource.set_rect(rect);
            }
            Tile::Painter(painter) => {
                painter.set_rect(rect);
            }
            Tile::Splitter(splitter) => {
                splitter.set_rect(rect);
            }
            Tile::Rock(r) => {
                *r = Some(rect);
            }
        }
    }

    pub fn get_rect(&self) -> Rect {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.rect.unwrap()
            }
            Tile::Trainsink(trainsink) => {
                trainsink.rect.unwrap()
            }
            Tile::Trainsource(trainsource) => {
                trainsource.rect.unwrap()
            }
            Tile::Painter(painter) => {
                painter.rect.unwrap()
            }
            Tile::Splitter(splitter) => {
                splitter.rect.unwrap()
            }
            Tile::Rock(rect) => {
                rect.unwrap()
            }
        }
    }
}
