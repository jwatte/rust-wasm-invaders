use macroquad::prelude::*;
mod assets;
use crate::assets::*;
mod params;
mod sprite;
use crate::sprite::*;
mod state;
use crate::state::*;
mod telemetry;
use crate::telemetry::*;


#[macroquad::main("Invaders")]
async fn main() {
    tele_startup();

    let mut state = new_game_state();
    let assets = load_assets().await;

    loop {
        update_state(&mut state, &assets);

        render_scene(&state, &assets);

        next_frame().await;
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

//  Given width/height, which part of the screen do we draw to?
fn letterbox(sw: f32, sh: f32) -> (f32, f32, f32, f32) {
    let (ww, wh) = if sh * 0.75 > sw {
        (sw, sw / 0.75)
    } else {
        (sh * 0.75, sh)
    };
    return (((sw - ww) / 2.0).floor(), ((sh - wh) / 2.0).floor(), ww, wh);
}
