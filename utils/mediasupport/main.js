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

const commonCodecs = {
  h264: {
    fullname: 'Advanced Video Coding',
    syntax: 'avc1.[.PPCCLL]',
    mime: 'avc1',
    mimes: ['h264', 'h.264', 'avc1'],
    profiles: [{ name: 'Baseline', code: '42000a' }],
    levels: [{ name: '1', code: '0a' }],
    containers: ['mp4', 'webm'], // Firefox uses WebM as container : https://bugzilla.mozilla.org/show_bug.cgi?id=1562862
    audio: ['mp4a'],
  },
  av1: {
    fullname: 'AOMedia Video 1',
    syntax: 'av01.P.LLT.DD[.M[.CCC[.cp[.tc[.mc[.F]]]]]]',
    mime: 'av01',
    mimes:['av1'],
    profiles: [{ name: '0', code: '0.00M.08' }],
    containers: ['mp4', 'webm'],
    audio: ['mp4a'],
  },
  h265: {
    fullname: 'High Efficiency Video Coding',
    mime: 'hev1',
    mimes: ['hvc1'],
    profiles: ['1.6.L93.B0'],
    containers: ['mp4'],
    audio: ['mp4a'],
  },
  vp9: {
    fullname: 'Video Processor 9',
    mime: 'vp09',
    mimes: ['vp9', 'vp9.0'],
    profiles: ['00.50.08'],
    containers: ['mp4', 'webm'],
    audio: ['mp4a', 'opus', 'vorbis'],
  },
  vp8: {
    fullname: 'Video Processor 8',
    mime: 'vp8',
    mimes: ['vp8.0'],
    profiles: [],
    containers: ['mp4', 'webm'],
    audio: ['mp4a', 'opus', 'vorbis'],
  }
}
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
