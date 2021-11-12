//  telemetry.rs
use crate::params;
use sapp_jsutils::JsObject;
use std::borrow::Borrow;

/* for telemetry, check out:
    https://github.com/not-fl3/quad-net/blob/master/examples/http_request.rs

    See also:
    https://github.com/not-fl3/miniquad-js-interop-demo/blob/master/js/demo.js
*/

extern "C" {
    fn queue_telemetry(argtype: JsObject, arg: JsObject);
}

pub fn tele_startup() {
    let obj = JsObject::object();
    //  Include game parameters
    obj.set_field_string("VERSION", params::VERSION);
    obj.set_field_f32("HORIZ_SPEED", params::HORIZ_SPEED);
    obj.set_field_f32("VERT_SPEED", params::VERT_SPEED);
    obj.set_field_f32("RIGHT_MARGIN", params::RIGHT_MARGIN);
    obj.set_field_f32("LEFT_MARGIN", params::LEFT_MARGIN);
    obj.set_field_f32("DOWN_DISTANCE", params::DOWN_DISTANCE);
    obj.set_field_f32("PHASE_SPEED", params::PHASE_SPEED);
    let kind = JsObject::string("start");
    unsafe {
        queue_telemetry(kind, obj);
    }
}

pub fn tele_loading_done() {
    let obj = JsObject::object();
    //  Include game parameters
    let kind = JsObject::string("loading_done");
    unsafe {
        queue_telemetry(kind, obj);
    }
}

pub fn tele_new_level(level: i32, score: i32) {
    let obj = JsObject::object();
    obj.set_field_string("level", format!("{}", level).borrow());
    obj.set_field_f32("score", score as f32);
    let kind = JsObject::string("new_level");
    unsafe {
        queue_telemetry(kind, obj);
    }
}

pub fn tele_shot(xpos: f32, score: i32, alien_y: f32) {
    let obj = JsObject::object();
    obj.set_field_f32("xpos", xpos);
    obj.set_field_f32("score", score as f32);
    obj.set_field_f32("alien_y", alien_y as f32);
    let kind = JsObject::string("shot");
    unsafe {
        queue_telemetry(kind, obj);
    }
}

pub fn tele_hit(xpos: f32, score: i32, kind: usize, points: i32, remaining: usize) {
    let obj = JsObject::object();
    obj.set_field_f32("xpos", xpos);
    obj.set_field_f32("score", score as f32);
    obj.set_field_string("kind", format!("{}", kind).borrow());
    obj.set_field_f32("points", points as f32);
    obj.set_field_f32("remaining", remaining as f32);
    let kind = JsObject::string("hit");
    unsafe {
        queue_telemetry(kind, obj);
    }
}

pub fn tele_miss(xpos: f32, score: i32, remaining: usize) {
    let obj = JsObject::object();
    obj.set_field_f32("xpos", xpos);
    obj.set_field_f32("score", score as f32);
    obj.set_field_f32("remaining", remaining as f32);
    let kind = JsObject::string("miss");
    unsafe {
        queue_telemetry(kind, obj);
    }
}

pub fn tele_advance(alien_y: f32, remaining: usize) {
    let obj = JsObject::object();
    obj.set_field_f32("alien_y", alien_y);
    obj.set_field_f32("remaining", remaining as f32);
    let kind = JsObject::string("advance");
    unsafe {
        queue_telemetry(kind, obj);
    }
}

pub fn tele_pause(paused: bool) {
    let obj = JsObject::object();
    obj.set_field_string("paused", format!("{}", paused).borrow());
    let kind = JsObject::string("pause");
    unsafe {
        queue_telemetry(kind, obj);
    }
}
