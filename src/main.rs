use macroquad::prelude::*;
use std::fmt::format;

#[macroquad::main("Invaders")]
async fn main() {
    let mut state = new_game_state();
    let assets = load_assets().await;

    loop {
        clear_background(DARKGRAY);

        let sw = screen_width();
        let sh = screen_height();
        let (left, top, width, height) = letterbox(sw, sh);

        {
            //  draw player
            let dtp = DrawTextureParams {
                dest_size: Some(vec2(width * 0.04, width * 0.04)),
                ..Default::default()
            };
            draw_texture_ex(
                assets.player_cannon,
                left + state.player_pos_fr * width - width * 0.02,
                top + height * 0.94 - width * 0.02,
                WHITE,
                dtp,
            );
        }

        //  draw lives
        for n in 0..state.lives {
            let dtp = DrawTextureParams {
                dest_size: Some(vec2(width * 0.018, width * 0.018)),
                ..Default::default()
            };
            draw_texture_ex(
                assets.player_cannon,
                left + width * (n as f32) * 0.02,
                top + height - width * 0.02,
                LIGHTGRAY,
                dtp,
            );
        }

        //  draw bullets
        for bullet in state.bullets.iter() {
            draw_texture_ex(
                assets.player_laser_bolt,
                left + width * (bullet.xpos - 0.004),
                top + width * bullet.ypos,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(width * 0.008, width * 0.024)),
                    ..Default::default()
                },
            );
            //let bdisp = format!("{},{}", bullet.xpos, bullet.ypos);
            //draw_text(&bdisp, left, top + height * 0.1, height * 0.02, RED);
        }

        //  draw masking bars
        draw_rectangle(0.0, 0.0, left, sh, BLACK);
        draw_rectangle(left, 0.0, width, top, BLACK);
        draw_rectangle(left + width, 0.0, sw - (left + width), sh, BLACK);
        draw_rectangle(left, top + height, width, sh - (top + height), BLACK);

        //  draw score
        let scoredisp = format!("Score: {}", state.score);
        draw_text(
            &scoredisp,
            left + width * 0.01,
            top + height * 0.03,
            height * 0.04,
            WHITE,
        );

        next_frame().await;

        update_state(&mut state);
    }
}

struct Bullet {
    xpos: f32,
    ypos: f32,
    velocity: f32,
    dead: bool,
}

struct Alien {
    sprite: Texture2D,
    xpos: f32,
    ypos: f32,
    phase: f32,
    dead: bool,
    dead_timer: f32,
}

struct State {
    reset_countdown: f32,
    current_level: i32,

    score: i32,
    lives: i32,
    player_pos_fr: f32,
    time_to_fire: f32,

    bullets: Vec<Bullet>,
    aliens: Vec<Alien>,

    player_speed: f32,
    firing_duration: f32,
    fire_velocity: f32,
}

fn new_game_state() -> State {
    return State {
        reset_countdown: 2.0,
        current_level: 0,

        score: 0,
        lives: 2,
        player_pos_fr: 0.48,
        time_to_fire: 0.0,

        bullets: Vec::new(),
        aliens: Vec::new(),

        player_speed: 1.0,
        firing_duration: 0.7,
        fire_velocity: 1.2,
    };
}

fn reset_level(_level: i32, state: &mut State) {
    state.bullets = Vec::new();
    state.aliens = Vec::new();
    state.player_pos_fr = 0.48;
}

fn update_state(state: &mut State) {
    let delta_time = get_frame_time();

    //  evolve timers
    if state.time_to_fire > 0.0 {
        state.time_to_fire -= delta_time;
    }

    //  evolve inputs
    if is_key_down(KeyCode::Right) {
        state.player_pos_fr += delta_time * state.player_speed;
    }
    if is_key_down(KeyCode::Left) {
        state.player_pos_fr -= delta_time * state.player_speed;
    }

    if state.reset_countdown > 0.0 {
        state.reset_countdown -= delta_time;
        if state.reset_countdown <= 0.0 {
            state.current_level += 1;
            reset_level(state.current_level, state);
        }
    } else {
        state.player_pos_fr = clamp(state.player_pos_fr, 0.02, 0.98);
        if is_key_down(KeyCode::Space) {
            if state.time_to_fire <= 0.0 {
                state.time_to_fire = state.firing_duration;
                state.bullets.push(Bullet {
                    xpos: state.player_pos_fr,
                    ypos: 1.33 * 0.94 - 0.03,
                    velocity: state.fire_velocity,
                    dead: false,
                })
            }
        }
    }

    //  evolve bullets
    let mut hasdead = false;
    for bullet in state.bullets.iter_mut() {
        bullet.ypos -= delta_time * bullet.velocity;
        if bullet.ypos < 0.0 {
            bullet.dead = true;
            hasdead = true;
        } else {
            //  check collision with barriers
            //  check collisions with aliens
            //  check collisions with ufos
        }
    }
    if hasdead {
        state.bullets.retain(|bullet| !bullet.dead);
    }
}

struct Assets {
    player_cannon: Texture2D,
    player_laser_bolt: Texture2D,
    alien_1: Texture2D,
    alien_2: Texture2D,
}

async fn load_assets() -> Assets {
    let player_cannon = load_texture("data/playerShip3_blue.png").await.unwrap();
    let player_laser_bolt = load_texture("data/Lasers/laserBlue03.png").await.unwrap();
    let alien_1 = load_texture("data/Enemies/enemyGreen3.png").await.unwrap();
    let alien_2 = load_texture("data/Enemies/enemyRed1.png").await.unwrap();
    return Assets {
        player_cannon: player_cannon,
        player_laser_bolt: player_laser_bolt,
        alien_1: alien_1,
        alien_2: alien_2,
    };
}

//  Given width/height, which part of the screen do we draw to?
fn letterbox(sw: f32, sh: f32) -> (f32, f32, f32, f32) {
    let (ww, wh) = if sh * 0.75 > sw {
        (sw, sw / 0.75)
    } else {
        (sh * 0.75, sh)
    };
    return (((sw - ww) / 2.0).floor(), ((sh - wh) / 2.0).floor(), ww, wh);
}
