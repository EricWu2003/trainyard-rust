use crate::connection::Connection;
use crate::edge::Edge;
use crate::levels::LevelInfo;
use crate::sprites::GameSprites;
use crate::tile::tracktile::ConnectionType;
use crate::tile::tracktile::Tracktile;
use crate::tile::BorderState;
use crate::tile::Tile;

use std::io::Write;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

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
        progress: f64,
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
}

impl Yard {
    pub fn new_blank() -> Yard {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for _ in 0..NUM_ROWS {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..NUM_COLS {
                row.push(Tile::Tracktile(Tracktile::new(None, None)));
            }
            tiles.push(row);
        }
        let drawn_tiles = tiles.clone();

        let mut h_edges: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..(NUM_ROWS + 1) {
            let mut row: Vec<Edge> = Vec::new();
            for _ in 0..NUM_COLS {
                row.push(Edge::new());
            }
            h_edges.push(row);
        }

        let mut v_edges: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..NUM_ROWS {
            let mut row: Vec<Edge> = Vec::new();
            for _ in 0..(NUM_COLS + 1) {
                row.push(Edge::new());
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
        }
    }
    pub fn new(level_info: &LevelInfo) -> Yard {
        let mut yard = Yard::new_blank();
        for i in 0..level_info.len() {
            let tile = &level_info[i];
            yard.tiles[tile.y as usize][tile.x as usize] = tile.tile.clone();
        }
        yard.level_info = level_info.clone();
        yard
    }

    pub fn clear_connections (&mut self, r: usize, c: usize, gs:&GameSprites) {
        if let Tile::Tracktile(tracktile) = &mut self.tiles[r][c] {
            if tracktile.connection_type() != ConnectionType::None {
                gs.sl.play(&gs.sl_erase_track);
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

    pub fn add_connection(&mut self, r: usize, c: usize, conn: Connection, gs: &GameSprites) {
        // we only allow a yard to add_connection during the drawing state.
        assert!(matches!(self.state, YardState::Drawing));
        if let Tile::Tracktile(tt) = &mut self.tiles[r][c] {
            tt.add_connection(conn, gs);
            if let Tile::Tracktile(tt_drawn) = &mut self.drawn_tiles[r][c] {
                tt_drawn.add_connection(conn, gs);
            }
        }
    }

    pub fn switch_connections(&mut self, r:usize, c:usize, gs: &GameSprites) {
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
    
    pub fn reset_self(&mut self) {
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
    }

    pub fn process_edges(&mut self, gs: &GameSprites) {
        assert!(matches!(
            self.state,
            YardState::Playing {..}
        ));

        // merge all trains that are still in tiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                self.tiles[r][c].process_end_of_tick(gs);
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
                self.h_edges[r][c].interact_trains(gs);
            }
        }
        for r in 0..NUM_ROWS {
            for c in 0..(NUM_COLS + 1) {
                self.v_edges[r][c].interact_trains(gs);
            }
        }

        // detect crashes on boundaries of yard (i.e. if a train is about to crash by going
        // too far up where there is no tile left to catch it)
        for c in 0..NUM_COLS {
            if let Some(_train) = self.h_edges[0][c].train_to_a {
                self.state = YardState::Crashed;
                gs.sl.play(&gs.sl_crash);
            }
            if let Some(_train) = self.h_edges[NUM_ROWS][c].train_to_b {
                self.state = YardState::Crashed;
                gs.sl.play(&gs.sl_crash);
            }
        }
        for r in 0..NUM_ROWS {
            if let Some(_train) = self.v_edges[r][0].train_to_a {
                self.state = YardState::Crashed;
                gs.sl.play(&gs.sl_crash);
            }
            if let Some(_train) = self.v_edges[r][NUM_COLS].train_to_b {
                self.state = YardState::Crashed;
                gs.sl.play(&gs.sl_crash);
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
                let not_crashed = self.tiles[r][c].accept_trains(border_state);
                if !not_crashed {
                    self.state = YardState::Crashed;
                    gs.sl.play(&gs.sl_crash);
                }
            }
        }

        
    }

    pub fn process_tick(&mut self, gs: &GameSprites) {
        assert!(matches!(
            self.state,
            YardState::Playing {..}
        ));
        

        // then process tick in all tiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                self.tiles[r][c].process_tick(gs);
            }
        }

        if self.has_won() {
            self.state = YardState::Won;
            gs.sl.play(&gs.sl_win_level);
        }
    }

    pub fn update(&mut self, speed: f64, gs: &GameSprites) {
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
                        self.process_edges(gs);
                        next_step = NextAction::ProcessTick;
                    }
                    NextAction::ProcessTick => {
                        self.process_tick(gs);
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
                        trainsink.process_tick(gs);
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
        canvas: &mut WindowCanvas,
        rect: &Rect,
        gs: &mut GameSprites,
    ) -> Result<(), String> {
        let block_width = (rect.width() / (NUM_COLS as u32)) as i32;
        let block_height = (rect.height() / (NUM_ROWS as u32)) as i32;
        
        let plus_sign_width = (block_width as f64 * (52.0 / 96.0)) as i32;
        let plus_sign_height = (block_height as f64 * (52.0 / 96.0)) as i32;
        let x0 = rect.x();
        let y0 = rect.y();

        //render all tracktiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                let rect = Rect::new(
                    x0 + c as i32 * block_width,
                    y0 + r as i32 * block_height,
                    block_width as u32,
                    block_height as u32,
                );
                let mut texture = &gs.tracktile_blank;
                let mut h_flip = false;
                let mut rot = 0;

                match &self.tiles[r][c] {
                    Tile::Tracktile(tracktile) => {
                        match tracktile.connection_type() {
                            ConnectionType::None => {}
                            ConnectionType::B => {
                                texture = &gs.tracktile_b;
                                rot = tracktile
                                    .has_connection_up_to_rot(Connection { dir1: 2, dir2: 3 });
                            }
                            ConnectionType::S => {
                                texture = &gs.tracktile_s;
                                rot = tracktile
                                    .has_connection_up_to_rot(Connection { dir1: 0, dir2: 2 });
                            }
                            ConnectionType::H => {
                                texture = &gs.tracktile_h;
                                rot = tracktile.has_active_connection_up_to_rot(Connection {
                                    dir1: 0,
                                    dir2: 2,
                                });
                            }
                            ConnectionType::Z => {
                                texture = &gs.tracktile_z;
                                rot = tracktile
                                    .has_connection_up_to_rot(Connection { dir1: 0, dir2: 1 });
                            }
                            ConnectionType::M => {
                                texture = &gs.tracktile_m;
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
                                    texture = &gs.tracktile_jb;
                                } else {
                                    texture = &gs.tracktile_js;
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
                        canvas.copy_ex(
                            &gs.atlas,
                            *texture,
                            rect,
                            rot as f64 * 90.0,
                            None,
                            h_flip,
                            false,
                        )?;
                    }
                    Tile::Trainsource(trainsource) => {
                        canvas.copy_ex(
                            &gs.atlas,
                            gs.trainsource_exit,
                            rect,
                            trainsource.dir as f64 * 90.0,
                            None,
                            false,
                            false,
                        )?;
                    }
                    Tile::Trainsink(trainsink) => {
                        canvas.copy(&gs.atlas, gs.tracktile_blank, rect)?;
                        if !trainsink.is_satisfied() {
                            for dir in 0..4 {
                                if trainsink.border_state[dir] {
                                    canvas.copy_ex(
                                        &gs.atlas,
                                        gs.trainsink_entry,
                                        rect,
                                        dir as f64 * 90.0,
                                        None,
                                        false,
                                        false,
                                    )?;
                                }
                            }
                        }
                    }
                    Tile::Rock => {
                        canvas.copy(&gs.atlas, gs.rock, rect)?;
                    }
                    Tile::Painter(painter) => {
                        canvas.copy(&gs.atlas, gs.tracktile_blank, rect)?;
                        canvas.copy_ex(
                            &gs.atlas,
                            gs.trainsink_entry,
                            rect,
                            painter.connection.dir1 as f64 * 90.0,
                            None,
                            false,
                            false,
                        )?;
                        canvas.copy_ex(
                            &gs.atlas,
                            gs.trainsink_entry,
                            rect,
                            painter.connection.dir2 as f64 * 90.0,
                            None,
                            false,
                            false,
                        )?;
                    }
                    Tile::Splitter(splitter) => {
                        canvas.copy_ex(
                            &gs.atlas,
                            gs.splitter_bg,
                            rect,
                            splitter.incoming_dir as f64 * 90.0,
                            None,
                            false,
                            false,
                        )?;
                    }
                }
            }
        }


        let current_progress: f64;
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
                let rect = Rect::new(
                    x0 + c as i32 * block_width,
                    y0 + r as i32 * block_height,
                    block_width as u32,
                    block_height as u32,
                );

                self.tiles[r][c].render_trains(
                    canvas,
                    &rect,
                    gs,
                    current_progress
                )?;
            }
        }


        //render non tracktile tiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                let rect = Rect::new(
                    x0 + c as i32 * block_width,
                    y0 + r as i32 * block_height,
                    block_width as u32,
                    block_height as u32,
                );

                match &self.tiles[r][c] {
                    Tile::Tracktile(_) => {}
                    Tile::Trainsource(trainsource) => {
                        canvas.copy(&gs.atlas, gs.source_sink_border, rect)?;

                        let num_cols;
                        if trainsource.trains.len() <= 1 {
                            num_cols = 1;
                        } else if trainsource.trains.len() <= 4 {
                            num_cols = 2;
                        } else if trainsource.trains.len() <= 9 {
                            num_cols = 3;
                        } else {
                            num_cols = 4;
                        }
                        for i in 0..trainsource.trains.len() {
                            if let Some(color) = trainsource.trains[i] {
                                let curr_col = i % num_cols;
                                let curr_row = i / num_cols;
                                let scaled_plus_sign_width = plus_sign_width / num_cols as i32;
                                let scaled_plus_sign_height = plus_sign_height / num_cols as i32;
                                let x_pos = rect.x()
                                    + (block_width - plus_sign_width) / 2
                                    + curr_col as i32 * scaled_plus_sign_width;
                                let y_pos = rect.y()
                                    + (block_height - plus_sign_height) / 2
                                    + curr_row as i32 * scaled_plus_sign_height;
                                gs.set_color(color);
                                canvas.copy(
                                    &gs.atlas_color,
                                    gs.plus_sign,
                                    Rect::new(
                                        x_pos,
                                        y_pos,
                                        scaled_plus_sign_width as u32,
                                        scaled_plus_sign_height as u32,
                                    ),
                                )?;
                            }
                        }
                    }
                    Tile::Trainsink(trainsink) => {
                        if !trainsink.is_satisfied() {
                            canvas.copy(&gs.atlas, gs.source_sink_border, rect)?;

                            let num_cols;
                            if trainsink.desires.len() <= 1 {
                                num_cols = 1;
                            } else if trainsink.desires.len() <= 4 {
                                num_cols = 2;
                            } else if trainsink.desires.len() <= 9 {
                                num_cols = 3;
                            } else {
                                num_cols = 4;
                            }
                            for i in 0..trainsink.desires.len() {
                                if let Some(color) = trainsink.desires[i] {
                                    let curr_col = i % num_cols;
                                    let curr_row = i / num_cols;
                                    let scaled_plus_sign_width = plus_sign_width / num_cols as i32;
                                    let scaled_plus_sign_height =
                                        plus_sign_height / num_cols as i32;
                                    let x_pos = rect.x()
                                        + (block_width - plus_sign_width) / 2
                                        + curr_col as i32 * scaled_plus_sign_width;
                                    let y_pos = rect.y()
                                        + (block_height - plus_sign_height) / 2
                                        + curr_row as i32 * scaled_plus_sign_height;
                                    gs.set_color(color);
                                    canvas.copy(
                                        &gs.atlas_color,
                                        gs.circle,
                                        Rect::new(
                                            x_pos,
                                            y_pos,
                                            scaled_plus_sign_width as u32,
                                            scaled_plus_sign_height as u32,
                                        ),
                                    )?;
                                }
                            }
                        } else {
                            canvas.copy(&gs.atlas, gs.sink_satisfied, rect)?;
                        }
                    }
                    Tile::Rock => {}
                    Tile::Painter(painter) => {
                        canvas.copy(&gs.atlas, gs.painter_bg, rect)?;
                        gs.set_color(painter.color);
                        canvas.copy(&gs.atlas_color, gs.painter_brush, rect)?;
                    }
                    Tile::Splitter(splitter) => {
                        canvas.copy_ex(
                            &gs.atlas,
                            gs.splitter,
                            rect,
                            splitter.incoming_dir as f64 * 90.0,
                            None,
                            false,
                            false,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}
