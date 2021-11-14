//  highscore.rs

use sapp_jsutils::JsObject;
use std::fmt;

#[derive(PartialEq)]
pub struct Score {
    name: String,
    score: i32,
}

extern "C" {
    fn register_highscore(arg: JsObject);
    fn read_highscores() -> JsObject;
}

pub fn register(score: i32) {
    let obj = JsObject::object();
    //  Include game parameters
    obj.set_field_f32("score", score as f32);
    unsafe {
        register_highscore(obj);
    }
}

pub fn get_list() -> Vec<Score> {
    let mut ret: Vec<Score> = Vec::new();
    unsafe {
        let obj = read_highscores();
        for ix in 0..9 {
            let ixs = std::format!("{}", ix);
            let sc = obj.field(&ixs);
            let mut name = String::new();
            sc.field("name").to_string(&mut name);
            let score = sc.field_u32("score") as i32;
            ret.push(Score {
                name: name.clone(),
                score: score,
            });
        }
    }
    return ret;
}
