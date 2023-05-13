use std::{fs, io, thread};
use std::error::Error;
use std::fmt::format;
use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::time::Duration;

use rodio::{Decoder, Sink, Source};
use rodio::source::SineWave;
use rustube::Video;
use url::Url;

fn get_url_iter() -> Lines<BufReader<File>> {
    const FILENAME: &str = "src/playlist.txt";
    let file = File::open(FILENAME).unwrap();
    BufReader::new(file).lines()
}

pub fn return_url_iter() -> Vec<Url> {
    let mut urls: Vec<Url> = vec![];
    for line in get_url_iter() {
        let url = Url::parse(line.unwrap().as_str().trim()).unwrap();
        urls.push(url);
    }
    urls
}

//downloads mp3 file using process builder yt-dlp
pub async fn download_mp3(urls: Vec<Url>) {
    const YT_DLP_PATH: &str = "/home/cavej/miniconda3/bin/yt-dlp";
    let mut filenames = vec![];
    let mut count = 0;


    for url in &urls {
        let mut child = std::process::Command::new(YT_DLP_PATH)
            .arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg("-f")
            .arg(".songs/bestaudio")
            .arg("-o")
            .arg(format!(".songs/{}.%(ext)s", count))
            .arg(url.as_str())
            .spawn().unwrap();
        child.wait().expect("failed to wait on child");

        filenames.push(format!(".songs/{}.mp3", count));
        count += 1;
    }
}

pub fn get_url_file_vector() -> Vec<String> {
    let mut count = 0;
    let mut vec_url = vec![];
    for line in return_url_iter() {
        vec_url.push(format!(".songs/{}.mp3", count));
        count += 1;
    }
    println!("VEC: {:?}", vec_url);
    vec_url
}

//play mp3 files using rodio
pub async fn play_audio_from_mp3(vector: Vec<String>) {
    println!("VEC: {:?}", vector);

    // Run the audio-playing logic in a separate function inside spawn_blocking
    tokio::task::spawn_blocking(move || {
        play_audio_blocking(vector);
    })
        .await
        .unwrap();
}

fn play_audio_blocking(vector: Vec<String>) {
    thread::sleep(Duration::from_secs(5));
    println!("PLAYING AUDIO");

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    for file in vector.iter() {
        let file = File::open(file).unwrap();
        println!("FILE: {:?}", file);
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}