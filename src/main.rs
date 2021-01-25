use serde_json::Result;

fn main() -> Result<()> {
    let player_data_path = "C:\\Users\\Auros\\AppData\\LocalLow\\Hyperbolic Magnetism\\Beat Saber\\PlayerData.dat";
    eprintln!("Deserializing player data...");

    let player_data_json = std::fs::read_to_string(player_data_path).expect("Could not read player data file.");
    let player_data: PlayerData = serde_json::from_str(&player_data_json)?;
    
    let players: Vec<LocalPlayer> = player_data.local_players;
    let size = players.len();

    if size == 0 {
        eprintln!("No local players detected.");
        std::process::exit(1);
    }

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