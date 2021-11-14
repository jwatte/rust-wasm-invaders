//  state.rs

use crate::assets;
use crate::explosion;
use crate::highscore;
use crate::params;
use crate::sprite;
use crate::telemetry;
use macroquad::prelude as mq;
use macroquad::rand;

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

pub struct Bomb {
    pub sprite: &'static sprite::Sprite,
    pub xpos: f32,
    pub ypos: f32,
    pub phase: f32,
    pub dead: bool,
}

#[derive(PartialEq)]
pub enum PlayerState {
    Playing,
    HitExploding,
    HitRespawning,
    GameOver,
}

const BOMB_MIN_TIME: f32 = 0.7;
const BOMB_EXTRA_TIME: f32 = 2.5;
pub const HIT_RESPAWN_TIME: f32 = 0.75;
const HIT_EXPLODE_TIME: f32 = 0.75;
const GAME_OVER_TIMEOUT: f32 = 2.0;

pub struct State {
    pub reset_countdown: f32,
    pub current_level: i32,
    pub paused: bool,

    pub score: i32,
    pub lives: i32,
    pub player_pos_fr: f32,
    pub time_to_fire: f32,
    pub speed_ratio: f32,
    pub player_state: PlayerState,
    pub player_hit_timer: f32,

    pub alien_state: AlienState,
    pub alien_target_y: f32,
    pub time_to_bomb: f32,

    pub bullets: Vec<Bullet>,
    pub aliens: Vec<Alien>,
    pub explosions: Vec<explosion::Explosion>,
    pub bombs: Vec<Bomb>,

    pub bassline_time: f32,
    pub bassline_speed: f32,

    pub player_speed: f32,
    pub firing_duration: f32,
    pub fire_velocity: f32,
    pub bomb_speed: f32,
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
        player_state: PlayerState::Playing,
        player_hit_timer: 0.0,

        alien_state: AlienState::Right,
        alien_target_y: 0.2,
        time_to_bomb: 1.0,

        bullets: Vec::new(),
        aliens: Vec::new(),
        explosions: Vec::new(),
        bombs: Vec::new(),

        bassline_time: 0.1,
        bassline_speed: 1.0,

