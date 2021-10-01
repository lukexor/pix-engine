use lazy_static::lazy_static;
use pix_engine::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BG_COLOR: [u8; 4] = [0, 0, 0, 255];

lazy_static! {
    static ref GLYPHS: Vec<char> = {
        let mut glyphs = vec!['0', '1', '2', '3', '4', '5', '7', '8', '9', 'Z', ' ', ':', '.', '"', '-', '+', '*', ';', '|', '_', '╌', '*',
    '=', 'ç', '<', '>', '¦'];
        for i in 0..96 {
            // SAFETY: We know 0x30A0..0x3100 can be represented as chars
            glyphs.push(char::from_u32(0x30A0 + i).expect("valid unicode"))
        }
        glyphs
    };
}

struct Glyph {
    value: char,
    buf: [u8; 4],
}

impl Glyph {
    const FONT: &'static str = "assets/GN-Koharuiro_Sunray.ttf";
    const SIZE: u32 = 24;
    const HEIGHT: u32 = 15;
    const WIDTH: u32 = 15;
    // const COLOR: [u8; 3] = [122, 235, 133];
    // const COLOR: [u8; 3] = [94, 201, 102];
    const COLOR: [u8; 3] = [60, 255, 70];
    const HIGHLIGHT: [u8; 3] = [190, 255, 200];
    const MORPH_PROB: usize = 1;

    fn new() -> Self {
        Self {
            value: Self::random_glyph(),
            buf: [0; 4],
        }
    }

    fn random_glyph() -> char {
        GLYPHS[random!(0, GLYPHS.len())]
    }

    fn draw(&mut self, s: &mut PixState, x: i32, y: i32) -> PixResult<()> {
        if random!(0, 1000) <= Self::MORPH_PROB {
            self.value = Self::random_glyph();
        }
        let ch = &self.value.encode_utf8(&mut self.buf);
        s.text_transformed([x, y], ch, 0.0, None, Flipped::Horizontal)?;
        Ok(())
    }
}

struct Stream {
    x: i32,
    y: i32,
    height: u32,
    highlight: bool,
    glyphs: Vec<Glyph>,
    size: u32,
    color: Color,
    speed: i32,
    spawned: bool,
}

impl Stream {
    const SPEED_RANGE: (i32, i32) = (2, 6);
    const HEIGHT_RANGE: (usize, usize) = (1, 25);
    const START_RANGE: (i32, i32) = (-500, -50);
    const SPAWN_RANGE: (i32, i32) = (-200, -50);
    const HIGHLIGHT_PROB: usize = 30;

    fn new(x: i32) -> Self {
        let mut stream = Self {
            x,
            y: random!(Self::START_RANGE.0, Self::START_RANGE.1),
            height: 0,
            highlight: false,
            glyphs: vec![],
            size: Glyph::SIZE,
            color: Glyph::COLOR.into(),
            speed: 0,
            spawned: false,
        };
        stream.randomize();
        stream
    }

    fn spawn(&mut self) -> Self {
        self.spawned = true;
        let mut stream = Stream::new(self.x);
        stream.speed = self.speed;
        stream.y = random!(Self::SPAWN_RANGE.0, Self::SPAWN_RANGE.1);
        stream
    }

    fn should_spawn(&self) -> bool {
        let height_threshold = 10 * random!(Self::HEIGHT_RANGE.0, Self::HEIGHT_RANGE.1);
        !self.spawned && (self.y - self.height as i32) > height_threshold as i32
    }

    fn randomize(&mut self) {
        self.speed = random!(Self::SPEED_RANGE.0, Self::SPEED_RANGE.1);

        if random!(0, 100) <= Self::HIGHLIGHT_PROB {
            self.highlight = true;
        }

        let count = random!(Self::HEIGHT_RANGE.0, Self::HEIGHT_RANGE.1);
        self.height = count as u32 * Glyph::HEIGHT;
        self.glyphs = Vec::with_capacity(count);
        for _ in 0..count {
            self.glyphs.push(Glyph::new());
        }
    }

    fn draw(&mut self, s: &mut PixState) -> PixResult<()> {
        self.y += self.speed;
        for (i, glyph) in self.glyphs.iter_mut().enumerate() {
            let y = self.y - (i as i32 * (Glyph::HEIGHT as i32));
            let color = if i == 0 && self.highlight {
                Glyph::HIGHLIGHT.into()
            } else {
                self.color
            };
            s.fill(color);
            s.font_size(self.size)?;
            glyph.draw(s, self.x, y)?;
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
        let count = (WIDTH / Glyph::WIDTH) as usize;
        let mut streams = Vec::with_capacity(count);
        let mut x = 0;
        for _ in 0..count {
            streams.push(Stream::new(x));
            x += Glyph::WIDTH as i32;
        }
        Self {
            streams,
            new_streams: vec![],
        }
    }
}

impl AppState for Matrix {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(BG_COLOR)?;
        s.font_style(FontStyle::BOLD);
        s.font_size(Glyph::SIZE)?;
        s.font_family(Glyph::FONT)?;
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        self.new_streams.clear();
        self.streams
            .retain(|stream| stream.y < (HEIGHT + stream.height) as i32);
        for stream in &mut self.streams {
            stream.draw(s)?;
            if stream.should_spawn() {
                self.new_streams.push(stream.spawn());
            }
        }
        self.streams.append(&mut self.new_streams);
        Ok(())
    }

    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<()> {
        match event.key {
            Key::Escape => {
                if s.running() {
                    s.no_run();
                } else {
                    s.run();
                }
            }
            Key::Return => {
                let fs = !s.fullscreen()?;
                s.set_fullscreen(fs)?;
            }
            _ => (),
        }
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("The Matrix")
        .position_centered()
        .with_frame_rate()
        .vsync_enabled()
        .build();
    let mut app = Matrix::new();
    engine.run(&mut app)
}
