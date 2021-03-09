use youtube_dl::YoutubeDl;
use youtube_dl::YoutubeDlOutput;
use std::collections::HashMap;
use std::env;
//use std::io::{self, Write};
//use std::process::Command;

struct Format {
  id: String,
  size: f64,
  ext: String,
}

fn main() {
  let args: Vec<String> = env::args().collect();
  //let output = YoutubeDl::new("https://www.youtube.com/watch?v=ZFihgCtk_aw")
  let url = &args[1];
  let output = YoutubeDl::new(url)
    .socket_timeout("15")
    .run()
    .unwrap();
  //println!("{:#?}", output);
  let mut video_title = "".to_string();
  let mut best_video = Format {id: "0".to_string(), size: 0.0, ext: "".to_string()};
  let mut best_audio = HashMap::<String, Format>::new();

  match output {
    YoutubeDlOutput::Playlist(_) => println!("playlist not supported"),
    YoutubeDlOutput::SingleVideo(v) => {
      //println!("singlevideo: {:#?}", v)
      //println!("title: {}", v.title);
      video_title = v.title.clone();
      //println!("formats: {:#?}", v.formats);
      for f in v.formats.unwrap() {
        //println!("{:#?}", f);
        let format_id = f.format_id.unwrap();
        let has_video = match f.vcodec {
          Some(t) => (t != "none"),
          None => false
        };
        let has_audio = match f.acodec {
          Some(t) => (t != "none"),
          None => false
        };
        let ext = f.ext.unwrap();
        let tbr = f.tbr.unwrap();
        //print!("{} {} {}k", format_id, ext, tbr);
        if has_video {
          //let width = f.width.unwrap();
          //let height = f.height.unwrap();
          //print!(" - {} x {}", width, height);
          if tbr > best_video.size {
            best_video.size = tbr;
            best_video.id = format_id.clone();
            best_video.ext = ext.clone();
          }
        }
        if has_audio {
          //let abr = f.abr.unwrap();
          //print!(" - {}kbps", abr);
          if !has_video && (!best_audio.contains_key(&ext) || tbr > best_audio[&ext].size) {
            best_audio.insert(ext.clone(), Format{id: format_id.clone(), size: tbr, ext: ext.clone()});
          }
        }
        println!();
      }
    }
  }

  let audio_ext = match best_video.ext.as_str() {
    "mp4" => "m4a".to_string(),
    "webm" => "webm".to_string(),
    _ => "".to_string(),
  };
  //println!("best video format: {} ({})", best_video.id, best_video.ext);
  //println!("best audio format: {} ({})", best_audio[&audio_ext].id, best_audio[&audio_ext].ext);
  println!("#!/bin/sh");
  println!("# video#{} ({}) - {}k", best_video.id, best_video.ext, best_video.size);
  println!("# audio#{} ({}) - {}k", best_audio[&audio_ext].id, best_audio[&audio_ext].ext, best_audio[&audio_ext].size);
  let video_output = format!("{}_{}.{}", video_title, best_video.id, best_video.ext).replace("(", "\\(").replace(")", "\\)");
  let audio_output = format!("{}_{}.{}", video_title, best_audio[&audio_ext].id, best_audio[&audio_ext].ext).replace("(", "\\(").replace(")", "\\)");
  let final_output = format!("{}.{}", video_title, best_video.ext).replace("(", "\\(").replace(")", "\\)");
  println!("youtube-dl -f {} --continue -o \"{}\" \"{}\"", best_video.id, video_output, url);
  println!("youtube-dl -f {} --continue -o \"{}\" \"{}\"", best_audio[&audio_ext].id, audio_output, url);
  println!("ffmpeg -i \"{}\" -i \"{}\" -c:v copy -c:a copy \"{}\"", video_output, audio_output, final_output);
  println!("rm \"{}\" \"{}\"", video_output, audio_output);
  println!();
  //if cfg!(target_os = "windows") {
    //let shell = Command::new("cmd");
  //} else {
    //let video_log = Command::new("sh")
    //  .arg("-c")
    //  .arg(format!(
    //    "youtube-dl -f {} --continue -o \"{}\" \"{}\"",
    //    best_video.id,
    //    video_output,
    //    url
    //  ))
    //  .output()
    //  .expect("failed to download video");
    //io::stdout().write_all(&video_log.stdout).unwrap();
    //io::stderr().write_all(&video_log.stderr).unwrap();
    //let audio_log = Command::new("sh")
    //  .arg("-c")
    //  .arg(format!(
    //    "youtube-dl -f {} --continue -o \"{}\" \"{}\"",
    //    best_audio[&audio_ext].id,
    //    audio_output,
    //    url
    //  ))
    //  .output()
    //  .expect("failed to download audio");
    //io::stdout().write_all(&audio_log.stdout).unwrap();
    //io::stderr().write_all(&audio_log.stderr).unwrap();
  //}
}
