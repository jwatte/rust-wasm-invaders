//  state.rs

use crate::sprite::*;
use macroquad::prelude::*;
use crate::params::*;
use crate::telemetry::*;
use crate::assets::*;


pub struct Bullet {
    pub xpos: f32,
    pub ypos: f32,
    pub velocity: f32,
    pub dead: bool,
}

pub struct Alien {
    pub sprite: &'static Sprite,
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

    pub alien_state: AlienState,
    pub alien_target_y: f32,

    pub bullets: Vec<Bullet>,
    pub aliens: Vec<Alien>,

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

        alien_state: AlienState::Right,
        alien_target_y: 0.2,

        bullets: Vec::new(),
        aliens: Vec::new(),

        player_speed: 1.0,
        firing_duration: 0.7,
        fire_velocity: 1.2,
    };
}

pub fn reset_level(_level: i32, state: &mut State) {
    state.bullets = Vec::new();
    state.aliens = Vec::new();
    state.player_pos_fr = 0.48;

    state.alien_state = AlienState::Right;

    //  todo: level progression
    tele_new_level(1, state.score);

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

pub fn update_state(state: &mut State, assets: &Assets) {
    let delta_time = get_frame_time();

    if is_key_pressed(KeyCode::Escape) {
        state.paused = !state.paused;
        tele_pause(state.paused);
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
                    });
                    tele_shot(state.player_pos_fr, state.score, state.alien_target_y);
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
                tele_miss(bullet.xpos, state.score, state.aliens.len());
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
            tele_advance(state.alien_target_y, state.aliens.len());
        } else if state.alien_state == AlienState::Left && min_x <= LEFT_MARGIN {
            state.alien_state = AlienState::DownToRight;
            state.alien_target_y = max_y + DOWN_DISTANCE;
            adjust_dx = LEFT_MARGIN - min_x; // positive
            adjust_dy = adjust_dx * VERT_SPEED / HORIZ_SPEED;
            tele_advance(state.alien_target_y, state.aliens.len());
        } else if state.alien_state == AlienState::DownToRight && max_y >= state.alien_target_y {
            state.alien_state = AlienState::Right;
            adjust_dy = state.alien_target_y - max_y;   //  negative
            adjust_dx = -adjust_dy * HORIZ_SPEED / VERT_SPEED;
        } else if state.alien_state == AlienState::DownToLeft && max_y >= state.alien_target_y {
            state.alien_state = AlienState::Left;
            adjust_dy = state.alien_target_y - max_y;   //  negative
            adjust_dx = adjust_dy * HORIZ_SPEED / VERT_SPEED;
        }
        //  adjust for fractional movement
        if adjust_dx != 0.0 || adjust_dy != 0.0 {
            for alien in state.aliens.iter_mut() {
                alien.xpos += adjust_dx;
                alien.ypos += adjust_dy;
            }
        }
        //  detect alien collisions with things
        let bspr : &LoadedSprite = &assets.sprites[IX_LASER];
        let num_aliens = state.aliens.len();
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
                        tele_hit(bullet.xpos, state.score, alien.sprite.index, alien.points, num_aliens-1);
                        //  TODO: spawn explosion
                }
            }
            //  TODO: check barrier collision
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

