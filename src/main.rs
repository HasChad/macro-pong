#![windows_subsystem = "windows"]

use macroquad::audio::*;
use macroquad::ui::{hash, root_ui};
use macroquad::{audio::load_sound, prelude::*};

const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 450.;

struct Ball {
    pos: Vec2,
    vel: Vec2,
}

impl Ball {
    fn zeroing(&mut self) {
        self.pos = Vec2 {
            x: SCREEN_WIDTH / 2.,
            y: SCREEN_HEIGHT / 2.,
        };

        self.vel = Vec2 {
            x: -self.vel.x,
            y: 0.,
        };
    }
}

struct Player {
    pos: Vec2,
}

impl Player {
    fn zeroing(&mut self) {
        self.pos.y = SCREEN_HEIGHT / 2.;
    }
}

struct Points {
    left: i32,
    right: i32,
}

impl Points {
    fn zeroing(&mut self) {
        *self = Points { left: 0, right: 0 };
    }
}

#[derive(Debug)]
enum GameState {
    MainMenu,
    Gameplay,
    Quit,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Macro-Pong".into(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let mut game_state = GameState::MainMenu;

    loop {
        match game_state {
            GameState::Gameplay => gameplay(&mut game_state).await,
            GameState::MainMenu => mainmenu(&mut game_state).await,
            GameState::Quit => break,
        }

        //println!("{:?}", game_state);
    }
}

