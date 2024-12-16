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
        // Use a closure to only collect entries where the title field matches the input game_name
        games.into_iter().filter(|game| game.title.to_lowercase() == game_name.to_lowercase()).collect()
    }
    fn date_only(&mut self) {
        // Get the index of the space, and basically shorten the entry beyond that space, 'cleaning' the date entry and removing anything else
        if let Some(space) = self.date.find(' ') {
            self.date = self.date[..space].to_string()
        }
    }
    fn format_date(&mut self) {
        // Further date cleaning, using chrono crate to easily reformat all dates to yyyy-mm-dd format, parsing them for processing
        if let Ok(parsed_date) = NaiveDate::parse_from_str(&self.date, "%m/%d/%Y") {
            self.date = parsed_date.format("%Y-%m-%d").to_string();
        }
    }
    fn hourly_average_users(games: Vec<GameData>) -> Vec<(String, f64)> {
        // Initialized an empty HashMap for the output
        let mut combined: HashMap<String, (i64, i64)> = HashMap::new();
        for game in games {
            // Create a new entry for the date if it doesn't exist in combined, and add the active users to the existing key if the date already exists in it
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
        // Use chrono crate to identify the day of the week, parsing the field for the crate to identify it in the match statement
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
            // Creates a key for the day of the week if it doesn't exist, adding the player count if it does
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
            // Identify the month with chrono crate
            let month = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Invalid date format").format("%Y-%m").to_string();
            // Add a new key for the month if it doesn't exist, and adds the playercount to the key if it does
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
    // Identify the index of date, active_users, and title in the CSV
    let date = headers.iter().position(|h| h == "Date").ok_or("Column 'Date' not found")?;
    let active_users = headers.iter().position(|h| h == "Active Users").ok_or("Column 'Active Users' not found")?;
    let title = headers.iter().position(|h| h == "Title").ok_or("Column 'Title' not found")?;
    let mut games_data = Vec::new();
    for result in rdr.records() {
        // Read the values in the CSV into the GameData struct, making a new struct for every record
        let record = result?;
        let mut game = GameData {
            date: record.get(date).ok_or("Missing 'Date' value")?.to_string(),
            active_users: record.get(active_users).ok_or("Missing 'Active Users' value")?.parse::<i64>()?,
            title: record.get(title).ok_or("Missing 'Title' value")?.to_string(),
        };
        // Use implemented methods to clean the date field before pushing it
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
fn predict_until_dead(start_date: &str, slope: f64, intercept: f64) -> (Vec<(String, f64)>, usize) {
    let mut predictions = Vec::new();
    let mut day = 0;
    let mut predicted_active_users = slope * day as f64 + intercept;
    while predicted_active_users > 0.0 && day < 365 {
        // Use chrono to increment the day by one, calculating the predicted active users until the player count will reach 0, or 365 days have passed
        let date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").expect("Invalid date format") + chrono::Duration::days(day as i64);
        let date_str = date.format("%Y-%m-%d").to_string();
        predictions.push((date_str, predicted_active_users));
        day += 1;
        predicted_active_users = slope * day as f64 + intercept;
    }
    let days_until_stopped = day;
    (predictions, days_until_stopped)
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "roblox_games_data.csv";
    let filtered_games = read_filtered_games(file_path)?;
    let game_name = "MurderMystery2By@Nikilis"; // Change for any game name to analyze
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
    let (predictions, days_until_zero) = predict_until_dead("2022-05-03", slope, intercept);
    for (date, predicted_active_users) in predictions {
        println!("Date: {}, Predicted daily average active users: {:.2}", date, predicted_active_users);
    }
    println!("It may take {} days for the game to die.", days_until_zero);
    Ok(())
}
#[test]
fn test_filter_by_game_name() {
    let games = vec![
        GameData {
            date: "2023-08-01".to_string(),
            active_users: 80000,
            title: "PetSimulatorX".to_string()
        },
        GameData {
            date: "2024-12-01 15:00:23".to_string(),
            active_users: 75000,
            title: "PetSimulatorX".to_string()
        },
        GameData {
            date: "2017-01-01".to_string(),
            active_users: 1200,
            title: "Brookhaven".to_string()
        },
        GameData {
            date: "2018-05-27 19:40:27".to_string(),
            active_users: 90000,
            title: "Brookhaven".to_string()
        }
    ];
    let filtered_games_by_name = GameData::filter_by_game_name(games, "Brookhaven");
    assert_eq!(filtered_games_by_name[0].title, "Brookhaven");
    assert_eq!(filtered_games_by_name[0].title, filtered_games_by_name[1].title);
    assert_eq!(filtered_games_by_name.len(), 2);
}
#[test]
fn test_date_cleaning() {
    let mut game = GameData { 
        date: "01/01/2023 12:00:00".to_string(), 
        active_users: 35000, 
        title: "MurderMystery2By@Nikilis".to_string() 
    };
    game.date_only();
    game.format_date();
    assert_eq!(game.date, "2023-01-01");
}
#[test]
fn test_hourly_active_users() {
    let games = vec![
        GameData {
            date: "2023-01-01".to_string(),
            active_users: 10000,
            title: "BloxFruits".to_string()
        },
        GameData {
            date: "2023-01-01".to_string(),
            active_users: 20000,
            title: "BloxFruits".to_string()
        }];
        let averages = GameData::hourly_average_users(games);
        assert_eq!(averages.len(), 1);
        assert_eq!(averages[0].0, "2023-01-01");
        assert_eq!(averages[0].1, 15000.0)
}
#[test]
fn test_chrono_day_of_week() {
    let day = GameData::day_of_week("2024-12-14");
    assert_eq!(day, "Saturday")
}
#[test]
fn test_linear_regression() {
    let data = vec![
        ("2023-01-01".to_string(), 100.0),
        ("2023-01-02".to_string(), 80.0),
        ("2023-01-03".to_string(), 60.0),
    ];
    let (slope, intercept) = linear_regression(data);
    assert_eq!(slope, -20.0);
    assert_eq!(intercept, 100.0);
    let (predictions, days_until_dead) = predict_until_dead("2023-01-01", slope, intercept);
    assert_eq!(days_until_dead, 5);
    assert_eq!(predictions.last().unwrap().0, "2023-01-05");
    assert_eq!(predictions.last().unwrap().1, (0.0 + slope).abs())
}