mod tests {
    use super::*;
    #[test]
    fn test_filter_by_game_name(){
        let games = vec![
            GameData {
                date: "2023-08-01".to_string(),
                active_users: 80000,
                title: "PetSimulatorX".to_string()
            }
            GameData {
                date: "2024-12-01".to_string(),
                active_users: 75000,
                title: "PetSimulatorX".to_string()
            }
            GameData {
                date: "2017-01-01",
                active_users: 1200,
                title: "Brookhaven".to_string()
            }
            GameData {
                date: "2018-05-27",
                active_users: 90000,
                title: "Brookhaven".to_string()
            }
        ];
        let filtered_games_by_name = games.filter_by_game_name("Brookhaven")
        assert_eq!(filtered_games_by_name[0].title, "Brookhaven")
    }
}