// ! Gameplay
async fn gameplay(game_state: &mut GameState) {
    //texture load
    let left_player = load_texture("sprites/playerLeft.png").await.unwrap();
    let right_player = load_texture("sprites/playerRight.png").await.unwrap();
    let ball = load_texture("sprites/ball.png").await.unwrap();
    let background = load_texture("sprites/background.png").await.unwrap();

    //sound load
    let rightplayer_collision_sound = load_sound("sounds/right_player.wav").await.unwrap();
    let leftplayer_collision_sound = load_sound("sounds/left_player.wav").await.unwrap();
    let border_collision_sound = load_sound("sounds/border.wav").await.unwrap();
    let winning_sound = load_sound("sounds/win.wav").await.unwrap();
    let countend_sound = load_sound("sounds/countend.wav").await.unwrap();

    //position and variable decleration
    let mut ball_promp = Ball {
        pos: Vec2 {
            x: SCREEN_WIDTH / 2.,
            y: SCREEN_HEIGHT / 2.,
        },
        vel: Vec2 { x: 300.0, y: 0.0 },
    };

    let mut left_player_promp = Player {
        pos: Vec2 {
            x: 50.,
            y: SCREEN_HEIGHT / 2.,
        },
    };

    let mut right_player_promp = Player {
        pos: Vec2 {
            x: SCREEN_WIDTH - 50.,
            y: SCREEN_HEIGHT / 2.,
        },
    };
    let mut points = Points { left: 0, right: 0 };
    let mut can_play = false;
    let mut count = 3.0;

    // ! Gameplay Loop
    'gameplay: while !is_key_pressed(KeyCode::Escape) {
        if !can_play {
            count -= get_frame_time();

            if count < 0.0 {
                can_play = true;
                play_sound_once(&countend_sound)
            }
        }
        if can_play {
            //user input
            if is_key_down(KeyCode::Up)
                && right_player_promp.pos.y - right_player.height() / 2. > 0.
            {
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
            ball_promp.pos.x += ball_promp.vel.x * get_frame_time();
            ball_promp.pos.y += ball_promp.vel.y * get_frame_time();

            //border collision
            if ball_promp.pos.y - ball.height() / 2. <= 0.
                || ball_promp.pos.y + ball.height() / 2. >= SCREEN_HEIGHT
            {
                play_sound_once(&border_collision_sound);
                ball_promp.vel.y = -ball_promp.vel.y;
            }

            //right player collision
            if ball_promp.pos.x + ball.width() / 2. > right_player_promp.pos.x //right player mid
                && ball_promp.pos.x + ball.width() / 2. > right_player_promp.pos.x - right_player.width() / 2. //right player line
                && ball_promp.pos.y - ball.height() / 2. < right_player_promp.pos.y + right_player.height() / 2. //lower player
                && ball_promp.pos.y + ball.height() / 2. > right_player_promp.pos.y - right_player.height() / 2. //upper player
                && ball_promp.pos.x < right_player_promp.pos.x
            {
                play_sound_once(&rightplayer_collision_sound);
                ball_promp.pos.x =
                    right_player_promp.pos.x - right_player.width() / 2. - ball.width() / 2.;
                ball_promp.vel.x = -ball_promp.vel.x;
                ball_promp.vel.y = (ball_promp.pos.y - right_player_promp.pos.y) * 5.;

                if ball_promp.vel.x > -600. {
                    ball_promp.vel.x -= 5.;
                }
            }

            //left player collision
            if ball_promp.pos.x - ball.width() / 2. < left_player_promp.pos.x //left player mid
                && ball_promp.pos.x - ball.width() / 2. < left_player_promp.pos.x + left_player.width() / 2. //passing left player line
                && ball_promp.pos.y - ball.height() / 2. < left_player_promp.pos.y + left_player.height() / 2. //lower player
                && ball_promp.pos.y + ball.height() / 2. > left_player_promp.pos.y - left_player.height() / 2. //upper player
                && ball_promp.pos.x > left_player_promp.pos.x
            {
                play_sound_once(&leftplayer_collision_sound);
                ball_promp.pos.x =
                    left_player_promp.pos.x + left_player.width() / 2. + ball.width() / 2.;
                ball_promp.vel.x = -ball_promp.vel.x;
                ball_promp.vel.y = (ball_promp.pos.y - left_player_promp.pos.y) * 5.;

                if ball_promp.vel.x < 600. {
                    ball_promp.vel.x += 5.;
                }
            }
        }

        //point counter
        if ball_promp.pos.x < 0. || ball_promp.pos.x > SCREEN_WIDTH {
            play_sound_once(&winning_sound);

            if ball_promp.pos.x < 0. {
                points.right += 1
            } else {
                points.left += 1
            }

            can_play = false;
            count = 3.0;

            ball_promp.zeroing();
            right_player_promp.zeroing();
            left_player_promp.zeroing();

            // ! EndGame Rendering
            if points.left == 5 || points.right == 5 {
                'endgame: loop {
                    clear_background(BLACK);

                    let mut text = "Right Player Won!";
                    if points.left > points.right {
                        text = "Left Player Won!";
                    }

                    let mut font_size = 30.;
                    let mut text_size = measure_text(text, None, font_size as _, 1.0);

                    draw_text(
                        text,
                        screen_width() / 2. - text_size.width / 2.,
                        screen_height() / 2. - text_size.height / 2.,
                        font_size,
                        WHITE,
                    );

                    text = "Press [ENTER] for rematch, [ESC] for main menu";
                    font_size = 30.;
                    text_size = measure_text(text, None, font_size as _, 1.0);

                    draw_text(
                        text,
                        screen_width() / 2. - text_size.width / 2.,
                        screen_height() / 2. - text_size.height / 2. + font_size,
                        font_size,
                        WHITE,
                    );

                    for key in get_keys_pressed() {
                        match key {
                            KeyCode::Enter => {
                                points.zeroing();
                                break 'endgame;
                            }
                            KeyCode::Escape => {
                                break 'gameplay;
                            }
                            _ => (),
                        }
                    }
                    next_frame().await
                }
            }
        }

        // ! ==> MAIN RENDERING <==

        //background
        let tiling_width = vec![0; ((SCREEN_WIDTH / background.width()) + 1.) as usize];
        let tiling_height = vec![0; ((SCREEN_HEIGHT / background.height()) + 1.) as usize];

        for (x, _) in tiling_width.iter().enumerate() {
            for (y, _) in tiling_height.iter().enumerate() {
                draw_texture(
                    &background,
                    x as f32 * background.width(),
                    y as f32 * background.height(),
                    WHITE,
                );
            }
        }

        //right player point
        draw_text(
            points.right.to_string().as_str(),
            screen_width() / 4. * 3. - 32.,
            45.,
            75.0,
            BLACK,
        );

        //left player point
        draw_text(
            points.left.to_string().as_str(),
            screen_width() / 4.,
            45.,
            75.0,
            BLACK,
        );

        if !can_play {
            let temp = count.ceil().to_string();
            let text = temp.as_str();
            let font_size = 100.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            //count text
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                WHITE,
            );
        }

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

        //render ball
        draw_texture(
            &ball,
            ball_promp.pos.x - ball.width() / 2.,
            ball_promp.pos.y - ball.height() / 2.,
            WHITE,
        );

        next_frame().await
    }

    *game_state = GameState::MainMenu;
}

// ! Main Manu
async fn mainmenu(game_state: &mut GameState) {
    clear_background(WHITE);

    if is_key_pressed(KeyCode::Enter) {
        *game_state = GameState::Gameplay
    }

    let ui_windows_size = Vec2::new(screen_width() - 400.0, screen_height() - 200.0);

    root_ui().window(
        hash!(),
        Vec2::new(200., 100.),
        Vec2::new(ui_windows_size.x, ui_windows_size.y),
        |ui| {
            if ui.button(
                Vec2::new(ui_windows_size.x / 2. - 15., ui_windows_size.y / 2. - 5.),
                "PLAY",
            ) {
                *game_state = GameState::Gameplay;
            }

            if ui.button(
                Vec2::new(ui_windows_size.x / 2. - 15., ui_windows_size.y / 2. + 20.),
                "Quit",
            ) {
                *game_state = GameState::Quit;
            }
        },
    );

    next_frame().await;
}
