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

use serde::{Serialize, Deserialize};

pub static VIDEO_MIME: &str = "video";
pub static AUDIO_MIME: &str = "video";

#[derive(Serialize, Deserialize)]
pub struct AVCodec {
    pub name: String,
    pub mime: String,
    pub decoder: bool,
    pub encoder: bool,
}
