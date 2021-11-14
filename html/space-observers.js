//  A plugin for macroquad game engine, using sapp_jsutils for intgration
//  See also:
//  https://github.com/not-fl3/miniquad-js-interop-demo/blob/master/js/demo.js

"use strict";

const clog = (console && console.log) || function () { }

function make_id_string() {
    const randArray = new Uint32Array(4);
    crypto.getRandomValues(randArray);
    return randArray[0].toString(16) + '-' + randArray[1].toString(16) + "-" + randArray[2].toString(16) + "-" + randArray[3].toString(16);
}

var observeTeleData;
if (window.localStorage.observeTeleData) {
    try {
        observeTeleData = JSON.parse(window.localStorage.observeTeleData);
    } catch (error) {
        clog(`localStorage error: ${error}`);
        observeTeleData = null;
    }
}
if (!observeTeleData || !observeTeleData.playerid) {
    const playerid = make_id_string();
    observeTeleData = {
        username: "",
        playerid: playerid,
    };
}
observeTeleData.sessionid = make_id_string();
window.localStorage.observeTeleData = JSON.stringify(observeTeleData);
clog(`playerid=${observeTeleData.playerid} sessionid=${observeTeleData.sessionid}`);

const BACKEND_URL = "https://watte.net/space-backend.php";
const BACKEND_AUTH = "Bearer 598c4bcd-e454-4fba-a87c-1068f5828eb4";

let onHighscoreLoaded = null;

function fetch_highscores() {
    let bereq = new XMLHttpRequest();
    bereq.onloadend = function (e) {
        let r = "" + bereq.status + " " + bereq.statusText;
        if (bereq.status >= 300 && bereq.responseText) {
            r = r + " " + bereq.responseText;
        }
        clog(`fetch_highscores: `, r);
        if (bereq.status >= 200 && bereq.status < 300) {
            load_highscores(bereq.responseText);
        }
    };
    bereq.open("GET", BACKEND_URL, true);
    bereq.setRequestHeader('Content-Type', 'text/json');
    bereq.setRequestHeader('Authorization', BACKEND_AUTH);
    bereq.send();
}

function load_highscores(text) {
    let dec = JSON.parse(text);
    if (dec && dec.highscores && dec.highscores.length > 0) {
        highscores = dec.highscores;
        if (onHighscoreLoaded) {
            onHighscoreLoaded(highscores);
        }
    }
}

function blind_backend_post(arg) {
    let bereq = new XMLHttpRequest();
    bereq.onloadend = function (e) {
        let r = "" + bereq.status + " " + bereq.statusText;
        if (bereq.status >= 300 && bereq.responseText) {
            r = r + " " + bereq.responseText;
        }
        clog(`blind_backend_post: `, r);
        if (bereq.status >= 200 && bereq.status < 300) {
            load_highscores(bereq.responseText);
        }
    };
    bereq.open("POST", BACKEND_URL, true);
    bereq.setRequestHeader('Content-Type', 'text/json');
    bereq.setRequestHeader('Authorization', BACKEND_AUTH);
    bereq.send(JSON.stringify(arg));
}

function post_session_data() {
    blind_backend_post({
        request: "onload",
        teledata: observeTeleData,
    });
}

let highscores = [
    { name: "AAA", score: 100.0 },
    { name: "BBB", score: 90.0 },
    { name: "CCC", score: 80.0 },
    { name: "DDD", score: 70.0 },
    { name: "EEE", score: 60.0 },
    { name: "FFF", score: 50.0 },
    { name: "GGG", score: 40.0 },
    { name: "HHH", score: 30.0 },
    { name: "III", score: 20.0 },
    { name: "JJJ", score: 10.0 },
    { name: "KKK", score: 0.0 },
];

