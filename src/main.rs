use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

struct Format {
  id: String,
  size: f64,
  ext: String,
}

fn execute(command: String) {
  if cfg!(target_os = "windows") {
    let mut handle = Command::new("cmd")
      .arg("/C")
      .arg(&command)
      .spawn()
      .expect(format!("failed on '{}'", command).as_str());
    handle.wait().expect("failed to wait");
  } else {
    let mut handle = Command::new("sh")
      .arg("-c")
      .arg(&command)
      .spawn()
      .expect(format!("failed on '{}'", command).as_str());
    handle.wait().expect("failed to wait");
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("youtube url is required as an argument");
  }
  let url: &str;
  match args[1].find('&') {
    None => {
      url = &args[1];
    },
    Some(offset) => {
      url = &args[1][0..offset];
    }
  }
  println!("Url: {}", url);
  let output = YoutubeDl::new(url)
    .socket_timeout("15")
    .run()
    .unwrap();
  //println!("{:#?}", output);
  let mut video_id = "".to_string();
  let mut video_title = "".to_string();
  let mut best_video = Format {
    id: "0".to_string(),
    size: 0.0,
    ext: "".to_string()
  };
  let mut best_audio = HashMap::<String, Format>::new();

  // parse
  match output {
    YoutubeDlOutput::Playlist(_) => println!("playlist not supported"),
    YoutubeDlOutput::SingleVideo(v) => {
      //println!("singlevideo: {:#?}", v);
      video_id = v.display_id.unwrap();
      println!("Id: {:?}", video_id.clone());
      println!("Title: {}", v.title);
      video_title = v.title.clone();
      //println!("formats: {:#?}", v.formats);
      println!("Formats:");
      // iterate through available formats
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
        print!("  {} {} {}k", format_id, ext, tbr);
        if has_video {
          let width = f.width.unwrap();
          let height = f.height.unwrap();
          print!(" - {} x {}", width, height);
          if tbr > best_video.size {
            best_video.size = tbr;
            best_video.id = format_id.clone();
            best_video.ext = ext.clone();
          }
        }
        if has_audio {
          if !has_video &&
            (!best_audio.contains_key(&ext) || tbr > best_audio[&ext].size) {
            best_audio.insert(
              ext.clone(),
              Format{
                id: format_id.clone(),
                size: tbr,
                ext: ext.clone()
              }
            );
          }
        }
        println!();
      }
    }
  }
  println!();

  // select best formats
  let audio_ext = match best_video.ext.as_str() {
    "mp4" => "m4a".to_string(),
    "webm" => "webm".to_string(),
    _ => "".to_string(),
  };

  // file names
  let video_output = format!(
    "{}_{}.{}",
    video_id,
    best_video.id,
    best_video.ext
  )
  .replace("/", "-")
  .trim()
  .to_string();
  let audio_output = format!(
    "{}_{}.{}",
    video_id,
    best_audio[&audio_ext].id,
    best_audio[&audio_ext].ext
  )
  .replace("/", "-")
  .trim()
  .to_string();

  let options = sanitize_filename::Options {
    truncate: true,
    windows: true,
    replacement: ""
  };

  let final_output = sanitize_filename::sanitize_with_options(
    format!("{}.{}", video_title, best_video.ext)
      .replace("/", "-")
      .trim()
      .to_string(),
    options
  );

  // display chosen formats
  println!("Selection:");
  println!(
    "  video@{} .{}: {}k",
    best_video.id,
    best_video.ext,
    best_video.size
  );
  println!(
    "  audio@{} .{}: {}k",
    best_audio[&audio_ext].id,
    best_audio[&audio_ext].ext,
    best_audio[&audio_ext].size
  );
  println!();

  // download video part
  execute(format!(
    "youtube-dl -f {} --continue -o \"{}\" \"{}\"",
    best_video.id,
    video_output,
    url
  ));

  // download audio part
  execute(format!(
    "youtube-dl -f {} --continue -o \"{}\" \"{}\"",
    best_audio[&audio_ext].id,
    audio_output,
    url
  ));

  // combine video and audio parts
  execute(format!(
    "ffmpeg -i \"{}\" -i \"{}\" -c:v copy -c:a copy \"{}\" < /dev/null",
    video_output,
    audio_output,
    final_output
  ));

  // unlink video and audio parts
  let cwd = match env::current_dir() {
    Ok(dir) => dir,
    Err(_) => { panic!(); }
  };
  //println!("cwd: {:?}", Path::new(cwd.to_str().unwrap()).join("somefile"));
  let video_path = Path::new(cwd.to_str().unwrap())
    .join(video_output.clone());
  print!("deleting {:?}... ", video_output.clone());
  match fs::remove_file(video_path) {
    Ok(_) => { println!("deleted"); },
    Err(e) => { println!("failed: {}", e); }
  }
  let audio_path = Path::new(cwd.to_str().unwrap())
    .join(audio_output.clone());
  print!("deleting {:?} ...", audio_output.clone());
  match fs::remove_file(audio_path) {
    Ok(_) => { println!("deleted"); },
    Err(e) => { println!("failed: {}", e); }
  }
}
