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

fn read_filtered_games(file_path: &str) -> Result<Vec<GameData>, Box<dyn Error>> {
    let file = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(file);
    let headers = rdr.headers()?;
    let date = headers.iter().position(|h| h == "Date").ok_or("Column 'Date' not found")?;
    let active_users = headers.iter().position(|h| h == "Active Users").ok_or("Column 'Active Users' not found")?;
    let title = headers.iter().position(|h| h == "Title").ok_or("Column 'Title' not found")?;
    let creator = headers.iter().position(|h| h == "Creator").ok_or("Column 'Creator' not found")?;
    let mut games_data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let game = GameData {
            date: record.get(date).ok_or("Missing 'Date' value")?.to_string(),
            active_users: record.get(active_users).ok_or("Missing 'Active Users' value")?.parse::<i64>()?,
            title: record.get(title).ok_or("Missing 'Title' value")?.to_string(),
            creator: record.get(creator).ok_or("Missing 'Creator' value")?.to_string(),
        };
        games_data.push(game);
    }
    Ok(games_data)
}
fn filter_games(mut rdr: csv::Reader<File>, headers: &csv::StringRecord, title_filter: Option<&str>, creator_filter: Option<&str>) -> Result<Vec<GameData>, Box<dyn Error>> {
    let date = headers.iter().position(|h| h == "Date").ok_or("Column 'Date' not found")?;
    let active_users = headers.iter().position(|h| h == "Active Users").ok_or("Column 'Active Users' not found")?;
    let title = headers.iter().position(|h| h == "Title").ok_or("Column 'Title' not found")?;
    let creator = headers.iter().position(|h| h == "Creator").ok_or("Column 'Creator' not found")?;
    let mut filtered_games = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let matches_title = title_filter.map_or(true, |filter| {
        record.get(title).map_or(false, |t| t == filter)
    });
        let matches_creator = creator_filter.map_or(true, |filter| {
        record.get(creator).map_or(false, |c| c == filter)
    });
    if matches_title && matches_creator {
        filtered_games.push(GameData {
            date: record.get(date).ok_or("Missing 'Date' value")?.to_string(),
            active_users: record.get(active_users).ok_or("Missing 'Active Users' value")?.parse::<i64>()?,
            title: record.get(title).ok_or("Missing 'Title' value")?.to_string(),
            creator: record.get(creator).ok_or("Missing 'Creator' value")?.to_string()
            });
        }
    }
    Ok(filtered_games)
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "roblox_games_data.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();
    let filtered_games = filter_games(rdr, &headers, Some("MurderMystery2By@Nikilis"), Some("@Nikilis"),)?;
    println!("Filtered by title and creator: {:?}", filtered_games);
    Ok(())
}