function register_highscore(arg) {
    arg = consume_js_object(arg);
    let atend = true;
    const score = 0 + arg.score;
    const username = observeTeleData.username;
    for (let i = 0; i != 10; i++) {
        if (highscores[i].score < arg.score) {
            highscores.splice(i, 0, { name: username, score: score });
            if (highscores.length > 11) {
                highscores.splice(11, highscores.length - 11);
            }
            atend = false;
            break;
        }
    }
    if (atend) {
        //  if I didn't beat anyone, then tack me on at the end
        highscores[10] = { name: username, score: score };
    }
    blind_backend_post({
        request: "highscore",
        teledata: observeTeleData,
        score: {
            name: username,
            score: 0 + arg.score,
        },
    });
}

function read_highscores() {
    let ret = {};
    //  because of interop shenanigans, I need a legit object with keys, not an array
    for (let i = 0, n = highscores.length(); i != n; i++) {
        ret[`${i}`] = highscores[i];
    }
    return ret;
}

// Will be called when wasm_exports and wasm_memory will be available
function on_init() {
    /// Call rust app function with string argument
    flush_telemetry_queue();
}

function register_plugin(importObject) {
    // make our functions available to call from rust/wasm app
    importObject.env.queue_telemetry = queue_telemetry;
    importObject.env.register_highscore = register_highscore;
    importObject.env.read_highscores = read_highscores;
}

// register this plugin in miniquad, required to make plugin's functions available from rust
miniquad_add_plugin({ register_plugin, on_init });

const TELEMETRY_URL = "https://collect.observe-eng.com/v1/http/invaders";
const AUTHORIZATION_HEADER = "Bearer 101 4vVFnBaMXQ9LovF-HxIJGVgxG2V7dmRo";
var QUEUE = [];
var pendingRequest = null;

function flush_telemetry_queue() {
    const alreadyPending = (pendingRequest !== null);
    //clog(`flush_telemetry_queue length=${QUEUE.length} alreadyPending=${alreadyPending}`);
    if (QUEUE.length > 0 && !alreadyPending) {
        let toflush = QUEUE;
        QUEUE = [];
        pendingRequest = new XMLHttpRequest();
        pendingRequest.onloadend = function (e) {
            let r = "" + pendingRequest.status + " " + pendingRequest.statusText;
            if (pendingRequest.status >= 300 && pendingRequest.responseText) {
                r = r + " " + pendingRequest.responseText;
            }
            clog(`telemetry count=${toflush.length}: `, r);
            pendingRequest = null;
        };
        pendingRequest.open("POST", TELEMETRY_URL, true);
        pendingRequest.setRequestHeader('content-type', 'application/x-ndjson');
        pendingRequest.setRequestHeader('authorization', AUTHORIZATION_HEADER);
        //  turn the objects into ndjson
        //  TODO: there's a slim chance that this marshaling will actually take enough time to cause a frame hitch.
        //  To solve that, we could be sending from a WebWorker. Important future direction!
        pendingRequest.send(toflush.map((x) => JSON.stringify(x)).join("\n") + "\n");
    }
}

//  Flush the queue every 2 seconds, and when it reaches 50 elements
setInterval(flush_telemetry_queue, 2000);
window.onunload = function () {
    //  If there are data, force the flush even if another one is already pending
    pendingRequest = null;
    flush_telemetry_queue();
}

//  This function should probably be a postMessage() on a WebWorker
function queue_telemetry(typearg, objarg) {
    let type = consume_js_object(typearg);
    let obj = consume_js_object(objarg);
    let payload = {
        metadata: {
            //  boo hiss -- this never increments by the delta, because of 53-bit doubles
            timestamp: Date.now() * 1000000 + QUEUE.length,
            username: observeTeleData.username,
            playerid: observeTeleData.playerid,
            sessionid: observeTeleData.sessionid,
        },
        type: type,
        data: obj,
    }
    QUEUE.push(payload);
    //console.log('queue_telemetry', payload);
    if (QUEUE.length >= 50) {
        flush_telemetry_queue();
    }
}
