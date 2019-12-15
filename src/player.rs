pub struct Player {
    pub size_w: u32,
    pub size_h: u32,
    pub pos_x: u32,
    pub pos_y: u32,
    pub sprite: [u32; 8],
    pub score: u32
}

impl Player {
    pub fn new() -> Self {
        Player {
            size_w: 8,
            size_h: 8,
            pos_x: 5,
            pos_y: 5,
            sprite: [0,0,0,0,0,0,0,0]
        }
    }

    pub fn update(&mut self) {
        
    }

    pub fn move_up(&mut self) {
        self.pos_y -= 1;
    }

    pub fn move_down(&mut self)  {
        self.pos_y += 1;
    }

    pub fn move_left(&mut self) {
        self.pos_x -= 1;
    }

    pub fn move_right(&mut self) {
        self.pos_x += 1;
    }

    pub fn can_move(&mut self, bounds: sdl2::rect::Rect) -> bool {
        print!("x: {}\ty: {}\r", self.pos_x, self.pos_y );
       (
           ( ( self.pos_x )  > bounds.x 
           && ( self.pos_x + 8 ) < bounds.w )
           || ( ( self.pos_y  ) > bounds.y
           && ( self.pos_y + 8 ) < bounds.h )
       )
    }

    pub fn draw<T>(&self, _canvas: T) {
        //canvas.fill_rect(Rect::new(5,5,8,8));
    }
}
