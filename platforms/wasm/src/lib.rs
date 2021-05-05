// Copyright 2021 mik
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use wasm_bindgen::prelude::*;
use web_sys::{MediaSource, MediaRecorder};
use vpiper::codecs::{AVCodec, VIDEO_MIME};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(js_name = "getCodecs")]
pub fn get_codecs() -> JsValue {
    let mime = format!("{}/mp4", &VIDEO_MIME);
    let media_source_support = MediaSource::is_type_supported(&mime);
    let media_recorder_support = MediaRecorder::is_type_supported(&mime);
    console_log!("mediaSource support {:?}  {:?}", mime,media_source_support);
    let codec = AVCodec {
        name: String::from("mp4"),
        mime: mime.to_string(),
        decoder: media_source_support,
        encoder: media_recorder_support,
    };
    let codecs = [codec];
    JsValue::from_serde(&codecs).unwrap()
}