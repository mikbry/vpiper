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

use serde::{Deserialize, Serialize};

pub static VIDEO_MIME: &str = "video";
pub static AUDIO_MIME: &str = "audio";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AVCodecProfile<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AVCodecLevel<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AVCodecDescriptor<'a> {
    pub name: &'a str,
    pub fullname: &'a str,
    pub syntax: &'a str,
    pub mime: &'a str,
    pub mimes: Vec<&'a str>,
    pub profiles: Vec<AVCodecProfile<'a>>,
    pub levels: Vec<AVCodecLevel<'a>>,
    pub containers: Vec<&'a str>,
    pub audio: Vec<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AVCodec<'a> {
    pub name: &'a str,
    pub mime: &'a str,
    pub decoder: bool,
    pub encoder: bool,
}

lazy_static! {
 static ref COMMON_DESCRIPTIONS: Vec<AVCodecDescriptor<'static>> = vec![
 AVCodecDescriptor {
     name: "H264 AVC1",
     fullname: "Advanced Video Coding",
     syntax: "avc1.[.PPCCLL]",
     mime: "avc1",
     mimes: vec!["h264", "h.264", "avc1"],
     profiles: vec![AVCodecProfile { name: "Baseline", code: "42000a" }],
     levels: vec![AVCodecLevel { name: "1", code: "0a" }],
     containers: vec!["mp4", "webm"], // Firefox uses WebM as container : https://bugzilla.mozilla.org/show_bug.cgi?id=1562862
     audio: vec!["mp4a"],
   },
   AVCodecDescriptor {
    name: "AV1",
    fullname: "AOMedia Video 1'",
    syntax: "av01.P.LLT.DD[.M[.CCC[.cp[.tc[.mc[.F]]]]]]",
    mime: "av01",
    mimes: vec!["av1"],
    profiles: vec![AVCodecProfile { name: "0", code: "0.00M.08" }],
    levels: vec![AVCodecLevel { name: "1", code: "0a" }],
    containers: vec!["mp4", "webm"],
    audio: vec!["mp4a"],
  }
];
 }

pub fn list_available_codecs<'a>(
    is_decoder_supported: fn(mime: &str) -> bool,
    is_encoder_supported: fn(mime: &str) -> bool,
) -> Box<[AVCodec<'a>]> {
    let mut codecs: Vec<AVCodec<'a>> = vec![];
    for desc in COMMON_DESCRIPTIONS.iter() {
        match desc {
            desc => {
                let mime = format!(
                    "{}/{};codecs={}.{}",
                    VIDEO_MIME, desc.containers[0], desc.mime, desc.profiles[0].code
                );
                let decoder_supported = is_decoder_supported(&mime);
                let encoder_supported = is_encoder_supported(&mime);
                if decoder_supported || encoder_supported {
                    let codec: AVCodec<'a> = AVCodec {
                        name: desc.name,
                        mime: desc.mime,
                        decoder: decoder_supported,
                        encoder: encoder_supported,
                    };
                    codecs.push(codec);
                }
            }
        }
    }
    codecs.into_boxed_slice()
}
