use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
#[derive(Debug)]
struct GameData {
    date: String,
    active_users: i64,
    title: String,
    creator: String,
}
impl GameData {
    fn filter_by_game_name(games: Vec<GameData>, game_name: &str) -> Vec<GameData> {
        games.into_iter().filter(|game| game.title.to_lowercase() == game_name.to_lowercase()).collect()
    }
    fn date_only(&mut self) {
        if let Some(space) = self.date.find(' ') {
            self.date = self.date[..space].to_string()
        }
    }
}
fn read_filtered_games(file_path: &str) -> Result<Vec<GameData>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().delimiter(b',').has_headers(true).from_reader(file);
    let headers = rdr.headers()?;
    let date = headers.iter().position(|h| h == "Date").ok_or("Column 'Date' not found")?;
    let active_users = headers.iter().position(|h| h == "Active Users").ok_or("Column 'Active Users' not found")?;
    let title = headers.iter().position(|h| h == "Title").ok_or("Column 'Title' not found")?;
    let creator = headers.iter().position(|h| h == "Creator").ok_or("Column 'Creator' not found")?;
    let mut games_data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut game = GameData {
            date: record.get(date).ok_or("Missing 'Date' value")?.to_string(),
            active_users: record.get(active_users).ok_or("Missing 'Active Users' value")?.parse::<i64>()?,
            title: record.get(title).ok_or("Missing 'Title' value")?.to_string(),
            creator: record.get(creator).ok_or("Missing 'Creator' value")?.to_string(),
        };
        game.date_only();
        games_data.push(game);
    }
    Ok(games_data)
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "roblox_games_data.csv";
    let filtered_games = read_filtered_games(file_path)?;
    let game_name = "MurderMystery2By@Nikilis";
    let filtered_games_by_name = GameData::filter_by_game_name(filtered_games, game_name);
    for game in filtered_games_by_name {
        println!("{:?}", game);
    }
    Ok(())
}
