pub mod lib;

use std::error::Error;
use tokio;
use tokio::join;
use ZazaPotify::return_url_iter;
use crate::lib::{download_mp3, get_url_file_vector, play_audio_from_mp3};

#[tokio::main]
async fn main(){
    let songs = get_url_file_vector();



    let download_handle = tokio::spawn(download_mp3(return_url_iter()));
    let play_handle = tokio::spawn(play_audio_from_mp3(songs));

    join!(download_handle, play_handle);
}




