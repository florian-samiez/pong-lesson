use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

// Dimension of the window
const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
// Movement speed for the paddle and the ball
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;
// Will be used to move the ball on the y axis
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

// Store the information of the current game (Players and ball position)
struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    player1_score: u8,
    player2_score: u8,
}
impl GameState {
    // Let's initialize the players/balls position, texture and speed
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );

        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        Ok(GameState {
            player1: Entity::new("player1".to_string(), player1_texture, player1_position),
            player2: Entity::new("player2".to_string(), player2_texture, player2_position),
            ball: Entity::with_velocity(
                "ball".to_string(),
                ball_texture,
                ball_position,
                ball_velocity,
            ), // Allow to give a direction to the ball
            player1_score: 0,
            player2_score: 0,
        })
    }
}
impl State for GameState {
    // Let's render on the screen the calculated position and texture of all the elements
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);
        Ok(())
    }

    // The logic of the game is here (Collision, movement)
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Those "if" will read if a key is pressed, and will move the paddle
        if input::is_key_down(ctx, Key::Z) {
            self.player1.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }

        // Add the velocity Vector to the Position Vector to move the ball
        self.ball.position += self.ball.velocity;

        // The bounds will help us to detect collision
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        // Logic of the movement of the ball when a collision happen
        if let Some(paddle) = paddle_hit {
            // The ball need to go to the opposite of the paddle with which it has collided, with a acceleration
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

            // Then the ball need to go a bit down or up based of its position in relation to the center of the paddle
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        // If the ball collide with the top or the bottom of the screen, we need it to go to the opposite direction
        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Then we need a winner
        if self.ball.position.x < 0.0 {
            self.player2_score += 1;
            self.ball.center();
            self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
        }
        if self.ball.position.x > WINDOW_WIDTH {
            self.player1_score += 1;
            self.ball.center();
            self.ball.velocity = Vec2::new(BALL_SPEED, 0.0);
        }

        if self.player1_score >= 5 || self.player2_score >= 5 {
            let winner = if self.player1_score > self.player2_score {
                &self.player1.name
            } else {
                &self.player2.name
            };
            println!("{} wins !", winner);
            window::quit(ctx);
        }

        Ok(())
    }
}

// An entity can be a ball or a player. It's defined by its appearance, position and velocity
struct Entity {
    name: String,
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}
impl Entity {
    fn new(name: String, texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(name, texture, position, Vec2::zero())
    }
    // For ball, we need a velocity
    fn with_velocity(
        name: String,
        texture: Texture,
        position: Vec2<f32>,
        velocity: Vec2<f32>,
    ) -> Entity {
        Entity {
            name,
            texture,
            position,
            velocity,
        }
    }
    fn width(&self) -> f32 {
        self.texture.width() as f32
    }
    fn height(&self) -> f32 {
        self.texture.height() as f32
    }
    // Using Rectangle struct will help us to detect collision
    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }
    // The center will help us to calculate the offset on the ball movement
    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
    fn center(&mut self) {
        self.position = Vec2::new(
            WINDOW_WIDTH / 2.0 - self.texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - self.texture.height() as f32 / 2.0,
        );
    }
}
