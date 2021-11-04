//  sprite.rs

use macroquad::prelude as mq;
use std::borrow::Cow;

pub fn draw_sprite(left : f32, top : f32, width : f32, spr: &LoadedSprite, x: f32, y: f32, r: f32, s: f32) {
    let dtp = mq::DrawTextureParams {
        dest_size: Some(mq::vec2(
            width * spr.f_w * s,
            width * spr.f_h * s)),
        rotation: r,
        pivot: Some(mq::vec2(left+width*x, top + width*y)),
        ..Default::default()
    };
    mq::draw_texture_ex(
        spr.texture,
        left + width * (x - spr.f_w*0.5*s),
        top + width * (y - spr.f_w*0.5*s),
        mq::WHITE,
        dtp,
    );
}

pub struct Sprite {
    pub index: usize,
    pub filename: Cow<'static, str>,
    pub width_pix: i32,
    pub height_pix: i32,
    pub scale: f32,
}

pub struct LoadedSprite {
    pub sprite: &'static Sprite,
    pub texture: mq::Texture2D,
    pub f_w: f32,
    pub f_h: f32,
}