        player_speed: 0.3,
        firing_duration: 0.8,
        fire_velocity: 1.1,
        bomb_speed: 0.6,
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

pub fn reset_level(state: &mut State) {
    telemetry::tele_new_level(state.current_level, state.score);

    state.bullets = Vec::new();
    state.aliens = Vec::new();
    state.explosions = Vec::new();
    state.bombs = Vec::new();
    state.player_pos_fr = 0.48;

    state.bassline_time = 0.1;
    state.bassline_speed = 1.0;

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

    state.alien_state = AlienState::Right;
    state.alien_target_y = ypos;
    state.time_to_bomb = 1.0;
}

pub fn update_state(delta_time: f32, state: &mut State, assets: &assets::Assets) {
    if mq::is_key_pressed(mq::KeyCode::Escape) {
        state.paused = !state.paused;
        telemetry::tele_pause(state.paused);
    }

    let num_aliens_i = state.aliens.len();
    let num_aliens = num_aliens_i as f32;

    if !state.paused {
        //  evolve timers
        if state.time_to_fire > 0.0 {
            state.time_to_fire -= delta_time;
        }

        let mut evolving = false;
        let mut hasdeadbullet = false;
        let mut hasdeadalien = false;
        let mut hasdeadbomb = false;

        if state.reset_countdown > 0.0 {
            state.reset_countdown -= delta_time;
            if state.reset_countdown <= 0.0 {
                state.current_level += 1;
                reset_level(state);
            }
        } else {
            evolving = true;

            if state.player_state == PlayerState::Playing {
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
                        //  Note: original Space Invaders only allowed one bullet alive at once
                        state.time_to_fire = state.firing_duration;
                        state.bullets.push(Bullet {
                            xpos: state.player_pos_fr,
                            ypos: 1.33 * 0.94 - 0.01,
                            velocity: state.fire_velocity,
                            dead: false,
                            fresh: true,
                        });
                        telemetry::tele_shot(
                            state.player_pos_fr,
                            state.score,
                            state.alien_target_y,
                        );
                    }
                }
            } else if state.player_state == PlayerState::HitExploding {
                state.player_hit_timer -= delta_time;
                if state.player_hit_timer <= 0.0 {
                    if state.lives > 0 {
                        state.lives -= 1;
                        state.player_state = PlayerState::HitRespawning;
                        state.player_pos_fr = 0.48;
                        state.player_hit_timer += HIT_RESPAWN_TIME;
                    } else {
                        state.player_state = PlayerState::GameOver;
                        evolving = false;
                        state.player_hit_timer = GAME_OVER_TIMEOUT;
                        telemetry::tele_game_over(
                            state.player_pos_fr,
                            state.score,
                            num_aliens_i,
                            state.current_level,
                        );
                        highscore::register(state.score);
                    }
                }
            } else if state.player_state == PlayerState::HitRespawning {
                state.player_hit_timer -= delta_time;
                if state.player_hit_timer <= 0.0 {
                    state.player_hit_timer = 0.0;
                    state.player_state = PlayerState::Playing;
                }
            } else if state.player_state == PlayerState::GameOver {
                state.player_hit_timer -= delta_time;
                evolving = false;
            }

            //  evolve bullets
            for bullet in state.bullets.iter_mut() {
                bullet.ypos -= delta_time * bullet.velocity;
                if bullet.ypos < 0.0 {
                    bullet.dead = true;
                    hasdeadbullet = true;
                    telemetry::tele_miss(bullet.xpos, state.score, num_aliens_i);
                    //  Note: original Space Invaders exploded the bullet at the top of the screen
                }
            }

            if evolving {
                //  evolve aliens
                let mut alien_dx = 0.0;
                let mut alien_dy = 0.0;
                let mut min_x = 1.0;
                let mut max_x = 0.0;
                let mut max_y = -1.0;
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
                    telemetry::tele_advance(state.alien_target_y, num_aliens_i);
                } else if state.alien_state == AlienState::Left && min_x <= params::LEFT_MARGIN {
                    state.alien_state = AlienState::DownToRight;
                    state.alien_target_y = max_y + params::DOWN_DISTANCE;
                    adjust_dx = params::LEFT_MARGIN - min_x; // positive
                    adjust_dy = adjust_dx * params::VERT_SPEED / params::HORIZ_SPEED;
                    telemetry::tele_advance(state.alien_target_y, num_aliens_i);
                } else if state.alien_state == AlienState::DownToRight
                    && max_y >= state.alien_target_y
                {
                    state.alien_state = AlienState::Right;
                    adjust_dy = state.alien_target_y - max_y; //  negative
                    adjust_dx = -adjust_dy * params::HORIZ_SPEED / params::VERT_SPEED;
                } else if state.alien_state == AlienState::DownToLeft
                    && max_y >= state.alien_target_y
                {
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

                state.time_to_bomb -= delta_time;

                //  detect alien collisions with things
                let bspr: &sprite::LoadedSprite = &assets.sprites[assets::IX_LASER];
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
                                num_aliens_i - 1,
                            );
                            state
                                .explosions
                                .push(explosion::new(bullet.xpos, bullet.ypos));
                        }
                    }
                    //  TODO: check barrier collision
                    //  TODO: check player collision
                } //  endfor aliens

                if state.time_to_bomb <= 0.0 {
                    state.time_to_bomb += rand::gen_range(
                        BOMB_MIN_TIME,
                        BOMB_MIN_TIME
                            + BOMB_EXTRA_TIME * (4.0 / (3.0 + state.current_level as f32)),
                    );
                    let source = &state.aliens[rand::gen_range(0, num_aliens_i)];
                    state.bombs.push(Bomb {
                        sprite: &assets::BOMB_SPRITE,
                        xpos: source.xpos,
                        ypos: source.ypos,
                        phase: 0.0,
                        dead: false,
                    })
                }

                let playspr: &sprite::LoadedSprite = &assets.sprites[assets::IX_PLAYER];
                for bomb in state.bombs.iter_mut() {
                    if bomb.dead {
                        continue;
                    }
                    bomb.ypos += state.bomb_speed * delta_time;
                    if bomb.ypos > 1.33 {
                        bomb.dead = true;
                        hasdeadbomb = true;
                    } else if state.player_state == PlayerState::Playing
                        && bomb.ypos >= 1.33 - 0.06 - playspr.f_h * 0.5
                        && bomb.xpos >= state.player_pos_fr - playspr.f_w * 0.5
                        && bomb.xpos <= state.player_pos_fr + playspr.f_w * 0.5
                    {
                        bomb.dead = true;
                        hasdeadbomb = true;
                        state.player_state = PlayerState::HitExploding;
                        state.player_hit_timer = HIT_EXPLODE_TIME;
                        state
                            .explosions
                            .push(explosion::new_player(state.player_pos_fr, 1.33 - 0.06));
                        telemetry::tele_bombed(
                            state.player_pos_fr,
                            state.score,
                            num_aliens_i,
                            state.lives,
                        );
                    }
                }
            } //  endif evolving
        }

        //  TODO: barriers

        if hasdeadbullet {
            state.bullets.retain(|bullet| !bullet.dead);
        }
        if hasdeadalien {
            state.aliens.retain(|alien| !alien.dead);
            if state.aliens.len() == 0 {
                //  wave clear!
                state.reset_countdown = 1.2;
            }
        }
        if hasdeadbomb {
            state.bombs.retain(|bomb| !bomb.dead);
        }

        //  evolve explosions -- this happens even if everything else is frozen
        let mut hasdeadexplosion = false;
        for ex in state.explosions.iter_mut() {
            explosion::evolve(delta_time, ex);
            hasdeadexplosion = hasdeadexplosion || ex.dead;
        }
        if hasdeadexplosion {
            state.explosions.retain(|x| !x.dead);
        }

        if state.player_state == PlayerState::GameOver && mq::is_key_pressed(mq::KeyCode::Space) {
            state.current_level = 0;
            state.score = 0;
            state.lives = 2;
            state.reset_countdown = 1.2;
            state.player_state = PlayerState::Playing;
            state.aliens = Vec::new();
            state.bombs = Vec::new();
            state.bullets = Vec::new();
            state.explosions = Vec::new();
        } else {
            state.speed_ratio = 4.0 / (3.0 + num_aliens);
        }
    }
}
