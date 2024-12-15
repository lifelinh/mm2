use csv::ReaderBuilder;
use chrono::NaiveDate;
use chrono::Datelike;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
#[derive(Debug)]
struct GameData {
    date: String,
    active_users: i64,
    title: String,
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
    fn format_date(&mut self) {
        if let Ok(parsed_date) = NaiveDate::parse_from_str(&self.date, "%m/%d/%Y") {
            self.date = parsed_date.format("%Y-%m-%d").to_string(); // Convert to yyyy-mm-dd
        }
    }
    fn hourly_average_users(games: Vec<GameData>) -> Vec<(String, f64)> {
        let mut combined: HashMap<String, (i64, i64)> = HashMap::new(); // (sum, count)
        for game in games {
            let entry = combined.entry(game.date.clone()).or_insert((0, 0));
            entry.0 += game.active_users;
            entry.1 += 1;
        }
        combined.into_iter().map(|(date, (sum, count))| {
            let average = sum as f64 / count as f64;
            (date, average)
        }).collect()
    }
    fn day_of_week(date: &str) -> String {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Date is not in yyyy-mm-dd format");
        match date.weekday() {
            chrono::Weekday::Mon => "Monday".to_string(),
            chrono::Weekday::Tue => "Tuesday".to_string(),
            chrono::Weekday::Wed => "Wednesday".to_string(),
            chrono::Weekday::Thu => "Thursday".to_string(),
            chrono::Weekday::Fri => "Friday".to_string(),
            chrono::Weekday::Sat => "Saturday".to_string(),
            chrono::Weekday::Sun => "Sunday".to_string(),
        }
    }
    fn average_by_day_of_week(games: Vec<(String, f64)>) -> HashMap<String, f64> {
        let mut day_of_week_sums: HashMap<String, (f64, i64)> = HashMap::new();
        for (date, average_active_users) in games {
            let day_of_week = GameData::day_of_week(&date);
            let entry = day_of_week_sums.entry(day_of_week).or_insert((0.0, 0));
            entry.0 += average_active_users;
            entry.1 += 1
        }
        day_of_week_sums.into_iter().map(|(day, (sum, count))| {
            let average = sum / count as f64;
            (day, average)
        }).collect()
    }
    fn average_by_month(games: Vec<(String, f64)>) -> Vec<(String, f64)> {
        let mut month_sums: HashMap<String, (f64, i64)> = HashMap::new();
        for (date, avg_active_users) in games {
            let month = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Invalid date format").format("%Y-%m").to_string();
            
            let entry = month_sums.entry(month).or_insert((0.0, 0));
            entry.0 += avg_active_users;
            entry.1 += 1;
        }
        let mut month_avg: Vec<(String, f64)> = month_sums.into_iter().map(|(month, (sum, count))| {
            let average = sum / count as f64;
            (month, average)
        }).collect();
        month_avg.sort_by(|a, b| a.0.cmp(&b.0));
        month_avg
    }
}
fn read_filtered_games(file_path: &str) -> Result<Vec<GameData>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().delimiter(b',').has_headers(true).from_reader(file);
    let headers = rdr.headers()?;
    let date = headers.iter().position(|h| h == "Date").ok_or("Column 'Date' not found")?;
    let active_users = headers.iter().position(|h| h == "Active Users").ok_or("Column 'Active Users' not found")?;
    let title = headers.iter().position(|h| h == "Title").ok_or("Column 'Title' not found")?;
    let mut games_data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut game = GameData {
            date: record.get(date).ok_or("Missing 'Date' value")?.to_string(),
            active_users: record.get(active_users).ok_or("Missing 'Active Users' value")?.parse::<i64>()?,
            title: record.get(title).ok_or("Missing 'Title' value")?.to_string(),
        };
        game.date_only();
        game.format_date();
        games_data.push(game);
    }
    Ok(games_data)
}
fn linear_regression(data: Vec<(String, f64)>) -> (f64, f64) {
    let n = data.len() as f64;
    let sum_x: f64 = (0..n as usize).map(|x| x as f64).sum();
    let sum_y: f64 = data.iter().map(|(_, y)| *y).sum();
    let sum_x_squared: f64 = (0..n as usize).map(|x| x as f64 * x as f64).sum();
    let sum_xy: f64 = data.iter().enumerate().map(|(x, (_, y))| x as f64 * y).sum();
    let m = (n * sum_xy - sum_x * sum_y) / (n * sum_x_squared - sum_x * sum_x);
    let b = (sum_y - m * sum_x) / n;
    
    (m, b)
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "roblox_games_data.csv";
    let filtered_games = read_filtered_games(file_path)?;
    let game_name = "MurderMystery2By@Nikilis";
    let filtered_games_by_name = GameData::filter_by_game_name(filtered_games, game_name);
    let mut daily_average = GameData::hourly_average_users(filtered_games_by_name);
    daily_average.sort_by(| a, b| a.0.cmp(&b.0));
    for (date, active_users) in &daily_average {
        println!("Date: {}, Daily average active Users: {}", date, active_users);
    }
    let day_of_week_averages = GameData::average_by_day_of_week(daily_average.clone());
    for (day, average_users) in day_of_week_averages {
        println!("{}: {:.2}", day, average_users);
    }
    let monthly_averages = GameData::average_by_month(daily_average.clone());
    for (month, average_users) in monthly_averages {
        println!("{}: {:.2}", month, average_users);
    }
    let (slope, intercept) = linear_regression(daily_average.clone());
    println!("Linear Regression: y = {:.2}x + {:.2}", slope, intercept);
    Ok(())
}