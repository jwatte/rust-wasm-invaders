//  assets.rs

use crate::sprite::*;
use macroquad::prelude::*;
use std::borrow::Cow;
use std::borrow::Borrow;

pub const ASSUMED_SCREEN_WIDTH: i32 = 2400;


pub struct Assets {
    pub sprites: Vec<LoadedSprite>,
}

const fn b_str(s: &'static str) -> Cow<'static, str> {
    return Cow::Borrowed(s);
}

pub const IX_PLAYER : usize = 0;
pub const IX_LASER : usize = 1;
pub const IX_ALIEN_1 : usize = 2;
pub const IX_ALIEN_2 : usize = 3;
pub const IX_ALIEN_3 : usize = 4;

pub static PLAYER_SPRITE : Sprite = Sprite {
    index: IX_PLAYER,
    filename: b_str("data/playerShip3_blue.png"),
    width_pix: 95,
    height_pix: 75,
    scale: 1.0,
};

pub static LASER_SPRITE : Sprite = Sprite {
    index: IX_LASER,
    filename: b_str("data/Lasers/laserBlue03.png"),
    width_pix: 9,
    height_pix: 37,
    scale: 1.0,
};

pub static ENEMY1_SPRITE : Sprite = Sprite {
    index: IX_ALIEN_1,
    filename: b_str("data/Enemies/enemyGreen3.png"),
    width_pix: 103,
    height_pix: 84,
    scale: 1.3,
};

pub static ENEMY2_SPRITE : Sprite = Sprite {
    index: IX_ALIEN_2,
    filename: b_str("data/Enemies/enemyBlue2.png"),
    width_pix: 93,
    height_pix: 84,
    scale: 1.35,
};

pub static ENEMY3_SPRITE : Sprite = Sprite {
    index: IX_ALIEN_3,
    filename: b_str("data/Enemies/enemyRed1.png"),
    width_pix: 104,
    height_pix: 84,
    scale: 1.3,
};

pub async fn load_sprite(spr : &'static Sprite) -> LoadedSprite {
    let texture = load_texture(spr.filename.borrow()).await.unwrap();
    return LoadedSprite{
        sprite: spr,
        texture: texture,
        f_w: (spr.width_pix as f32) / (ASSUMED_SCREEN_WIDTH as f32) * spr.scale,
        f_h: (spr.height_pix as f32) / (ASSUMED_SCREEN_WIDTH as f32) * spr.scale,
    };
}

pub async fn load_assets() -> Assets {
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

