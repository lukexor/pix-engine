use lazy_static::lazy_static;

use pix_engine::prelude::*;

const TITLE: &str = "The Matrix";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// Glyph Constants
const GLYPH_FONT: &str = "Courier New";
const GLYPH_SIZE: u32 = 16;
const GLYPH_COLOR: [u8; 3] = [0, 155, 00];
const GLYPH_COLOR_DARK: [u8; 3] = [0, 155, 00];
const GLYPH_HIGHLIGHT: [u8; 3] = [200, 255, 200];
const GLYPH_HIGHLIGHT_PROBABILITY: usize = 25;
const MORPH_PROBABILITY: usize = 0;
const MORPH_INTERVAL_MIN: usize = 2;
const MORPH_INTERVAL_MAX: usize = 20;

// Stream Constants
const START_Y_MIN: i32 = -2000;
const START_Y_MAX: i32 = -200;
const SPAWN_Y_MIN: i32 = -500;
const SPAWN_Y_MAX: i32 = -100;
const SPEED_MIN: u32 = 1;
const SPEED_MAX: u32 = 2;
const STREAM_EMPTY_PROBABILITY: usize = 0;
const STREAM_MIN: usize = 3;
const STREAM_MAX: usize = 30;

lazy_static! {
    static ref GLYPHS: Vec<char> = {
        let mut glyphs = vec![' ', '$', '@', ':', '-', '+', '*', ';', '.', '<', '>'];
        for i in 0..10 {
            glyphs.push(char::from_digit(i, 10).unwrap());
        }
        // for i in 0..96 {
        //     glyphs.push(char::from_u32(0x30A0 + i).unwrap());
        // }
        glyphs
    };
}

struct Glyph {
    pos: Point,
    value: char,
    size: u32,
    color: Color,
    morph_interval: usize,
}

impl Glyph {
    pub fn new(pos: impl Into<Point>, size: u32, color: Color) -> Self {
        let mut glyph = Self {
            pos: pos.into(),
            value: GLYPHS[0],
            size,
            color,
            morph_interval: random!(MORPH_INTERVAL_MIN, MORPH_INTERVAL_MAX),
        };
        glyph.randomize();
        glyph
    }

    pub fn randomize(&mut self) {
        self.value = GLYPHS[random!(0, GLYPHS.len())];
    }

    pub fn draw(&mut self, s: &mut PixState) -> PixResult<()> {
        let morph_roll = random!(0, 100);
        if s.frame_count() % self.morph_interval == 0 && morph_roll <= MORPH_PROBABILITY {
            self.randomize();
        }
        s.fill(self.color);
        s.font_size(self.size);
        let mut tmp = [0; 2];
        s.text(self.pos, self.value.encode_utf8(&mut tmp))?;
        Ok(())
    }
}

struct Stream {
    pos: Point,
    height: i32,
    highlight: bool,
    glyphs: Vec<Glyph>,
    size: u32,
    color: Color,
    speed: u32,
    spawned: bool,
}

impl Stream {
    pub fn new(pos: impl Into<Point>) -> Self {
        let mut stream = Self {
            pos: pos.into(),
            height: 0,
            highlight: false,
            glyphs: Vec::new(),
            size: GLYPH_SIZE,
            color: GLYPH_COLOR.into(),
            speed: 0,
            spawned: false,
        };
        stream.randomize();
        stream
    }

    pub fn randomize(&mut self) {
        self.glyphs = Vec::new();
        self.speed = random!(SPEED_MIN, SPEED_MAX);
        if self.speed == SPEED_MIN {
            self.color = GLYPH_COLOR_DARK.into();
        }
        if random!(0, 100) <= GLYPH_HIGHLIGHT_PROBABILITY {
            self.highlight = true;
        }
        let empty_prob = random!(0, 100) <= STREAM_EMPTY_PROBABILITY;
        if !empty_prob {
            let count = random!(STREAM_MIN, STREAM_MAX);
            for i in 0..count {
                let y = self.pos.y - i as Scalar * self.size as Scalar;
                let glyph = Glyph::new([self.pos.x, y], self.size, self.color);
                self.glyphs.push(glyph);
            }
            self.height = count as i32 * self.size as i32;
        }
    }

    pub fn x(&self) -> i32 {
        self.pos.x.round() as i32
    }

    pub fn y(&self) -> i32 {
        self.pos.y.round() as i32
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn draw(&mut self, s: &mut PixState) -> PixResult<()> {
        self.pos.y += (self.speed * self.size) as Scalar;
        let len = self.glyphs.len();
        for i in 0..len {
            let idx = (i as Scalar - (self.pos.y / self.size as Scalar).floor()) as usize % len;
            let mut glyph = &mut self.glyphs[idx];
            if i == 0 && self.highlight {
                glyph.color = GLYPH_HIGHLIGHT.into();
            } else {
                glyph.color = self.color;
            }
            glyph.pos.y = self.pos.y - (i as Scalar) * (self.size as Scalar);
            glyph.draw(s)?;
        }
        Ok(())
    }
}

struct Matrix {
    streams: Vec<Stream>,
    new_streams: Vec<Stream>,
}

impl Matrix {
    fn new() -> Self {
        let count = (WIDTH / GLYPH_SIZE) as usize;
        let mut streams = Vec::with_capacity(count);
        for i in 0..2 {
            let x = i as i32 * GLYPH_SIZE as i32;
            let y = random!(START_Y_MIN, START_Y_MAX);
            streams.push(Stream::new([x, y]));
        }
        Self {
            streams,
            new_streams: Vec::with_capacity(count),
        }
    }
}

impl AppState for Matrix {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background([0, 0, 0, 255]);
        s.font_style(FontStyle::BOLD);
        s.font_family(GLYPH_FONT);
        s.set_frame_rate(10);
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear();
        let height = HEIGHT as i32;
        for stream in &mut self.streams {
            stream.draw(s)?;
            if !stream.spawned && stream.y() >= (3 * height / 4) {
                let y = random!(SPAWN_Y_MIN, SPAWN_Y_MAX);
                let new_stream = Stream::new([stream.x(), y]);
                self.new_streams.push(new_stream);
                stream.spawned = true;
            }
        }
        self.streams
            .retain(|stream| stream.y() <= (height + stream.height()));
        self.streams.append(&mut self.new_streams);
        Ok(())
    }

    fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .position_centered()
        .vsync_enabled()
        .build();
    let mut matrix = Matrix::new();
    engine.run(&mut matrix)
}
