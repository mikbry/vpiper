Media support

Get codecs from MediaSource MediaRecorder and return a list of all available

Experiment for `$ffmeg -codecs`

Minimal support we need to achieve

```
Codecs:
 D..... = Decoding supported
 .E.... = Encoding supported
 ..V... = Video codec
 ..A... = Audio codec
 ..S... = Subtitle codec
 ...I.. = Intra frame-only codec
 ....L. = Lossy compression
 .....S = Lossless compression

 DEV.L. av1                  Alliance for Open Media AV1 (decoders: libdav1d libaom-av1 ) (encoders: libaom-av1 librav1e )
 DEV..S gif                  GIF (Graphics Interchange Format)
 DEV.LS h264                 H.264 / AVC / MPEG-4 AVC / MPEG-4 part 10 (encoders: libx264 libx264rgb )
 D.V.L. hevc                 H.265 / HEVC (High Efficiency Video Coding)
 DEVILS jpeg2000             JPEG 2000
 DEV..S png                  PNG (Portable Network Graphics) image
 DEV.L. vp9                  Google VP9 (decoders: vp9 libvpx-vp9 ) (encoders: libvpx-vp9 )
 D.VILS webp                 WebP
..A.L. opus                 Opus (Opus Interactive Audio Codec)
DEA.L. vorbis               Vorbis
 ```

 Inspiration

 http://dmlinking.net/~pe1rxq/video.html

 https://stackoverflow.com/questions/41739837/all-mime-types-supported-by-mediarecorder-in-firefox-and-chrome

 https://cconcolato.github.io/media-mime-support/

 https://research.akamaized.net/capabilitiescheck.html
