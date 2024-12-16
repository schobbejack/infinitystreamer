# Infinity Streamer
Infinity Streamer is a small demo/test project to generate a neverending HLS stream.
A single pre-encoded audio and video mp4 fragment are repeated in a live playlist with a media sequence calculated from the time since the unix epoch.

## Build
### Local
```sh
$ cargo build
```
Will result in a binary at target/debug/infinitystreamer
### Docker
```sh
$ docker build -t infinitystreamer .
```
Start the image with
```sh
$ docker run -d --rm -p 3001:80 infinitystreamer
```
The stream can be reached at http://localhost:3001/master.m3u8

## Looping fragment
To get audio and video fragments of equal length, we need to find a common length that fits both the 1024 sample AAC frame and the duration of a video frame at a chosen framerate.  
One way to calculate the required fragment length, is to put the durations of a video and audio frame in a  common timebase. For most cases 90000 will work. Get the GCD of the duration of a video and AAC frame (1024/48000).  
Divide the duration of the AAC frame by the GCD to get the number of required video frames, and divide the duration of a video frame by the GCD to get the number of required AAC frames.
Taking 25fps for example:
```
90000 / 25 = 3600
1024*90000 / 48000 = 1920
GCD(3600,1920) = 240
1920 / 240 = 8 video frames, 8 / 25 = 0.32 seconds
3600 / 240 = 15 AAC frames, 15 * 1024 / 48000 = 0.32 seconds
```
So for 25 fps, any multiple of 8 frames, or 0.32 seconds will do.

For fractional framerates, this may result in large fragment lengths. 30000/1001 for example would result in about 21.35 seconds

### Fragment encoding
The included sample asset is encoded at 60fps, which requires a multiple of 32 frames. With 256 frames, this results in a duration of 4.266667. 

### Audio
Create a looping 48KHz PCM source of 256 * 48000 / 60 = 204800 samples. Divided by 1024 this will be 200 AAC frames.

AAC requires padding, to avoid unwanted padding in the fragment, create an intermediary file with the source looped 2 times:
```sh
$ ffmpeg -stream_loop 2 -i loop.wav -c:a aac -b:a 128k intermediary.m4a
```
Create the required segment and init file from a slice of the intermediary file. We need a start seek time that aligns with the 1024 sample frame size, and round the duration down a little:
```sh
$ ffmpeg -ss 0.32 -i intermediary.m4a -c:a copy -t 4.26 -start_at_zero -f dash fragment.mpd
```
This should result in a fragment containing 200 AAC frames, each frame 1024 samples, which can be verified with:
```sh
$ ffprobe -show_frames fragment.mpd | grep 'nb_samples=1024' | sort | uniq -c
```
In this case the output should be `200 nb_samples=1024` and no other sample count.

### Video
The video is a 256 frame looping animation, 2-pass encoded to an h264 fragment:
```sh
$ ffmpeg -i 0001-0256.avi -c:v libx264 -x264-params keyint=256:min-keyint=256:scenecut=0:bframes=2:b-adapt=0:ref=2:b-adapt=0:open-gop=0:nal-hrd=cbr -b:v 8M -minrate 8M -maxrate 8M -bufsize 16M -pix_fmt yuv420p -preset veryslow -pass 1 -f null /dev/null
$ ffmpeg -i 0001-0256.avi -c:v libx264 -x264-params keyint=256:min-keyint=256:scenecut=0:bframes=2:b-adapt=0:ref=2:b-adapt=0:open-gop=0:nal-hrd=cbr -b:v 8M -minrate 8M -maxrate 8M -bufsize 16M -pix_fmt yuv420p -preset veryslow -pass 2 -f dash fragment.mpd
```
Only the chunk and init files are required and embedded during build, the generated mpd's can be discarded.

## License
Distributed under the MIT license. See LICENCE for more information.