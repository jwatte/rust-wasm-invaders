use macroquad::prelude as mq;
mod assets;
mod explosion;
mod params;
mod sounds;
mod sprite;
mod state;
mod telemetry;

/*

TODO:

- reset field when all aliens dead
- increment pace for each reset
- alien bombs
- bottom shields
- top UFO
- highscores

*/

#[macroquad::main("Invaders")]
async fn main() {
    telemetry::tele_startup();

    draw_intro().await;

    let assets = assets::load_assets().await;

    let mut state = state::new_game_state();

    telemetry::tele_loading_done();

    //  make sure frame time isn't vastly off the first time it runs
    mq::next_frame().await;
    mq::next_frame().await;

    loop {
        let delta_time = mq::get_frame_time().min(0.1);

        state::update_state(delta_time, &mut state, &assets);

        sounds::update_sounds(delta_time, &mut state, &assets);

        render_scene(&state, &assets);

        mq::next_frame().await;
    }
}

async fn draw_intro() {
    let mut accumtime = 0.0;
    while accumtime < 1.0 {
        mq::clear_background(mq::Color::new(0.11, 0.11, 0.11, 1.0));
        mq::draw_text("Loading...", 100.0, 100.0, 48.0, mq::WHITE);
        mq::next_frame().await;
        accumtime += mq::get_frame_time().min(0.1);
    }
}

fn render_scene(state: &state::State, assets: &assets::Assets) {
    mq::clear_background(mq::Color::new(0.11, 0.11, 0.11, 1.00));

    let sw = mq::screen_width();
    let sh = mq::screen_height();
    let (left, top, width, height) = letterbox(sw, sh);

    {
        //  draw lives
        let pspr = &assets.sprites[0];
        for n in 0..state.lives {
            sprite::draw_sprite(
                left,
                top,
                width,
                pspr,
                (n as f32 + 0.6) * pspr.f_w * 0.5,
                1.333 - pspr.f_h * 0.6 * 0.45,
                0.0,
                0.45,
            );
        }

        //  draw player
        sprite::draw_sprite(
            left,
            top,
            width,
            pspr,
            state.player_pos_fr,
            1.333 - 0.06,
            0.0,
            1.0,
        );
    }

    //  draw aliens
    for alien in state.aliens.iter() {
        let asp = &assets.sprites[alien.sprite.index];
        sprite::draw_sprite(
            left,
            top,
            width,
            asp,
            alien.xpos,
            alien.ypos,
            alien.phase.sin() * 0.1,
            1.0,
        );
    }

    for ex in state.explosions.iter() {
        explosion::render(left, top, width, ex, &assets);
    }

    {
        let bspr = &assets.sprites[1];
        //  draw bullets
        for bullet in state.bullets.iter() {
            sprite::draw_sprite(
                left,
                top,
                width,
                bspr,
                bullet.xpos,
                bullet.ypos + bspr.f_h * 0.5,
                0.0,
                1.0,
            );
        }
    }

    //  draw masking bars
    mq::draw_rectangle(0.0, 0.0, left, sh, mq::BLACK);
    mq::draw_rectangle(left, 0.0, width, top, mq::BLACK);
    mq::draw_rectangle(left + width, 0.0, sw - (left + width), sh, mq::BLACK);
    mq::draw_rectangle(left, top + height, width, sh - (top + height), mq::BLACK);

    //  draw score
    let scoredisp = format!("Score: {}", state.score);
    mq::draw_text(
        &scoredisp,
        left + width * 0.01,
        top + height * 0.03,
        height * 0.04,
        mq::WHITE,
    );

    if state.paused {
        let pdisp = format!("Press ESC to Unpause");
        mq::draw_rectangle(
            left + width * 0.2,
            top + height * 0.15,
            width * 0.6,
            height * 0.08,
            mq::BLACK,
        );
        mq::draw_text(
            &pdisp,
            left + width * 0.25,
            top + height * 0.2,
            height * 0.04,
            mq::WHITE,
        );
    } else if state.reset_countdown > 0.0 {
        let ctdisp = format!("Countdown: {:.1}", state.reset_countdown);
        mq::draw_text(
            &ctdisp,
            left + width * 0.3,
            top + height * 0.2,
            height * 0.04,
            mq::WHITE,
        );
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
