use pyxel::{Pyxel, PyxelCallback};

pub struct App {
    x: i32,
    y: i32,
}

impl App {
    fn new() -> App {
        App { x: 0, y: 0 }
    }
}

impl PyxelCallback for App {
    fn update(&mut self, pyxel: &mut Pyxel) {
        if pyxel.frame_count() < 60 {
            self.x += (pyxel.frame_count() % 2) as i32;
            self.y -= 1;
        }
    }

    fn draw(&mut self, pyxel: &mut Pyxel) {
        pyxel.cls(3);
        pyxel.pset(self.x, 20, 7);
        pyxel.rect(self.x + 10, 25, 15, 10, 8);
        pyxel.rectb(self.x + 15, 45, 15, 10, pyxel::COLOR_WHITE);
    }
}

pub fn main() {
    let mut pyxel = Pyxel::new(200, 150, Some("Hello, Pyxel in Rust!"), None, None, None);
    let mut app = App::new();

    pyxel.run(&mut app);
}
