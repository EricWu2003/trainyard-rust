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
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::sprites::GameSprites;
use crate::particle::ParticleList;


pub type BorderState = [Option<Color>; 4];

#[derive(Clone)]
pub enum Tile {
    Tracktile(Tracktile),
    Trainsource(Trainsource),
    Trainsink(Trainsink),
    Rock,
    Painter(Painter),
    Splitter(Splitter),
}

impl Tile {
    pub fn accept_trains(&mut self, trains: BorderState) -> bool {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.accept_trains(trains)
            }
            Tile::Trainsource(trainsource) => {
                trainsource.accept_trains(trains)
            }
            Tile::Trainsink(trainsink) => {
                trainsink.accept_trains(trains)
            }
            Tile::Rock => {
                for train in trains {
                    if train.is_some() {
                        return false;
                    }
                }
                return true;
            }
            Tile::Painter(painter) => {
                painter.accept_trains(trains)
            }
            Tile::Splitter(splitter) => {
                splitter.accept_trains(trains)
            }
        }
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
            Tile::Rock => {
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
    pub fn process_end_of_tick(&mut self, gs: &GameSprites) {
        // the tracktile Tile type is the only one which needs to process things at the end of each tick (merging trains)
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.interact_trains(gs);
            }
            _ => {}
        }
    }

    pub fn process_tick(&mut self, gs: &GameSprites, p: &mut ParticleList) {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.process_tick(gs);
            }
            Tile::Trainsource(trainsource) => {
                trainsource.process_tick(p);
            }
            Tile::Trainsink(trainsink) => {
                trainsink.process_tick(gs, p);
            }
            Tile::Painter(painter) => {
                painter.process_tick(gs)
            }
            Tile::Splitter(splitter) => {
                splitter.process_tick(gs, p)
            }
            Tile::Rock => {}
        }
    }
    
    pub fn get_char(&self) -> char {
        // a deprecated method for rendering a tile to the console screen.
        match self {
            Tile::Tracktile(tracktile) => tracktile.connection_type().get_char(),
            Tile::Trainsource(_) => 'S',
            Tile::Trainsink(_) => 'S',
            Tile::Rock => 'R',
            Tile::Painter(_) => 'P',
            Tile::Splitter(_) => 'C',
        }
    }

    pub fn render_trains(&self, canvas: &mut WindowCanvas, rect: &Rect, gs: &mut GameSprites, progress: f64) -> Result<(), String> {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.render_trains(canvas, rect, gs, progress)?
            }
            Tile::Trainsink(trainsink) => {
                trainsink.render_trains(canvas, rect, gs, progress)?
            }
            Tile::Trainsource(trainsource) => {
                trainsource.render_trains(canvas, rect, gs, progress)?
            }
            Tile::Painter(painter) => {
                painter.render_trains(canvas, rect, gs, progress)?
            }
            Tile::Splitter(splitter) => {
                splitter.render_trains(canvas, rect, gs, progress)?
            }
            Tile::Rock => {}
        }
        Ok(())
    }

    pub fn set_rect(&mut self, rect:Rect) {
        match self {
            Tile::Tracktile(_tracktile) => {
                // tracktile.set_rect(rect);
            }
            Tile::Trainsink(trainsink) => {
                trainsink.set_rect(rect);
            }
            Tile::Trainsource(trainsource) => {
                trainsource.set_rect(rect);
            }
            Tile::Painter(_painter) => {
                // painter.set_rect(rect);
            }
            Tile::Splitter(splitter) => {
                splitter.set_rect(rect);
            }
            Tile::Rock => {}
        }
    }
}
