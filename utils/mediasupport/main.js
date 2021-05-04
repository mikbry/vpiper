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

const codecs = {
  video: [
    ['h264', 'h.264', 'avc1', 'avc1.42000a'],
    ['h265', 'h.265', 'hevc', 'hvc1', 'hev1', 'hev1.1.6.L93.B0', 'hvc1.1.6.L93.B0'],
    ['vp9', 'vp9.0'],
    ['vp8', 'vp8.0'],
    ['av1', 'av01', 'av01.0.00M.08'],
  ],
  audio: [
    ['mp4a'],
    ['opus'],
    ['vorbis'],
  ]
};

const containers = [
  'mp4',
  'webm',
  'x-matroska'
];

let supportedTypes;

function getSupportedMimeTypes() {
  supportedTypes = [];
  // TODO handle audio
  containers.forEach((container) => {
    codecs.video.forEach((codecRefs) => {
      codecRefs.forEach((codec) => {
        const mime = `video/${container};codecs=${codec}`;
        let mediaSourceSupport = MediaSource.isTypeSupported(mime);
        let mediaRecorderSupport = false;
        try {
          mediaRecorderSupport = MediaRecorder.isTypeSupported(mime);
        } catch (e) {
          console.log("No MediaRecorder");
        }
        supportedTypes.push({ name: mime, mediaSourceSupport, mediaRecorderSupport });
      });
    });
  });

  return supportedTypes;
}
