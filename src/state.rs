//  state.rs

use crate::assets;
use crate::explosion;
use crate::params;
use crate::sprite;
use crate::telemetry;
use macroquad::prelude as mq;

pub struct Bullet {
    pub xpos: f32,
    pub ypos: f32,
    pub velocity: f32,
    pub fresh: bool, //  used for sound
    pub dead: bool,
}

pub struct Alien {
    pub sprite: &'static sprite::Sprite,
    pub xpos: f32,
    pub ypos: f32,
    pub phase: f32,
    pub points: i32,
    pub dead: bool,
}

#[derive(PartialEq)]
pub enum AlienState {
    Right,
    Left,
    DownToRight,
    DownToLeft,
}

pub struct State {
    pub reset_countdown: f32,
    pub current_level: i32,
    pub paused: bool,

    pub score: i32,
    pub lives: i32,
    pub player_pos_fr: f32,
    pub time_to_fire: f32,
    pub speed_ratio: f32,

    pub alien_state: AlienState,
    pub alien_target_y: f32,

    pub bullets: Vec<Bullet>,
    pub aliens: Vec<Alien>,
    pub explosions: Vec<explosion::Explosion>,

    pub bassline_time: f32,
    pub bassline_speed: f32,

    pub player_speed: f32,
    pub firing_duration: f32,
    pub fire_velocity: f32,
}

pub fn new_game_state() -> State {
    return State {
        reset_countdown: 1.2,
        current_level: 0,
        paused: false,

        score: 0,
        lives: 2,
        player_pos_fr: 0.48,
        time_to_fire: 0.0,
        speed_ratio: 0.1,

        alien_state: AlienState::Right,
        alien_target_y: 0.2,

        bullets: Vec::new(),
        aliens: Vec::new(),
        explosions: Vec::new(),

        bassline_time: 0.0,
        bassline_speed: 1.0,

        player_speed: 0.3,
        firing_duration: 0.8,
        fire_velocity: 1.1,
    };
}

fn push_line(aliens: &mut Vec<Alien>, sprite: &'static sprite::Sprite, ypos: f32, points: i32) {
    let mut ix = 0;
    while ix < 10 {
        aliens.push(Alien {
            sprite: sprite,
            xpos: 0.07 + 0.07 * (ix as f32),
            ypos: ypos,
            phase: 0.0,
            points: points,
            dead: false,
        });
        ix += 1;
    }
}

pub fn reset_level(_level: i32, state: &mut State) {
    state.bullets = Vec::new();
    state.aliens = Vec::new();
    state.player_pos_fr = 0.48;

    state.bassline_time = 0.0;
    state.bassline_speed = 1.0;

    state.alien_state = AlienState::Right;

    //  todo: level progression
    telemetry::tele_new_level(1, state.score);

    let mut ypos = 0.2;

    push_line(&mut state.aliens, &assets::ENEMY3_SPRITE, ypos, 30);
    ypos += 0.1;
    push_line(&mut state.aliens, &assets::ENEMY2_SPRITE, ypos, 20);
    ypos += 0.1;
    push_line(&mut state.aliens, &assets::ENEMY2_SPRITE, ypos, 20);
    ypos += 0.1;
    push_line(&mut state.aliens, &assets::ENEMY1_SPRITE, ypos, 10);
    ypos += 0.1;
    push_line(&mut state.aliens, &assets::ENEMY1_SPRITE, ypos, 10);

    state.alien_target_y = ypos;
}

