mod game;

const MAX_W: u32 = 128;
const MAX_H: u32 = 64;

fn main() {
    let mut app = game::Game::new("space_shooter", MAX_W, MAX_H);
    app.run();
    /*let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();

    let window = video
        .window("space_shooter", MAX_W, MAX_H)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    let mut events = ctx.event_pump().unwrap();

    let mut pl = player::Player::new();

    // let _palette = Palette::with_colors(&[Color::RGB(0,0,0), Color::RGB(255,255,255)]); unused yet..

    'running: loop {
        
        for event in events.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'running,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), ..} => {
                    if pl.can_move( sdl2::rect::Rect::new(0,0,128,64) )  {
                        pl.move_left();
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                    if pl.can_move( sdl2::rect::Rect::new(0,0,128,64) )  {
                        pl.move_right();
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), ..} => {
                    if pl.can_move( sdl2::rect::Rect::new(0,0,128,64) )  {
                        pl.move_up();
                    }
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), ..} => {
                    if pl.can_move( sdl2::rect::Rect::new(0,0,128,64) )  {
                        pl.move_down();
                    }
                },
                _ => {}
            }
        }

        pl.draw(&canvas); // unused...
        pl.update();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.fill_rect(sdl2::rect::Rect::new( pl.pos_x.into(), pl.pos_y.into(), pl.size_w.into(), pl.size_h.into())).unwrap();
        canvas.present();
        
    }*/
}
