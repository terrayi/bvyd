# bvyd (Best Video Youtube Downloader)

 This is a simple wrapper around youtube-dl and ffmpeg to store youtube video of the best available quality.

 Instead of manually running a multiple number of commands below each time when I got to download a video from Youtube, this small utility will do it for me with only one command.

```
youtube-dl -F "url"
youtube-dl -f "videoId" "url"
youtube-dl -f "audioId" "url"
ffmpeg -i "videoPath" -i "audioPath" -c:v copy -c:a copy "finalFilePath"
```

 This is also an exercise to learn Rust myself. :) This only has one depended crate: youtube-dl * surprise! * Otherwise, you need to have youtube-dl and ffmpeg in the PATH.

##### FYI
I am horrible at naming clearly. I didn't bother too much :p
