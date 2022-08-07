use macroquad::prelude::*;
use crate::connection::Connection;
use crate::edge::Edge;
use crate::levels::{LevelInfo, Level, PositionedTile};
use crate::particle::ParticleList;
use crate::particle::drawn_arrow::DrawnArrow;
use crate::particle::smoke::Smoke;
use crate::sprites::GameSprites;
use crate::sprites::SoundType::{EraseTrack, Crash, WinLevel};
use crate::tile::tracktile::ConnectionType;
use crate::tile::tracktile::Tracktile;
use crate::tile::BorderState;
use crate::tile::Tile;

use std::io::Write;
use std::f32::consts::PI;

pub const NUM_ROWS: usize = 7;
pub const NUM_COLS: usize = 7;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NextAction {
    ProcessTick,
    ProcessEdges,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum YardState {
    Drawing,
    Playing {
        num_ticks_elapsed: u32,
        progress: f32,
        next_step: NextAction,
    },
    Crashed,
    Won,
}

// In the Yard struct, we keep a copy of tiles and drawn_tiles. When the user is drawing, we update both.
// When the yard is in the Playing state, we only update tiles (and keep drawn_tiles static) when trains switch
// active/passive connections. This way we can use drawn_tiles to revert the tiles if the user 
// returns to the drawing board.
pub struct Yard {
    tiles: Vec<Vec<Tile>>,
    drawn_tiles: Vec<Vec<Tile>>,
    h_edges: Vec<Vec<Edge>>,
    v_edges: Vec<Vec<Edge>>,
    pub state: YardState,
    level_info: LevelInfo,
    pub rect: Rect,
}

impl Yard {
    pub fn new_blank(rect: Rect) -> Yard {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for _ in 0..NUM_ROWS {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..NUM_COLS {
                row.push(Tile::Tracktile(Tracktile::new(None, None)));
            }
            tiles.push(row);
        }
        let drawn_tiles = tiles.clone();

        let w = rect.w/NUM_COLS as f32;
        let h = rect.h/NUM_ROWS as f32;
        let half_w = w/2.;
        let half_h = h/2.;

        let mut h_edges: Vec<Vec<Edge>> = Vec::new();
        for r in 0..(NUM_ROWS + 1) {
            let mut row: Vec<Edge> = Vec::new();
            for c in 0..NUM_COLS {
                let x_pos = rect.x + half_w + c as f32 * w;
                let y_pos = rect.y + r as f32 * h;
                row.push(Edge::new(x_pos, y_pos));
            }
            h_edges.push(row);
        }

        let mut v_edges: Vec<Vec<Edge>> = Vec::new();
        for r in 0..NUM_ROWS {
            let mut row: Vec<Edge> = Vec::new();
            for c in 0..(NUM_COLS + 1) {
                let x_pos = rect.x + c as f32 * w;
                let y_pos = rect.y + half_h + r as f32 * h;
                row.push(Edge::new(x_pos, y_pos));
            }
            v_edges.push(row);
        }

        Yard {
            tiles,
            drawn_tiles,
            h_edges,
            v_edges,
            state: YardState::Drawing,
            level_info: vec![],
            rect,
        }
    }
    pub fn new(level: &Level, rect: Rect, gs: &GameSprites) -> Yard {
        let mut yard = Yard::new_blank(rect);
        let level_info = &level.level_info;
        for i in 0..level_info.len() {
            let tile = &level_info[i];
            yard.tiles[tile.y as usize][tile.x as usize] = tile.tile.clone();
        }
        let current_progress = &level.current_progress;
        for i in 0..current_progress.len() {
            let tile = &current_progress[i];
            match yard.tiles[tile.y as usize][tile.x as usize] {
                Tile::Tracktile(_) => {},
                _ => {
                    println!("Warning, overriding a non tracktile tile at column {}, row {} when loading level progress.", tile.x, tile.y)
                }
            }
            yard.tiles[tile.y as usize][tile.x as usize] = tile.tile.clone();
        }
        yard.drawn_tiles = yard.tiles.clone();

        let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);
        let new_w = w/(NUM_COLS as f32);
        let new_h = h/(NUM_ROWS as f32);
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                yard.tiles[row][col].set_rect(
                    Rect::new(
                        x + (new_w*col as f32),
                        y + (new_h*row as f32),
                        new_w,
                        new_h,
                    ),
                    gs,
                )
            }
        }
        yard.level_info = level_info.clone();
        yard
    }

    pub fn clear_connections (&mut self, r: usize, c: usize, gs:&mut GameSprites) {
        if let Tile::Tracktile(tracktile) = &mut self.tiles[r][c] {
            if tracktile.connection_type() != ConnectionType::None {
                gs.add_sound(EraseTrack);
            }
            tracktile.clear_connections();
        }
        if let Tile::Tracktile(tracktile) = &mut self.drawn_tiles[r][c] {
            tracktile.clear_connections();
        }
    }

    pub fn display(&self) {
        // deprecated, used for displaying stuff to the terminal.
        for r in 0..NUM_ROWS {
            print!(" ");
            for c in 0..NUM_COLS {
                print!("{}", self.h_edges[r][c].get_char().to_string() + " ");
            }
            println!("");
            for c in 0..NUM_COLS {
                print!(
                    "{}",
                    self.v_edges[r][c].get_char().to_string()
                        + &self.tiles[r][c].get_char().to_string()
                );
            }
            println!("{}", self.v_edges[r][NUM_ROWS].get_char());
        }

        print!(" ");
        for c in 0..NUM_COLS {
            print!("{}", self.h_edges[NUM_ROWS][c].get_char().to_string() + " ");
        }
        println!("");

        std::io::stdout().flush().unwrap();
    }

    pub fn add_connection(&mut self, r: usize, c: usize, conn: Connection, gs: &mut GameSprites, p: &mut ParticleList) {
        // we only allow a yard to add_connection during the drawing state.
        assert!(matches!(self.state, YardState::Drawing));
        if let Tile::Tracktile(tt) = &mut self.tiles[r][c] {
            tt.add_connection(conn, gs);

            let tracktile_width = self.rect.w/NUM_COLS as f32;
            let tracktile_height = self.rect.h/NUM_ROWS as f32;

            let center_x = self.rect.x + tracktile_width/2. + tracktile_width * c as f32;
            let center_y = self.rect.y + tracktile_height/2. + tracktile_height * r as f32;
            let scale = tracktile_width / gs.tracktile_blank.width();

            if conn.contains(0) {
                p.push(Box::new(DrawnArrow::new(
                    center_x, 
                    center_y - tracktile_height/2., 
                    0,
                    scale,
                )));
            }
            if conn.contains(2) {
                p.push(Box::new(DrawnArrow::new(
                    center_x, 
                    center_y + tracktile_height/2., 
                    2,
                    scale,
                )));
            }
            if conn.contains(1) {
                p.push(Box::new(DrawnArrow::new(
                    center_x + tracktile_width/2., 
                    center_y,
                    1,
                    scale,
                )));
            }
            if conn.contains(3) {
                p.push(Box::new(DrawnArrow::new(
                    center_x - tracktile_width/2., 
                    center_y,
                    3,
                    scale,
                )));
            }

            if let Tile::Tracktile(tt_drawn) = &mut self.drawn_tiles[r][c] {
                tt_drawn.add_connection(conn, gs);
            }
        }
    }

    pub fn switch_connections(&mut self, r:usize, c:usize, gs: &mut GameSprites) {
        // we only allow a yard to manually switch connections during the drawing state.
        // during a playing state each tracktile is responsible for switching itself.
        assert!(matches!(self.state, YardState::Drawing));
        if let Tile::Tracktile(tt) = &mut self.tiles[r][c] {
            tt.switch_active_passive(gs);
            if let Tile::Tracktile(tt_drawn) = &mut self.drawn_tiles[r][c]{
                tt_drawn.switch_active_passive(gs);
            }
        }
    }
    
    pub fn reset_self(&mut self, gs: &GameSprites) {
        // used to recover from a crashed state back to a drawing state.
        // also used when the user presses "back to drawing board".

        for tile in &self.level_info {
            self.tiles[tile.y as usize][tile.x as usize] = tile.tile.clone();
        }

        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                if let Tile::Tracktile(tracktile) = &mut self.tiles[r][c] {
                    tracktile.clear_trains();
                }
            }
        }

        for r in 0..(NUM_ROWS + 1) {
            for c in 0..NUM_COLS {
                self.h_edges[r][c].clear_trains();
            }
        }
        for r in 0..NUM_ROWS {
            for c in 0..(NUM_COLS + 1) {
                self.v_edges[r][c].clear_trains();
            }
        }

        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                if let Tile::Tracktile(tracktile) = &self.drawn_tiles[r][c] {
                    if tracktile.connection_type() != ConnectionType::None {
                        // we only do this if the connection_type is not none because otherwise we would
                        // overwrite other tiles like trainsources.
                        self.tiles[r][c] = Tile::Tracktile(tracktile.clone());
                    }
                }
            }
        }

        let rect = self.rect;
        let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);
        let new_w = w/(NUM_COLS as f32);
        let new_h = h/(NUM_ROWS as f32);
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                self.tiles[row][col].set_rect(
                    Rect::new(
                        x + (new_w*col as f32),
                        y + (new_h*row as f32),
                        new_w,
                        new_h,
                    ),
                    gs,
                )
            }
        }
    }

    pub fn process_edges(&mut self, gs: &mut GameSprites, p: &mut ParticleList) {
        assert!(matches!(
            self.state,
            YardState::Playing {..}
        ));

        let tracktile_width = self.rect.w/NUM_COLS as f32;
        let tracktile_height = self.rect.h/NUM_ROWS as f32;
        let scale = tracktile_width / gs.tracktile_blank.width();

        // merge all trains that are still in tiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                self.tiles[r][c].process_end_of_tick(gs, p);
            }
        }


        // dispatch all trains and store them in edges.
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                [
                    self.h_edges[r][c].train_to_a,
                    self.v_edges[r][c + 1].train_to_b,
                    self.h_edges[r + 1][c].train_to_b,
                    self.v_edges[r][c].train_to_a,
                ] = self.tiles[r][c].dispatch_trains();
            }
        }
        // mix edges
        for r in 0..(NUM_ROWS + 1) {
            for c in 0..NUM_COLS {
                self.h_edges[r][c].interact_trains(gs, p, scale);
            }
        }
        for r in 0..NUM_ROWS {
            for c in 0..(NUM_COLS + 1) {
                self.v_edges[r][c].interact_trains(gs, p, scale);
            }
        }

        // detect crashes on boundaries of yard (i.e. if a train is about to crash by going
        // too far up where there is no tile left to catch it)
        for c in 0..NUM_COLS {
            if let Some(train) = self.h_edges[0][c].train_to_a {
                self.state = YardState::Crashed;
                p.push(Box::new(Smoke::new(
                    self.rect.x + tracktile_width /2. + (tracktile_width * c as f32),
                    self.rect.y,
                    train,
                    scale,
                )));
                gs.add_sound(Crash);
            }
            if let Some(train) = self.h_edges[NUM_ROWS][c].train_to_b {
                self.state = YardState::Crashed;
                p.push(Box::new(Smoke::new(
                    self.rect.x + tracktile_width /2. + (tracktile_width * c as f32),
                    self.rect.y + self.rect.h,
                    train,
                    scale,
                )));
                gs.add_sound(Crash);
            }
        }
        for r in 0..NUM_ROWS {
            if let Some(train) = self.v_edges[r][0].train_to_a {
                self.state = YardState::Crashed;
                p.push(Box::new(Smoke::new(
                    self.rect.x,
                    self.rect.y + tracktile_height /2. + (tracktile_height * r as f32),
                    train,
                    scale,
                )));
                gs.add_sound(Crash);
            }
            if let Some(train) = self.v_edges[r][NUM_COLS].train_to_b {
                self.state = YardState::Crashed;
                p.push(Box::new(Smoke::new(
                    self.rect.x + self.rect.w,
                    self.rect.y + tracktile_height /2. + (tracktile_height * r as f32),
                    train,
                    scale,
                )));
                gs.add_sound(Crash);
            }
        }

        // all tiles pull in trains from the edges. A crash occurs if there is a
        // train entering a tile but the tile does not pull it in.
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                let border_state: BorderState = [
                    self.h_edges[r][c].train_to_b,
                    self.v_edges[r][c + 1].train_to_a,
                    self.h_edges[r + 1][c].train_to_a,
                    self.v_edges[r][c].train_to_b,
                ];
                let not_crashed = self.tiles[r][c].accept_trains(border_state, p, scale);
                if !not_crashed {
                    self.state = YardState::Crashed;
                    gs.add_sound(Crash);
                }
            }
        }

        
    }

    pub fn process_tick(&mut self, gs: &mut GameSprites, p: &mut ParticleList) {
        assert!(matches!(
            self.state,
            YardState::Playing {..}
        ));
        

        // then process tick in all tiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                self.tiles[r][c].process_tick(gs, p);
            }
        }

        if self.has_won() {
            self.state = YardState::Won;
            gs.add_sound(WinLevel);
        }
    }

    pub fn update(&mut self, speed: f32, gs: &mut GameSprites, p: &mut ParticleList) {
        if let YardState::Playing {
            mut num_ticks_elapsed,
            mut progress,
            mut next_step,
        } = self.state
        {
            progress += speed;
            if progress > 1.0 {
                progress -= 1.0;
                match next_step {
                    NextAction::ProcessEdges => {
                        self.process_edges(gs, p);
                        next_step = NextAction::ProcessTick;
                    }
                    NextAction::ProcessTick => {
                        self.process_tick(gs, p);
                        next_step = NextAction::ProcessEdges;

                    }
                }

                num_ticks_elapsed += 1;
            }
            if self.state != YardState::Crashed && self.state != YardState::Won {
                self.state = YardState::Playing {
                    num_ticks_elapsed,
                    progress,
                    next_step,
                }
            }
        } else if self.state == YardState::Crashed {
            for r in 0..NUM_ROWS {
                for c in 0..NUM_COLS {
                    if let Tile::Trainsink(trainsink) = &mut self.tiles[r][c] {
                        // this only exists for the edge case where two trains simultaneously enter a trainsink with only 1 desire.
                        // in that case, one train enters, the other crashes.
                        trainsink.process_tick(gs, p);
                    }
                }
            }
        }
    }

    pub fn has_won(&self) -> bool {
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                match &self.tiles[r][c] {
                    Tile::Trainsink(trainsink) => {
                        if !trainsink.is_satisfied() {
                            return false;
                        }
                    }
                    Tile::Trainsource(trainsource) => {
                        if !trainsource.is_empty() {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }
        for r in 0..(NUM_ROWS + 1) {
            for c in 0..NUM_COLS {
                if !self.h_edges[r][c].is_empty() {
                    return false;
                }
            }
        }
        for r in 0..NUM_ROWS {
            for c in 0..(NUM_COLS + 1) {
                if !self.v_edges[r][c].is_empty() {
                    return false;
                }
            }
        }
        true
    }

    pub fn render(
        &self,
        gs: &GameSprites,
    ) {
        let rect = self.rect;

        let block_width = rect.w / (NUM_COLS as f32);
        let block_height = rect.h / (NUM_ROWS as f32);

        let dest_size = Some(Vec2::new(block_width, block_height));

        let x0 = rect.x;
        let y0 = rect.y;

        //render all tracktiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                let x_pos = x0 + c as f32 * block_width;
                let y_pos = y0 + r as f32 * block_height;

                let mut texture = gs.tracktile_blank;
                let mut h_flip = false;
                let mut rot = 0;

                match &self.tiles[r][c] {
                    Tile::Tracktile(tracktile) => {
                        match tracktile.connection_type() {
                            ConnectionType::None => {}
                            ConnectionType::B => {
                                texture = gs.tracktile_b;
                                rot = tracktile
                                    .has_connection_up_to_rot(Connection { dir1: 2, dir2: 3 });
                            }
                            ConnectionType::S => {
                                texture = gs.tracktile_s;
                                rot = tracktile
                                    .has_connection_up_to_rot(Connection { dir1: 0, dir2: 2 });
                            }
                            ConnectionType::H => {
                                texture = gs.tracktile_h;
                                rot = tracktile.has_active_connection_up_to_rot(Connection {
                                    dir1: 0,
                                    dir2: 2,
                                });
                            }
                            ConnectionType::Z => {
                                texture = gs.tracktile_z;
                                rot = tracktile
                                    .has_connection_up_to_rot(Connection { dir1: 0, dir2: 1 });
                            }
                            ConnectionType::M => {
                                texture = gs.tracktile_m;
                                if tracktile.has_active_passive_connections_up_to_rot(
                                    Connection { dir1: 2, dir2: 3 },
                                    Connection { dir1: 1, dir2: 2 },
                                ) != -1
                                {
                                    h_flip = false;
                                    rot = tracktile.has_active_passive_connections_up_to_rot(
                                        Connection { dir1: 2, dir2: 3 },
                                        Connection { dir1: 1, dir2: 2 },
                                    );
                                } else {
                                    h_flip = true;
                                    rot = tracktile.has_active_passive_connections_up_to_rot(
                                        Connection { dir1: 1, dir2: 2 },
                                        Connection { dir1: 2, dir2: 3 },
                                    );
                                }
                            }
                            ConnectionType::J => {
                                if tracktile.has_active_connection_up_to_rot(Connection {
                                    dir1: 0,
                                    dir2: 1,
                                }) != -1
                                {
                                    texture = gs.tracktile_jb;
                                } else {
                                    texture = gs.tracktile_js;
                                }
                                if tracktile.has_connections_up_to_rot(
                                    Connection { dir1: 0, dir2: 2 },
                                    Connection { dir1: 3, dir2: 2 },
                                ) != -1
                                {
                                    h_flip = false;
                                    rot = tracktile.has_connections_up_to_rot(
                                        Connection { dir1: 0, dir2: 2 },
                                        Connection { dir1: 3, dir2: 2 },
                                    )
                                } else {
                                    h_flip = true;
                                    rot = tracktile.has_connections_up_to_rot(
                                        Connection { dir1: 0, dir2: 2 },
                                        Connection { dir1: 1, dir2: 2 },
                                    )
                                }
                            }
                        }
                        draw_texture_ex(
                            texture,
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: rot as f32 * PI/2.,
                                flip_x: h_flip,
                                flip_y: false,
                                pivot: None
                            }
                        );
                    }
                    Tile::Trainsource(trainsource) => {
                        draw_texture_ex(
                            gs.trainsource_exit,
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: trainsource.dir as f32 * PI/2.,
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                    }
                    Tile::Trainsink(trainsink) => {
                        draw_texture_ex(
                            gs.tracktile_blank,
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: 0.,
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                        if !trainsink.is_satisfied() {
                            for dir in 0..4 {
                                if trainsink.border_state[dir] {
                                    draw_texture_ex(
                                        gs.trainsink_entry,
                                        x_pos,
                                        y_pos,
                                        WHITE,
                                        DrawTextureParams { 
                                            dest_size,
                                            source: None,
                                            rotation: dir as f32 * PI/2.,
                                            flip_x: false,
                                            flip_y: false,
                                            pivot: None
                                        }
                                    );
                                }
                            }
                        }
                    }
                    Tile::Rock(_) => {
                        draw_texture_ex(
                            gs.rock,
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: 0.,
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                    }
                    Tile::Painter(painter) => {
                        draw_texture_ex(
                            gs.tracktile_blank,
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: 0.,
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                        draw_texture_ex(
                            gs.trainsink_entry,
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: painter.connection.dir1 as f32 * PI/2.,
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                        draw_texture_ex(
                            gs.trainsink_entry,
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: painter.connection.dir2 as f32 * PI/2.,
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                    }
                    Tile::Splitter(splitter) => {
                        draw_texture_ex(
                            gs.splitter_bg,
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: splitter.incoming_dir as f32 * PI/2.,
                                flip_x: false,
                                flip_y: false,
                                pivot: None
                            }
                        );
                    }
                }
            }
        }


        let current_progress: f32;
        if let YardState::Playing{progress, next_step, ..} = self.state {
            // we do this to account for differences in how each tile's render methods expect progress to be passed in,
            // and how the yard struct stores progress.
            // progress is how yard stores it. current_progress is how each tile expects it.
            if next_step == NextAction::ProcessTick {
                current_progress = progress;
            } else {
                current_progress = progress + 1.0;
            }
        } else {
            current_progress = 0.0;
        }

        //render all trains on tracktiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                self.tiles[r][c].render_trains(gs, current_progress);
            }
        }


        //render non tracktile tiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                let x_pos = x0 + c as f32 * block_width;
                let y_pos = y0 + r as f32 * block_height;

                match &self.tiles[r][c] {
                    Tile::Tracktile(_) => {}
                    Tile::Trainsource(trainsource) => {
                        draw_texture_ex(
                            gs.source_sink_border, 
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: 0.0,
                                flip_x: false,
                                flip_y: false,
                                pivot: None,
                            }
                        );
                        for i in 0..trainsource.trains.len() {
                            if let Some(color) = trainsource.trains[i] {
                                let (x_pos, y_pos) = (trainsource.icon_rects[i].x, trainsource.icon_rects[i].y);
                                let dest_size = Some(Vec2::new(trainsource.icon_rects[i].w, trainsource.icon_rects[i].h));
                                draw_texture_ex(
                                    gs.plus_sign, 
                                    x_pos,
                                    y_pos,
                                    color.get_color(),
                                    DrawTextureParams { 
                                        dest_size,
                                        source: None,
                                        rotation: 0.0,
                                        flip_x: false,
                                        flip_y: false,
                                        pivot: None,
                                    }
                                );
                            }
                        }
                    }
                    Tile::Trainsink(trainsink) => {
                        if !trainsink.is_satisfied() {
                            draw_texture_ex(
                                gs.source_sink_border, 
                                x_pos,
                                y_pos,
                                WHITE,
                                DrawTextureParams { 
                                    dest_size,
                                    source: None,
                                    rotation: 0.0,
                                    flip_x: false,
                                    flip_y: false,
                                    pivot: None,
                                }
                            );
                            for i in 0..trainsink.desires.len() {
                                if let Some(color) = trainsink.desires[i] {
                                    let (x_pos, y_pos) = (trainsink.icon_rects[i].x, trainsink.icon_rects[i].y);
                                    let dest_size = Some(Vec2::new(trainsink.icon_rects[i].w, trainsink.icon_rects[i].h));
                                    draw_texture_ex(
                                        gs.circle, 
                                        x_pos,
                                        y_pos,
                                        color.get_color(),
                                        DrawTextureParams { 
                                            dest_size,
                                            source: None,
                                            rotation: 0.0,
                                            flip_x: false,
                                            flip_y: false,
                                            pivot: None,
                                        }
                                    );
                                }
                            }
                        } else {
                            draw_texture_ex(
                                gs.sink_satisfied, 
                                x_pos,
                                y_pos,
                                WHITE,
                                DrawTextureParams { 
                                    dest_size,
                                    source: None,
                                    rotation: 0.0,
                                    flip_x: false,
                                    flip_y: false,
                                    pivot: None,
                                }
                            );
                        }
                    }
                    Tile::Rock(_) => {}
                    Tile::Painter(painter) => {
                        draw_texture_ex(
                            gs.painter_bg, 
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: 0.0,
                                flip_x: false,
                                flip_y: false,
                                pivot: None,
                            }
                        );
                        draw_texture_ex(
                            gs.painter_brush, 
                            x_pos,
                            y_pos,
                            painter.color.get_color(),
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: 0.0,
                                flip_x: false,
                                flip_y: false,
                                pivot: None,
                            }
                        );
                    }
                    Tile::Splitter(splitter) => {
                        draw_texture_ex(
                            gs.splitter, 
                            x_pos,
                            y_pos,
                            WHITE,
                            DrawTextureParams { 
                                dest_size,
                                source: None,
                                rotation: splitter.incoming_dir as f32 * PI/2.,
                                flip_x: false,
                                flip_y: false,
                                pivot: None,
                            }
                        );
                    }
                }
            }
        }
    }

    pub fn set_rect(&mut self, rect: Rect, gs: &GameSprites) {
        self.rect = rect;
        let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);
        let w = w/(NUM_COLS as f32);
        let h = h/(NUM_ROWS as f32);
        let half_w = w/2.;
        let half_h = h/2.;
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                self.tiles[row][col].set_rect(
                    Rect::new(
                        x + (w*col as f32),
                        y + (h*row as f32),
                        w,
                        h,
                    ),
                    gs,
                )
            }
        }

        for r in 0..(NUM_ROWS + 1) {
            for c in 0..NUM_COLS {
                let x_pos = rect.x + half_w + c as f32 * w;
                let y_pos = rect.y + r as f32 * h;
                self.h_edges[r][c].set_pos(x_pos, y_pos);
            }
        }

        for r in 0..NUM_ROWS {
            for c in 0..(NUM_COLS + 1) {
                let x_pos = rect.x + c as f32 * w;
                let y_pos = rect.y + half_h + r as f32 * h;
                self.v_edges[r][c].set_pos(x_pos, y_pos);
            }
        }
    }

    pub fn get_current_progress(&self) -> LevelInfo {
        let mut connection_vec = vec![];
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                if let Tile::Tracktile(tracktile) = &self.drawn_tiles[r][c] {
                    if tracktile.connection_type() != ConnectionType::None {
                        connection_vec.push(PositionedTile{
                            tile: Tile::Tracktile(tracktile.clone()),
                            x: c as u8,
                            y: r as u8,
                        });
                    }
                }
            }
        }
        connection_vec
    }
}
