use macroquad::{audio::load_sound, input, prelude::*};

const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 450.;

struct Ball {
    pos: Vec2,
    vel: Vec2,
}

struct Player {
    pos: Vec2,
}

struct Points {
    left: i32,
    right: i32,
}

#[macroquad::main("Macro-Pong")]
async fn main() {
    //texture load
    let left_player = load_texture("assets/sprites/playerLeft.png").await.unwrap();
    let right_player = load_texture("assets/sprites/playerRight.png")
        .await
        .unwrap();
    let ball = load_texture("assets/sprites/ball.png").await.unwrap();
    let border = load_texture("assets/sprites/border.png").await.unwrap();
    let background = load_texture("assets/sprites/background.png").await.unwrap();

    //sound load
    let player_collision_sound = load_sound("assets/sounds/player.wav").await.unwrap();
    let border_collision_sound = load_sound("assets/sounds/border.wav").await.unwrap();
    let winning_sound = load_sound("assets/sounds/win.wav").await.unwrap();
    let count_sound = load_sound("assets/sounds/count.wav").await.unwrap();
    let countend_sound = load_sound("assets/sounds/countend.wav").await.unwrap();
    let game_music = load_sound("assets/sounds/game-music.mp3").await.unwrap();

    //position and variable decleration
    let mut ball_promp = Ball {
        pos: Vec2 {
            x: SCREEN_WIDTH / 2.,
            y: SCREEN_HEIGHT / 2.,
        },
        vel: Vec2 { x: 2.0, y: 0.0 },
    };

    let mut left_player_promp = Player {
        pos: Vec2 {
            x: 50.0,
            y: SCREEN_HEIGHT / 2.,
        },
    };

    let mut right_player_promp = Player {
        pos: Vec2 {
            x: 750.0,
            y: SCREEN_HEIGHT / 2.,
        },
    };
    let points = Points { left: 0, right: 0 };

    loop {
        clear_background(LIGHTGRAY);
        request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

        //user input
        if is_key_down(KeyCode::Up) && right_player_promp.pos.y - right_player.height() / 2. > 0. {
            right_player_promp.pos.y -= 500. * get_frame_time();
        }

        if is_key_down(KeyCode::Down)
            && right_player_promp.pos.y + right_player.height() / 2. < SCREEN_HEIGHT
        {
            right_player_promp.pos.y += 500. * get_frame_time();
        }

        if is_key_down(KeyCode::W) && left_player_promp.pos.y - left_player.height() / 2. > 0. {
            left_player_promp.pos.y -= 500. * get_frame_time();
        }

        if is_key_down(KeyCode::S)
            && left_player_promp.pos.y + left_player.height() / 2. < SCREEN_HEIGHT
        {
            left_player_promp.pos.y += 500. * get_frame_time();
        }

        //ball movement
        ball_promp.pos.x += ball_promp.vel.x;
        ball_promp.pos.y += ball_promp.vel.y;

        //border collision
        if ball.height() / 2. <= 0. || ball_promp.pos.y + ball.height() / 2. >= SCREEN_HEIGHT {
            ball_promp.vel.y = -ball_promp.vel.y;
        }

        if (ball_promp.pos.x + ball.width() / 2.
            > right_player_promp.pos.x - right_player.width() / 2.)
            || (ball_promp.pos.x - ball.width() / 2.
                < left_player_promp.pos.x + left_player.width() / 2.)
        {
            ball_promp.vel.x = -ball_promp.vel.x;
        }

        //render ball
        draw_texture(
            &ball,
            ball_promp.pos.x - ball.width() / 2.,
            ball_promp.pos.y - ball.height() / 2.,
            WHITE,
        );

        //left player
        draw_texture(
            &left_player,
            left_player_promp.pos.x - left_player.width() / 2.,
            left_player_promp.pos.y - right_player.height() / 2.,
            WHITE,
        );

        //right player
        draw_texture(
            &right_player,
            right_player_promp.pos.x - right_player.width() / 2.,
            right_player_promp.pos.y - right_player.height() / 2.,
            WHITE,
        );
        draw_text(
            get_fps().to_string().as_str(),
            screen_width() / 2.,
            30.,
            50.0,
            BLACK,
        );

        next_frame().await
    }
}
