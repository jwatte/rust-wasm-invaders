use macroquad::prelude::*;
use std::fmt::format;
use std::borrow::Cow;
use std::borrow::Borrow;
use quad_net::http_request::RequestBuilder;
use quad_net::http_request::Request;

const ASSUMED_SCREEN_WIDTH: i32 = 2400;

#[macroquad::main("Invaders")]
async fn main() {
    make_request();

    let mut state = new_game_state();
    let assets = load_assets().await;

    /* for telemetry, check out:
    https://github.com/not-fl3/quad-net/blob/master/examples/http_request.rs
     */

    loop {
        poll_requests();

        update_state(&mut state, &assets);

        render_scene(&state, &assets);

        next_frame().await;
    }
}

static TELEMETRY_URL : &str = "https://collect.observe-eng.com/v1/http/invaders";
static AUTHORIZATION_HEADER : &str = "Bearer 101 4vVFnBaMXQ9LovF-HxIJGVgxG2V7dmRo";

struct Pending {
    req: Request,
    //  hack time -- I can't get try_recv() to actually return the data,
    //  so just blindly call it good after some amount of polling.
    num: i32,
}

static mut PENDING_REQUESTS : Vec<Pending> = Vec::new();

fn make_request() {
    info!("Request start");
    unsafe {
        PENDING_REQUESTS.push(Pending {
            req: RequestBuilder::new(TELEMETRY_URL)
                    .method(quad_net::http_request::Method::Post)
                    .header("Authorization", AUTHORIZATION_HEADER)
                    .body("{\"type\":\"test\"}")
                    .send(),
            num: 200,
        });
    }
}

fn poll_requests() {
    unsafe {
        for req in PENDING_REQUESTS.iter_mut() {
            if let Some(data) = req.req.try_recv() {
                info!("Request done");
                req.num = 0;
            } else if req.num > 0 {
                req.num -= 1;
            }
        }
        PENDING_REQUESTS.retain(|req| req.num > 0);
    }
}


fn render_scene(state: &State, assets: &Assets) {
    clear_background(Color::new(0.11, 0.11, 0.11, 1.00));

    let sw = screen_width();
    let sh = screen_height();
    let (left, top, width, height) = letterbox(sw, sh);

    {
        //  draw lives
        let pspr = &assets.sprites[0];
        for n in 0..state.lives {
            draw_sprite(left, top, width, pspr, (n as f32 + 0.6) * pspr.f_w * 0.5, 1.333-pspr.f_h*0.6*0.45, 0.0, 0.45);
        }

        //  draw player
        draw_sprite(left, top, width, pspr, state.player_pos_fr, 1.333-0.06, 0.0, 1.0);
    }

    //  draw aliens
    for alien in state.aliens.iter() {
        let asp = &assets.sprites[alien.sprite.index];
        draw_sprite(left, top, width, asp, alien.xpos, alien.ypos, alien.phase.sin()*0.1, 1.0);
    }

    {
        let bspr = &assets.sprites[1];
        //  draw bullets
        for bullet in state.bullets.iter() {
            draw_sprite(left, top, width, bspr, bullet.xpos, bullet.ypos+bspr.f_h*0.5, 0.0, 1.0);
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

    if state.paused {
        let pdisp = format!("Press ESC to Unpause");
        draw_text(&pdisp, left + width * 0.25, top + height * 0.2, height * 0.04, WHITE);
    } else if state.reset_countdown > 0.0 {
        let ctdisp = format!("Countdown: {:.1}", state.reset_countdown);
        draw_text(&ctdisp, left + width * 0.3, top + height * 0.2, height * 0.04, WHITE);
    }
}

fn draw_sprite(left : f32, top : f32, width : f32, spr: &LoadedSprite, x: f32, y: f32, r: f32, s: f32) {
    let dtp = DrawTextureParams {
        dest_size: Some(vec2(
            width * spr.f_w * s,
            width * spr.f_h * s)),
        rotation: r,
        pivot: Some(vec2(left+width*x, top + width*y)),
        ..Default::default()
    };
    draw_texture_ex(
        spr.texture,
        left + width * (x - spr.f_w*0.5*s),
        top + width * (y - spr.f_w*0.5*s),
        LIGHTGRAY,
        dtp,
    );
}

struct Bullet {
    xpos: f32,
    ypos: f32,
    velocity: f32,
    dead: bool,
}

struct Alien {
    sprite: &'static Sprite,
    xpos: f32,
    ypos: f32,
    phase: f32,
    points: i32,
    dead: bool,
}

#[derive(PartialEq)]
enum AlienState {
    Right,
    Left,
    DownToRight,
    DownToLeft,
}

struct State {
    reset_countdown: f32,
    current_level: i32,
    paused: bool,

    score: i32,
    lives: i32,
    player_pos_fr: f32,
    time_to_fire: f32,

    alien_state: AlienState,
    alien_target_y: f32,

    bullets: Vec<Bullet>,
    aliens: Vec<Alien>,

    player_speed: f32,
    firing_duration: f32,
    fire_velocity: f32,
}

fn new_game_state() -> State {
    return State {
        reset_countdown: 1.2,
        current_level: 0,
        paused: false,

        score: 0,
        lives: 2,
        player_pos_fr: 0.48,
        time_to_fire: 0.0,

        alien_state: AlienState::Right,
        alien_target_y: 0.2,

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

    state.alien_state = AlienState::Right;

    let mut ypos = 0.2;

    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.07, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.14, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.21, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.28, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.35, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.42, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.49, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.56, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.63, ypos: ypos, phase: 0.0, points: 30, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY3_SPRITE, xpos: 0.70, ypos: ypos, phase: 0.0, points: 30, dead: false });

    ypos += 0.1;
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.07, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.14, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.21, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.28, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.35, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.42, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.49, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.56, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.63, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.70, ypos: ypos, phase: 0.0, points: 20, dead: false });

    ypos += 0.1;
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.07, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.14, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.21, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.28, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.35, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.42, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.49, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.56, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.63, ypos: ypos, phase: 0.0, points: 20, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY2_SPRITE, xpos: 0.70, ypos: ypos, phase: 0.0, points: 20, dead: false });

    ypos += 0.1;
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.07, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.14, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.21, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.28, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.35, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.42, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.49, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.56, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.63, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.70, ypos: ypos, phase: 0.0, points: 10, dead: false });

    ypos += 0.1;
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.07, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.14, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.21, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.28, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.35, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.42, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.49, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.56, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.63, ypos: ypos, phase: 0.0, points: 10, dead: false });
    state.aliens.push(Alien { sprite:&ENEMY1_SPRITE, xpos: 0.70, ypos: ypos, phase: 0.0, points: 10, dead: false });

    state.alien_target_y = ypos;
}

