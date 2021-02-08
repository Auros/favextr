use serde_json::Result;
use chrono::prelude::*;

fn main() -> Result<()> {
    let platform_dirs = platform_dirs::AppDirs::new(Some(""), false).unwrap();
    let player_data_path= platform_dirs.cache_dir.to_str().unwrap().to_owned().trim_end_matches("\\").to_owned() + "Low\\Hyperbolic Magnetism\\Beat Saber\\PlayerData.dat";
    println!("{}", player_data_path);
    eprintln!("Deserializing player data...");

    let player_data_json = std::fs::read_to_string(player_data_path).expect("Could not read player data file.");
    let player_data: PlayerData = serde_json::from_str(&player_data_json)?;
    
    let players: Vec<LocalPlayer> = player_data.local_players;
    if players.is_empty() {
        eprintln!("No local players detected.");
        std::process::exit(1);
    }

    let date = Local::now().date().to_string();
    let player = players.get(0).expect("msg").to_owned();
    let title = format!("{} {}", "Favorites", date);
    let mut playlist = Playlist {
        songs: Vec::new(),
        playlist_title: title,
        playlist_author: String::new(),
        playlist_description: String::new()
    };
    for id in player.favorites_level_ids {
        playlist.songs.push(Song { hash: id.replace("custom_level_", "").to_lowercase().to_owned() })
    }

    let serial = serde_json::to_string(&playlist)?;
    let save_path = "FavoritesPlaylists.bplist";
    std::fs::write(save_path, serial).expect("Unable to write file");

    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPlayer {
    pub player_id: String,
    pub player_name: String,
    pub favorites_level_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    pub version: String,
    pub local_players: Vec<LocalPlayer>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    songs: Vec<Song>,
    playlist_title: String,
    playlist_author: String,
    playlist_description: String
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    hash: String
}