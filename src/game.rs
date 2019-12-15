use sdl2;
use sdl2::video;
use sdl2::event::Event;
use sdl2::pixels::Color;
pub struct Game {
    pub title: &'static str,
    pub events: sdl2::EventPump,
    pub canvas: sdl2::render::Canvas<video::Window>,
    pub width: u32,
    pub height: u32,
}

impl Game {
    pub fn new(title: &'static str, width: u32, height: u32 ) -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video.window(title, width, height)
            .position_centered()
            .build()
            .unwrap();
        
        let canvas = window.into_canvas()
            .build()
            .unwrap();
        
        Game {
            title: title,
            canvas: canvas,
            events: sdl.event_pump().unwrap(),
            width: width,
            height: height
        }
    }
    pub fn run(&mut self) {
        'running: loop {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit {..} => break 'running,
                    _ => {}
                }
            }

            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color( Color::RGB(0,0,0) );
        self.canvas.clear();
        self.canvas.present(); 
    }

    pub fn draw(&mut self) {

    }
}
