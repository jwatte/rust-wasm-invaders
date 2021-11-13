//  assets.rs

use crate::sprite;
use futures::join;
use macroquad::audio;
use macroquad::prelude as mq;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt;

pub const ASSUMED_SCREEN_WIDTH: i32 = 2400;

pub struct Assets {
    pub sprites: Vec<sprite::LoadedSprite>,
    pub sounds: Vec<audio::Sound>,
    pub basslines: Vec<BasslineSound>,
}

const fn b_str(s: &'static str) -> Cow<'static, str> {
    return Cow::Borrowed(s);
}

pub const IX_PLAYER: usize = 0;
pub const IX_LASER: usize = 1;
pub const IX_ALIEN_1: usize = 2;
pub const IX_ALIEN_2: usize = 3;
pub const IX_ALIEN_3: usize = 4;
pub const IX_EXPLOSION_0: usize = 5;
pub const IX_EXPLOSION_1: usize = 6;
pub const IX_EXPLOSION_2: usize = 7;
pub const IX_EXPLOSION_3: usize = 8;
pub const IX_EXPLOSION_4: usize = 9;
pub const IX_EXPLOSION_5: usize = 10;
pub const IX_EXPLOSION_6: usize = 11;
pub const IX_EXPLOSION_7: usize = 12;
pub const IX_EXPLOSION_8: usize = 13;
pub const IX_BOMB: usize = 14;

pub static PLAYER_SPRITE: sprite::Sprite = sprite::Sprite {
    index: IX_PLAYER,
    filename: b_str("data/playerShip3_blue.png"),
    width_pix: 95,
    height_pix: 75,
    scale: 1.0,
};

pub static LASER_SPRITE: sprite::Sprite = sprite::Sprite {
    index: IX_LASER,
    filename: b_str("data/Lasers/laserBlue03.png"),
    width_pix: 9,
    height_pix: 37,
    scale: 1.0,
};

pub static ENEMY1_SPRITE: sprite::Sprite = sprite::Sprite {
    index: IX_ALIEN_1,
    filename: b_str("data/Enemies/enemyGreen3.png"),
    width_pix: 103,
    height_pix: 84,
    scale: 1.3,
};

pub static ENEMY2_SPRITE: sprite::Sprite = sprite::Sprite {
    index: IX_ALIEN_2,
    filename: b_str("data/Enemies/enemyBlue2.png"),
    width_pix: 93,
    height_pix: 84,
    scale: 1.35,
};

pub static ENEMY3_SPRITE: sprite::Sprite = sprite::Sprite {
    index: IX_ALIEN_3,
    filename: b_str("data/Enemies/enemyRed1.png"),
    width_pix: 104,
    height_pix: 84,
    scale: 1.3,
};

pub static BOMB_SPRITE: sprite::Sprite = sprite::Sprite {
    index: IX_BOMB,
    filename: b_str("data/Lasers/laserRed07.png"),
    width_pix: 9,
    height_pix: 37,
    scale: 1.0,
};

pub static EXPLOSION_SPRITES: [sprite::Sprite; 9] = [
    sprite::Sprite {
        index: IX_EXPLOSION_0,
        filename: b_str("data/Explosions/explosion00.png"),
        width_pix: 128,
        height_pix: 118,
        scale: 1.0,
    },
    sprite::Sprite {
        index: IX_EXPLOSION_1,
        filename: b_str("data/Explosions/explosion01.png"),
        width_pix: 128,
        height_pix: 118,
        scale: 1.0,
    },
    sprite::Sprite {
        index: IX_EXPLOSION_2,
        filename: b_str("data/Explosions/explosion02.png"),
        width_pix: 128,
        height_pix: 109,
        scale: 1.0,
    },
    sprite::Sprite {
        index: IX_EXPLOSION_3,
        filename: b_str("data/Explosions/explosion03.png"),
        width_pix: 128,
        height_pix: 142,
        scale: 1.0,
    },
    sprite::Sprite {
        index: IX_EXPLOSION_4,
        filename: b_str("data/Explosions/explosion04.png"),
        width_pix: 128,
        height_pix: 132,
        scale: 1.0,
    },
    sprite::Sprite {
        index: IX_EXPLOSION_5,
        filename: b_str("data/Explosions/explosion05.png"),
        width_pix: 128,
        height_pix: 122,
        scale: 1.0,
    },
    sprite::Sprite {
        index: IX_EXPLOSION_6,
        filename: b_str("data/Explosions/explosion06.png"),
        width_pix: 128,
        height_pix: 130,
        scale: 1.0,
    },
    sprite::Sprite {
        index: IX_EXPLOSION_7,
        filename: b_str("data/Explosions/explosion07.png"),
        width_pix: 128,
        height_pix: 105,
        scale: 1.0,
    },
    sprite::Sprite {
        index: IX_EXPLOSION_8,
        filename: b_str("data/Explosions/explosion08.png"),
        width_pix: 128,
        height_pix: 125,
        scale: 1.0,
    },
];

pub async fn load_sprite(spr: &'static sprite::Sprite) -> sprite::LoadedSprite {
    let fnb = spr.filename.borrow();
    /*
    this doesn't actually generate a good loading screen

    mq::clear_background(mq::Color::new(0.11, 0.11, 0.11, 1.00));
    let loaddisp = format!("Loading: {}", fnb);
    mq::draw_text(&loaddisp, 100.0, 100.0, 30.0, mq::WHITE);
    mq::next_frame().await;
    */

    mq::info!("loading {}", fnb);
    let texture = mq::load_texture(fnb).await.unwrap();
    return sprite::LoadedSprite {
        sprite: spr,
        texture: texture,
        f_w: (spr.width_pix as f32) / (ASSUMED_SCREEN_WIDTH as f32) * spr.scale,
        f_h: (spr.height_pix as f32) / (ASSUMED_SCREEN_WIDTH as f32) * spr.scale,
    };
}

pub struct BasslineSound {
    pub sound: audio::Sound,
    pub tempo: i32,
    pub duration: f32,
}

pub async fn load_bassline(tempo: i32) -> BasslineSound {
    let fm = format!("data/sound/bass-{}.mp3", tempo);
    mq::info!("loading {}", fm);
    let ret = audio::load_sound(&fm).await.unwrap();
    return BasslineSound {
        sound: ret,
        tempo: tempo,
        duration: 240.0 / (tempo as f32),
    };
}

pub const IX_SOUND_EXPLOSION: usize = 0;
pub const IX_SOUND_LASER: usize = 1;
pub const IX_SOUND_POP_01: usize = 2;
pub const IX_SOUND_POP_02: usize = 3;
pub const IX_SOUND_POP_03: usize = 4;
pub const IX_SOUND_POP_04: usize = 5;

pub async fn load_assets() -> Assets {
    let mut vec: Vec<sprite::LoadedSprite> = Vec::new();
    //  join!() macro only takes fixed-size tuple, not vec?
    let (
        s_player,
        s_laser,
        s_enemy1,
        s_enemy2,
        s_enemy3,
        s_exp0,
        s_exp1,
        s_exp2,
        s_exp3,
        s_exp4,
        s_exp5,
        s_exp6,
        s_exp7,
        s_exp8,
        s_bomb,
    ) = join!(
        load_sprite(&PLAYER_SPRITE),
        load_sprite(&LASER_SPRITE),
        load_sprite(&ENEMY1_SPRITE),
        load_sprite(&ENEMY2_SPRITE),
        load_sprite(&ENEMY3_SPRITE),
        load_sprite(&EXPLOSION_SPRITES[0]),
        load_sprite(&EXPLOSION_SPRITES[1]),
        load_sprite(&EXPLOSION_SPRITES[2]),
        load_sprite(&EXPLOSION_SPRITES[3]),
        load_sprite(&EXPLOSION_SPRITES[4]),
        load_sprite(&EXPLOSION_SPRITES[5]),
        load_sprite(&EXPLOSION_SPRITES[6]),
        load_sprite(&EXPLOSION_SPRITES[7]),
        load_sprite(&EXPLOSION_SPRITES[8]),
        load_sprite(&BOMB_SPRITE),
    );
    vec.push(s_player);
    vec.push(s_laser);
    vec.push(s_enemy1);
    vec.push(s_enemy2);
    vec.push(s_enemy3);
    vec.push(s_exp0);
    vec.push(s_exp1);
    vec.push(s_exp2);
    vec.push(s_exp3);
    vec.push(s_exp4);
    vec.push(s_exp5);
    vec.push(s_exp6);
    vec.push(s_exp7);
    vec.push(s_exp8);
    vec.push(s_bomb);
    mq::info!("loading sprites done");

    let mut snd: Vec<audio::Sound> = Vec::new();
    let (sn_explosion, sn_laser, sn_pop01, sn_pop02, sn_pop03, sn_pop04) = join!(
        audio::load_sound("data/sound/explosion.mp3"),
        audio::load_sound("data/sound/laser.mp3"),
        audio::load_sound("data/sound/pop-01.mp3"),
        audio::load_sound("data/sound/pop-02.mp3"),
        audio::load_sound("data/sound/pop-03.mp3"),
        audio::load_sound("data/sound/pop-04.mp3"),
    );
    snd.push(sn_explosion.unwrap());
    snd.push(sn_laser.unwrap());
    snd.push(sn_pop01.unwrap());
    snd.push(sn_pop02.unwrap());
    snd.push(sn_pop03.unwrap());
    snd.push(sn_pop04.unwrap());

    let mut bl: Vec<BasslineSound> = Vec::new();
    let (
        sn_60,
        sn_70,
        sn_80,
        sn_90,
        sn_100,
        sn_110,
        sn_120,
        sn_130,
        sn_140,
        sn_150,
        sn_160,
        sn_170,
        sn_180,
        sn_200,
        sn_220,
        sn_240,
        sn_260,
        sn_280,
        sn_300,
        sn_320,
        sn_340,
        sn_360,
    ) = join!(
        load_bassline(60),
        load_bassline(70),
        load_bassline(80),
        load_bassline(90),
        load_bassline(100),
        load_bassline(110),
        load_bassline(120),
        load_bassline(130),
        load_bassline(140),
        load_bassline(150),
        load_bassline(160),
        load_bassline(170),
        load_bassline(180),
        load_bassline(200),
        load_bassline(220),
        load_bassline(240),
        load_bassline(260),
        load_bassline(280),
        load_bassline(200),
        load_bassline(320),
        load_bassline(340),
        load_bassline(360),
    );
    bl.push(sn_60);
    bl.push(sn_70);
    bl.push(sn_80);
    bl.push(sn_90);
    bl.push(sn_100);
    bl.push(sn_110);
    bl.push(sn_120);
    bl.push(sn_130);
    bl.push(sn_140);
    bl.push(sn_150);
    bl.push(sn_160);
    bl.push(sn_170);
    bl.push(sn_180);
    bl.push(sn_200);
    bl.push(sn_220);
    bl.push(sn_240);
    bl.push(sn_260);
    bl.push(sn_280);
    bl.push(sn_300);
    bl.push(sn_320);
    bl.push(sn_340);
    bl.push(sn_360);

    mq::info!("loading sounds done");

    return Assets {
        sprites: vec,
        sounds: snd,
        basslines: bl,
    };
}
