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
use vpiper::codecs::{list_available_codecs};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn is_decoder_supported(mime:&str) -> bool {
    let supported = MediaSource::is_type_supported(&mime);
    console_log!("mime decoder support: {:?} {}", mime, supported);
    supported
}

fn is_encoder_supported(mime:&str) -> bool {
    let supported = MediaRecorder::is_type_supported(&mime);
    console_log!("mime encoder support: {:?} {}", mime, supported);
    supported
}

#[wasm_bindgen(js_name = "getCodecs")]
pub fn get_codecs() -> JsValue {
    let available = list_available_codecs(is_decoder_supported, is_encoder_supported);
    console_log!("codecs  available: {:?}", available);
    let codecs = available;
    JsValue::from_serde(&codecs).unwrap()
}