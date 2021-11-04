//  A plugin for macroquad game engine, using sapp_jsutils for intgration
//  See also:
//  https://github.com/not-fl3/miniquad-js-interop-demo/blob/master/js/demo.js

"use strict";

const clog = (console && console.log) || function() {}

function make_id_string() {
    const randArray = new Uint32Array(4);
    crypto.getRandomValues(randArray);
    return randArray[0].toString(16) + '-' + randArray[1].toString(16) + "-" + randArray[2].toString(16) + "-" + randArray[3].toString(16);
}

var observeTeleData;
if (window.localStorage.observeTeleData) {
    try {
        observeTeleData = JSON.parse(window.localStorage.observeTeleData);
    } catch(error) {
        clog(`localStorage error: ${error}`);
        observeTeleData = null;
    }
}
if (!observeTeleData || !observeTeleData.playerid) {
    const playerid = make_id_string();
    observeTeleData = {
        username: "Player1",
        playerid: playerid,
    };
}
observeTeleData.sessionid = make_id_string();
window.localStorage.observeTeleData = JSON.stringify(observeTeleData);
clog(`playerid=${observeTeleData.playerid} sessionid=${observeTeleData.sessionid}`);
    
// Will be called when wasm_exports and wasm_memory will be available
function on_init() {
    /// Call rust app function with string argument
    flush_queue();
}

function register_plugin(importObject) {
    // make our functions available to call from rust/wasm app
    importObject.env.queue_telemetry = queue_telemetry;
}

// register this plugin in miniquad, required to make plugin's functions available from rust
miniquad_add_plugin({ register_plugin, on_init });

const TELEMETRY_URL = "https://collect.observe-eng.com/v1/http/invaders";
const AUTHORIZATION_HEADER = "Bearer 101 4vVFnBaMXQ9LovF-HxIJGVgxG2V7dmRo";
var QUEUE = [];
var pendingRequest = null;

function flush_queue() {
    const alreadyPending = (pendingRequest !== null);
    //clog(`flush_queue length=${QUEUE.length} alreadyPending=${alreadyPending}`);
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
        pendingRequest.send(toflush.map((x) => JSON.stringify(x)).join("\n")+"\n");
    }
}

//  Flush the queue every 2 seconds, and when it reaches 50 elements
setInterval(flush_queue, 2000);
window.onunload = function() {
    //  If there are data, force the flush even if another one is already pending
    pendingRequest = null;
    flush_queue();
}

//  This function should probably be a postMessage() on a WebWorker
function queue_telemetry(typearg, objarg) {
    let type = get_js_object(typearg);
    let obj = get_js_object(objarg);
    let payload = {
        metadata:{
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
        flush_queue();
    }
}