fn update_state(state: &mut State, assets: &Assets) {
    let delta_time = get_frame_time();

    if is_key_pressed(KeyCode::Escape) {
        state.paused = !state.paused;
    }
    if !state.paused {
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

            state.player_pos_fr = clamp(state.player_pos_fr, LEFT_MARGIN, RIGHT_MARGIN);
            if is_key_down(KeyCode::Space) {
                if state.time_to_fire <= 0.0 {
                    //  TODO: original Space Invaders only allowed one bullet alive at once
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

        //  evolve bullets
        let mut hasdeadbullet = false;
        for bullet in state.bullets.iter_mut() {
            bullet.ypos -= delta_time * bullet.velocity;
            if bullet.ypos < 0.0 {
                bullet.dead = true;
                hasdeadbullet = true;
                //  TODO: original Space Invaders exploded the bullet at the top of the screen
            }
        }

        //  evolve aliens
        let mut hasdeadalien = false;
        let mut alien_dx = 0.0;
        let mut alien_dy = 0.0;
        let mut min_x = 1.0;
        let mut max_x = 0.0;
        let mut max_y = -1.0;
        let num_aliens = state.aliens.len() as f32;
        let h_mul : f32 = HORIZ_SPEED / (3.0 + num_aliens);
        let v_mul : f32 = VERT_SPEED / (3.0 + num_aliens);
        if state.alien_state == AlienState::Right {
            alien_dx = h_mul * delta_time;
        } else if state.alien_state == AlienState::Left {    //  Left
            alien_dx = h_mul * delta_time * -1.0;
        } else {    //  DownToLeft, DownToRight
            alien_dy = v_mul * delta_time;
        }
        let phase_delta = delta_time * PHASE_SPEED / (3.0 + num_aliens.sqrt());
        for alien in state.aliens.iter_mut() {
            alien.phase += phase_delta;
            if alien.phase >= 3.1416 {
                alien.phase -= 6.2832;
            }
            alien.xpos += alien_dx;
            if alien.xpos > max_x {
                max_x = alien.xpos;
            }
            if alien.xpos < min_x {
                min_x = alien.xpos;
            }
            alien.ypos += alien_dy;
            if alien.ypos > max_y {
                max_y = alien.ypos;
            }
            if alien.ypos > 1.333 {
                alien.dead = true;
                hasdeadalien = true;
            }
        }
        let mut adjust_dx = 0.0;
        let mut adjust_dy = 0.0;
        if state.alien_state == AlienState::Right && max_x >= RIGHT_MARGIN {
            state.alien_state = AlienState::DownToLeft;
            state.alien_target_y = max_y + DOWN_DISTANCE;
            adjust_dx = RIGHT_MARGIN - max_x; // negative
            adjust_dy = -adjust_dx * VERT_SPEED / HORIZ_SPEED;
        } else if state.alien_state == AlienState::Left && min_x <= LEFT_MARGIN {
            state.alien_state = AlienState::DownToRight;
            state.alien_target_y = max_y + DOWN_DISTANCE;
            adjust_dx = LEFT_MARGIN - min_x; // positive
            adjust_dy = adjust_dx * VERT_SPEED / HORIZ_SPEED;
        } else if state.alien_state == AlienState::DownToRight && max_y >= state.alien_target_y {
            state.alien_state = AlienState::Right;
            adjust_dy = state.alien_target_y - max_y;   //  negative
            adjust_dx = -adjust_dy * HORIZ_SPEED / VERT_SPEED;
        } else if state.alien_state == AlienState::DownToLeft && max_y >= state.alien_target_y {
            state.alien_state = AlienState::Left;
            adjust_dy = state.alien_target_y - max_y;   //  negative
            adjust_dx = adjust_dy * HORIZ_SPEED / VERT_SPEED;
        }
        //  maybe adjust for fractional movement
        if adjust_dx != 0.0 || adjust_dy != 0.0 {
            for alien in state.aliens.iter_mut() {
                alien.xpos += adjust_dx;
                alien.ypos += adjust_dy;
            }
        }
        //  detect bullet collisions
        let bspr : &LoadedSprite = &assets.sprites[IX_LASER];
        for alien in state.aliens.iter_mut() {
            if alien.dead {
                continue;
            }
            let aspr : &LoadedSprite = &assets.sprites[alien.sprite.index];
            for bullet in state.bullets.iter_mut() {
                if bullet.dead {
                    continue;
                }
                if bullet.xpos + bspr.f_w * 0.5 > alien.xpos - aspr.f_w * 0.5 &&
                    bullet.xpos - bspr.f_w * 0.5 < alien.xpos + aspr.f_w * 0.5 &&
                    bullet.ypos > alien.ypos - aspr.f_h * 0.5 &&
                    bullet.ypos - bspr.f_w < alien.ypos + aspr.f_h * 0.5 {
                        state.score += alien.points;
                        bullet.dead = true;
                        alien.dead = true;
                        hasdeadbullet = true;
                        hasdeadalien = true;
                        //  TODO: spawn explosion
                }
            }
            //  TODO: check player collision
        }
        if hasdeadbullet {
            state.bullets.retain(|bullet| !bullet.dead);
        }
        if hasdeadalien {
            state.aliens.retain(|alien| !alien.dead);
        }

        //  TODO: alien bomb dropping
        //  TODO: alien bomb evolution
        //  TODO: barriers
        //  TODO: detect level clear
    }
}

const HORIZ_SPEED : f32 = 1.0;
const VERT_SPEED : f32 = 4.0;
const RIGHT_MARGIN : f32 = 0.96;
const LEFT_MARGIN : f32 = 0.04;
const DOWN_DISTANCE : f32 = 0.04;
const PHASE_SPEED : f32 = 30.0;

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
const IX_ALIEN_3 : usize = 4;

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
    scale: 1.3,
};

static ENEMY2_SPRITE : Sprite = Sprite {
    index: IX_ALIEN_2,
    filename: b_str("data/Enemies/enemyBlue2.png"),
    width_pix: 93,
    height_pix: 84,
    scale: 1.35,
};

static ENEMY3_SPRITE : Sprite = Sprite {
    index: IX_ALIEN_3,
    filename: b_str("data/Enemies/enemyRed1.png"),
    width_pix: 104,
    height_pix: 84,
    scale: 1.3,
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
    vec.push(load_sprite(&ENEMY3_SPRITE).await);
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