pub fn update_state(delta_time: f32, state: &mut State, assets: &assets::Assets) {
    if mq::is_key_pressed(mq::KeyCode::Escape) {
        state.paused = !state.paused;
        telemetry::tele_pause(state.paused);
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
                reset_level(state.current_level, state);
            }
        } else {
            //  evolve inputs
            if mq::is_key_down(mq::KeyCode::Right) {
                state.player_pos_fr += delta_time * state.player_speed;
            }
            if mq::is_key_down(mq::KeyCode::Left) {
                state.player_pos_fr -= delta_time * state.player_speed;
            }

            state.player_pos_fr = mq::clamp(
                state.player_pos_fr,
                params::LEFT_MARGIN,
                params::RIGHT_MARGIN,
            );
            if mq::is_key_down(mq::KeyCode::Space) {
                if state.time_to_fire <= 0.0 {
                    //  TODO: original Space Invaders only allowed one bullet alive at once
                    state.time_to_fire = state.firing_duration;
                    state.bullets.push(Bullet {
                        xpos: state.player_pos_fr,
                        ypos: 1.33 * 0.94 - 0.01,
                        velocity: state.fire_velocity,
                        dead: false,
                        fresh: true,
                    });
                    telemetry::tele_shot(state.player_pos_fr, state.score, state.alien_target_y);
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
                telemetry::tele_miss(bullet.xpos, state.score, state.aliens.len());
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
        let level_speed = (state.current_level as f32 + 4.0) / 5.0;
        let h_mul: f32 = params::HORIZ_SPEED / (3.0 + num_aliens) * level_speed;
        let v_mul: f32 = params::VERT_SPEED / (3.0 + num_aliens) * level_speed;

        if state.alien_state == AlienState::Right {
            alien_dx = h_mul * delta_time;
        } else if state.alien_state == AlienState::Left {
            //  Left
            alien_dx = h_mul * delta_time * -1.0;
        } else {
            //  DownToLeft, DownToRight
            alien_dy = v_mul * delta_time;
        }
        let phase_delta = delta_time * params::PHASE_SPEED / (2.0 + num_aliens.sqrt());
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
        if state.alien_state == AlienState::Right && max_x >= params::RIGHT_MARGIN {
            state.alien_state = AlienState::DownToLeft;
            state.alien_target_y = max_y + params::DOWN_DISTANCE;
            adjust_dx = params::RIGHT_MARGIN - max_x; // negative
            adjust_dy = -adjust_dx * params::VERT_SPEED / params::HORIZ_SPEED;
            telemetry::tele_advance(state.alien_target_y, state.aliens.len());
        } else if state.alien_state == AlienState::Left && min_x <= params::LEFT_MARGIN {
            state.alien_state = AlienState::DownToRight;
            state.alien_target_y = max_y + params::DOWN_DISTANCE;
            adjust_dx = params::LEFT_MARGIN - min_x; // positive
            adjust_dy = adjust_dx * params::VERT_SPEED / params::HORIZ_SPEED;
            telemetry::tele_advance(state.alien_target_y, state.aliens.len());
        } else if state.alien_state == AlienState::DownToRight && max_y >= state.alien_target_y {
            state.alien_state = AlienState::Right;
            adjust_dy = state.alien_target_y - max_y; //  negative
            adjust_dx = -adjust_dy * params::HORIZ_SPEED / params::VERT_SPEED;
        } else if state.alien_state == AlienState::DownToLeft && max_y >= state.alien_target_y {
            state.alien_state = AlienState::Left;
            adjust_dy = state.alien_target_y - max_y; //  negative
            adjust_dx = adjust_dy * params::HORIZ_SPEED / params::VERT_SPEED;
        }
        //  adjust for fractional movement
        if adjust_dx != 0.0 || adjust_dy != 0.0 {
            for alien in state.aliens.iter_mut() {
                alien.xpos += adjust_dx;
                alien.ypos += adjust_dy;
            }
        }
        //  detect alien collisions with things
        let bspr: &sprite::LoadedSprite = &assets.sprites[assets::IX_LASER];
        let num_aliens = state.aliens.len();
        for alien in state.aliens.iter_mut() {
            if alien.dead {
                continue;
            }
            let aspr: &sprite::LoadedSprite = &assets.sprites[alien.sprite.index];
            for bullet in state.bullets.iter_mut() {
                if bullet.dead {
                    continue;
                }
                if bullet.xpos + bspr.f_w * 0.5 > alien.xpos - aspr.f_w * 0.5
                    && bullet.xpos - bspr.f_w * 0.5 < alien.xpos + aspr.f_w * 0.5
                    && bullet.ypos > alien.ypos - aspr.f_h * 0.5
                    && bullet.ypos - bspr.f_w < alien.ypos + aspr.f_h * 0.5
                {
                    state.score += alien.points;
                    bullet.dead = true;
                    alien.dead = true;
                    hasdeadbullet = true;
                    hasdeadalien = true;
                    telemetry::tele_hit(
                        bullet.xpos,
                        state.score,
                        alien.sprite.index,
                        alien.points,
                        num_aliens - 1,
                    );
                    state
                        .explosions
                        .push(explosion::new(bullet.xpos, bullet.ypos));
                }
            }
            //  TODO: check barrier collision
            //  TODO: check player collision
        }

        //  evolve explosions
        let mut hasdeadexplosion = false;
        for ex in state.explosions.iter_mut() {
            explosion::evolve(delta_time, ex);
            hasdeadexplosion = hasdeadexplosion || ex.dead;
        }

        if hasdeadbullet {
            state.bullets.retain(|bullet| !bullet.dead);
        }
        if hasdeadalien {
            state.aliens.retain(|alien| !alien.dead);
            if state.aliens.len() == 0 {
                state.reset_countdown = 1.2;
            }
        }
        if hasdeadexplosion {
            state.explosions.retain(|x| !x.dead);
        }

        state.speed_ratio = 4.0 / (3.0 + num_aliens as f32);

        //  TODO: alien bomb dropping
        //  TODO: alien bomb evolution
        //  TODO: barriers
        //  TODO: detect level clear
    }
}
