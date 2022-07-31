use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([50f32, 100f32]);
const OBSTACLE_SIZE: Vec2 = const_vec2!([50f32, 50f32]);
const JUMP_POWER: f32 = 7f32;
const GRAVITY: f32 = 0.25f32;
const OBSTACLE_VELOCITY: f32 = 5f32;

#[macroquad::main("Input_keys")]
async fn main() {

    let mut player = Player::new();
    let mut obstacle = Obstacle::new();

    loop {
        clear_background(BEIGE);

        //Floor
        draw_line(
            0.0,
            screen_height() * 0.75,
            screen_width(),
            screen_height() * 0.75,
            5.0,
            BLACK,
        );

        // Player
        player.draw();
        player.update(get_frame_time());

        // Obstacle
        obstacle.draw();
        obstacle.update(get_frame_time());


        draw_text("by shuwais", 20.0, 20.0, 20.0, RED);
        next_frame().await
    }
}

//Player Struct
struct Player {
    rect: Rect,
    vel: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(100.0, 100.0, PLAYER_SIZE.x, PLAYER_SIZE.y),
            vel: 0f32,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }

    pub fn update(&mut self, dt: f32) {
        // Apply Gravity
        self.vel -= GRAVITY;

        // Check if on floor
        // if on the floor vel = 0
        if self.rect.y + self.rect.h >= screen_height() * 0.75 {
            self.vel = 0f32;
        }

        // Jump only if the player is on the floor
        if is_key_down(KeyCode::Space) && self.rect.y + self.rect.h >= screen_height() * 0.75 {
            self.vel = JUMP_POWER;
        }

        // Apply the calculation
        self.rect.y -= self.vel * dt * 60f32;
    }
}

struct Obstacle {
    rect: Rect,
    vel: f32,
}

impl Obstacle {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width(),
                screen_height() * 0.75f32 - OBSTACLE_SIZE.y,
                OBSTACLE_SIZE.x,
                OBSTACLE_SIZE.y,
            ),
            vel: OBSTACLE_VELOCITY,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x -= self.vel * dt * 60f32;

        if self.rect.x + self.rect.w < 0f32 {
            self.rect.x = screen_width();
        }
    }
}
