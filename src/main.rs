use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([50f32, 100f32]);
const OBSTACLE_SIZE: Vec2 = const_vec2!([50f32, 50f32]);
const JUMP_POWER: f32 = 7f32;
const GRAVITY: f32 = 0.25f32;
const OBSTACLE_VELOCITY: f32 = 5f32;

pub enum GameState {
    Startscreen,
    Play,
    Gameover,
}

#[macroquad::main("Input_keys")]
async fn main() {
    let mut player = Player::new();
    let mut obstacle = Obstacle::new();
    let mut score: u32 = 0;

    let mut game_state = GameState::Startscreen;
    loop {
        match game_state {
            GameState::Startscreen => {
                screen_printer("Start", "Press Space to start");
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Play;
                }
            }
            GameState::Play => {
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
                obstacle.update(get_frame_time(), &mut score);

                //Collision Detector
                if collision_detector(player.rect, obstacle.rect) {
                    game_state = GameState::Gameover;
                }

                draw_text(&score.to_string(), 20.0, 20.0, 20.0, RED);
            }

            GameState::Gameover => {
                screen_printer("Game Over", "Press Space to restart");
                if is_key_down(KeyCode::Space) {
                    // reset Game
                    player = Player::new();
                    obstacle = Obstacle::new();
                    score = 0;
                    game_state = GameState::Play;
                }
            }
        }
        next_frame().await;
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

    pub fn update(&mut self, dt: f32, score: &mut u32) {
        self.rect.x -= self.vel * dt * 60f32;

        if self.rect.x + self.rect.w < 0f32 {
            self.rect.x = screen_width();
            *score += 1;
        }
    }
}

// Collision Detector
fn collision_detector(p: Rect, o: Rect) -> bool {
    o.x <= p.x + p.w && o.x >= p.x && o.y <= p.y + p.h
}

// For Displaying Startscreen and Gameover screen
fn screen_printer(heading: &str, sub: &str) {
    draw_text(
        heading,
        screen_width() / 2f32,
        screen_height() / 2f32,
        50f32,
        RED,
    );
    draw_text(
        sub,
        screen_width() / 2f32 + 30f32,
        screen_height() / 2f32 + 30f32,
        10f32,
        RED,
    );
}

