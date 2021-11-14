use crate::assets;
use crate::explosion;
use crate::state;
use macroquad::audio;

fn calc_bassline_index(cnt: usize, ratio: f32, level: i32) -> usize {
    return ((cnt as f32 * ratio) as usize + level as usize).clamp(0, cnt - 1);
}

pub fn update_sounds(delta_time: f32, st: &mut state::State, ass: &assets::Assets) {
    if !st.paused && st.player_state != state::PlayerState::GameOver {
        if st.reset_countdown <= 0.0 {
            st.bassline_time -= delta_time;
            if st.bassline_time <= 0.0 {
                let ix = calc_bassline_index(ass.basslines.len(), st.speed_ratio, st.current_level);
                st.bassline_time = ass.basslines[ix].duration;
                audio::play_sound_once(ass.basslines[ix].sound);
            }
        } else {
            st.bassline_time = 0.0;
        }
        for bu in st.bullets.iter_mut() {
            if bu.fresh {
                audio::play_sound_once(ass.sounds[assets::IX_SOUND_LASER]);
                bu.fresh = false;
            }
        }
        for ex in st.explosions.iter_mut() {
            //  spawning sounds in the state, not the render, is a bit ugly
            if ex.fresh {
                audio::play_sound_once(ass.sounds[ex.sound]);
                ex.fresh = false;
            }
        }
    }
}
