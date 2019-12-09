use sdl2;
use sdl2::pixels::Color;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();

    let window = video
        .window("My first game", 128, 64)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    let mut events = ctx.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'running,
                _ => {}
            }
        }
    }
}
