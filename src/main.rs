use macroquad::prelude::*;
use std::fmt::format;
use std::borrow::Cow;
use std::borrow::Borrow;

const ASSUMED_SCREEN_WIDTH: i32 = 2400;

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
            //  draw lives
            let pspr = &assets.sprites[0];
            for n in 0..state.lives {
                let dtp = DrawTextureParams {
                    dest_size: Some(vec2(width * pspr.f_w * 0.45, width * pspr.f_h * 0.45)),
                    ..Default::default()
                };
                draw_texture_ex(
                    pspr.texture,
                    left + width * (n as f32) * pspr.f_w * 0.5,
                    top + height - width * pspr.f_w * 0.5,
                    LIGHTGRAY,
                    dtp,
                );
            }

            //  draw player
            let dtp = DrawTextureParams {
                dest_size: Some(vec2(width * pspr.f_w, width * pspr.f_h)),
                ..Default::default()
            };
            draw_texture_ex(
                pspr.texture,
                left + state.player_pos_fr * width - width * pspr.f_w * 0.5,
                top + height * 0.94 - width * pspr.f_w * 0.5,
                WHITE,
                dtp,
            );
        }

        //  draw aliens
        for alien in state.aliens.iter() {
            let asp = &assets.sprites[alien.index];
            let adtp = DrawTextureParams {
                dest_size: Some(vec2(width * asp.f_w, width * asp.f_h)),
                rotation: alien.phase.sin()*0.1,
                pivot: Some(vec2(left+width*alien.xpos, top + width*alien.ypos)),
                ..Default::default()
            };
            draw_texture_ex(
                asp.texture,
                left + width * alien.xpos - width * asp.f_w * 0.5,
                top + width * alien.ypos - width * asp.f_h * 0.5,
                WHITE,
                adtp
            )
        }

        {
            let bspr = &assets.sprites[1];
            //  draw bullets
            for bullet in state.bullets.iter() {
                draw_texture_ex(
                    bspr.texture,
                    left + width * (bullet.xpos - bspr.f_w * 0.5),
                    top + width * bullet.ypos,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(width * bspr.f_w, width * bspr.f_h)),
                        ..Default::default()
                    },
                );
            }
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

        if state.reset_countdown > 0.0 {
            let ctdisp = format!("Countdown: {:.1}", state.reset_countdown);
            draw_text(&ctdisp, left + width * 0.3, top + height * 0.2, height * 0.04, WHITE);
        }

        next_frame().await;

        update_state(&mut state, &assets);
    }
}

struct Bullet {
    xpos: f32,
    ypos: f32,
    velocity: f32,
    dead: bool,
}

struct Alien {
    index: usize,
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

fn reset_level(_level: i32, state: &mut State, assets: &Assets) {
    state.bullets = Vec::new();
    state.aliens = Vec::new();
    state.player_pos_fr = 0.48;

    state.aliens.push(Alien { index: IX_ALIEN_1, xpos: 0.1, ypos: 0.1, phase: 0.0, dead: false, dead_timer: 0.0 });
    state.aliens.push(Alien { index: IX_ALIEN_2, xpos: 0.15, ypos: 0.1, phase: 0.0, dead: false, dead_timer: 0.0 });
    state.aliens.push(Alien { index: IX_ALIEN_1, xpos: 0.2, ypos: 0.1, phase: 0.0, dead: false, dead_timer: 0.0 });
    state.aliens.push(Alien { index: IX_ALIEN_2, xpos: 0.25, ypos: 0.1, phase: 0.0, dead: false, dead_timer: 0.0 });
}

fn update_state(state: &mut State, assets: &Assets) {
    let delta_time = get_frame_time();

    //  evolve timers
    if state.time_to_fire > 0.0 {
        state.time_to_fire -= delta_time;
    }

    if state.reset_countdown > 0.0 {
        state.reset_countdown -= delta_time;
        if state.reset_countdown <= 0.0 {
            state.current_level += 1;
            reset_level(state.current_level, state, assets);
        }
    } else {
        //  evolve inputs
        if is_key_down(KeyCode::Right) {
            state.player_pos_fr += delta_time * state.player_speed;
        }
        if is_key_down(KeyCode::Left) {
            state.player_pos_fr -= delta_time * state.player_speed;
        }

        state.player_pos_fr = clamp(state.player_pos_fr, 0.02, 0.98);
        if is_key_down(KeyCode::Space) {
            if state.time_to_fire <= 0.0 {
                state.time_to_fire = state.firing_duration;
                state.bullets.push(Bullet {
                    xpos: state.player_pos_fr,
                    ypos: 1.33 * 0.94 - 0.01,
                    velocity: state.fire_velocity,
                    dead: false,
                })
            }
        }
    }

    //  evolve aliens
    for alien in state.aliens.iter_mut() {
        alien.phase += delta_time * 2.4;
        if alien.phase >= 3.1416 {
            alien.phase -= 6.2832;
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



struct Sprite {
    index: usize,
    filename: Cow<'static, str>,
    width_pix: i32,
    height_pix: i32,
    scale: f32,
}

struct LoadedSprite {
    sprite: &'static Sprite,
    texture: Texture2D,
    f_w: f32,
    f_h: f32,
}

struct Assets {
    sprites: Vec<LoadedSprite>,
}

const fn b_str(s: &'static str) -> Cow<'static, str> {
    return Cow::Borrowed(s);
}

const IX_PLAYER : usize = 0;
const IX_LASER : usize = 1;
const IX_ALIEN_1 : usize = 2;
const IX_ALIEN_2 : usize = 3;

static PLAYER_SPRITE : Sprite = Sprite {
    index: IX_PLAYER,
    filename: b_str("data/playerShip3_blue.png"),
    width_pix: 95,
    height_pix: 75,
    scale: 1.0,
};

static LASER_SPRITE : Sprite = Sprite {
    index: IX_LASER,
    filename: b_str("data/Lasers/laserBlue03.png"),
    width_pix: 9,
    height_pix: 37,
    scale: 1.0,
};

static ENEMY1_SPRITE : Sprite = Sprite {
    index: IX_ALIEN_1,
    filename: b_str("data/Enemies/enemyGreen3.png"),
    width_pix: 103,
    height_pix: 84,
    scale: 1.0,
};

static ENEMY2_SPRITE : Sprite = Sprite {
    index: IX_ALIEN_2,
    filename: b_str("data/Enemies/enemyRed1.png"),
    width_pix: 93,
    height_pix: 84,
    scale: 1.0,
};

async fn load_sprite(spr : &'static Sprite) -> LoadedSprite {
    let texture = load_texture(spr.filename.borrow()).await.unwrap();
    return LoadedSprite{
        sprite: spr,
        texture: texture,
        f_w: (spr.width_pix as f32) / (ASSUMED_SCREEN_WIDTH as f32) * spr.scale,
        f_h: (spr.height_pix as f32) / (ASSUMED_SCREEN_WIDTH as f32) * spr.scale,
    };
}

async fn load_assets() -> Assets {
    let mut vec: Vec<LoadedSprite> = Vec::new();
    vec.push(load_sprite(&PLAYER_SPRITE).await);
    vec.push(load_sprite(&LASER_SPRITE).await);
    vec.push(load_sprite(&ENEMY1_SPRITE).await);
    vec.push(load_sprite(&ENEMY2_SPRITE).await);
    return Assets{
        sprites: vec,
    }
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
