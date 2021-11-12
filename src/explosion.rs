//  explosion.rs

use crate::assets;
use crate::sprite;
use macroquad::rand;

pub struct Explosion {
    pub sprite: &'static sprite::Sprite,
    pub xpos: f32,
    pub ypos: f32,
    pub speed: f32,
    pub phase: f32,
    pub sound: usize,
    pub fresh: bool,    //  used for sound
    pub growing: bool,
    pub dead: bool,
}

pub fn new(xpos: f32, ypos: f32) -> Explosion {
    let ix: usize = rand::gen_range(0, 9);
    let six: usize = rand::gen_range(0, 4) + assets::IX_SOUND_POP_01;
    return Explosion {
        sprite: &assets::EXPLOSION_SPRITES[ix],
        xpos: xpos,
        ypos: ypos,
        speed: 4.0,
        phase: 0.2,
        sound: six,
        fresh: true,
        growing: true,
        dead: false,
    };
}

pub fn evolve(dt: f32, x: &mut Explosion) {
    x.phase += dt * if x.growing { x.speed } else { -x.speed };
    if x.phase > 1.0 {
        x.phase = 1.0;
        x.growing = false;
    }
    if x.phase < 0.0 {
        x.phase = 0.0;
        x.dead = true;
    }
}

pub fn render(left: f32, top: f32, width: f32, x: &Explosion, ass: &assets::Assets) {
    sprite::draw_sprite(
        left,
        top,
        width,
        &ass.sprites[x.sprite.index],
        x.xpos,
        x.ypos,
        0.0,
        x.phase,
    );
}
