extern crate curl;
extern crate serde;
extern crate chrono;
extern crate serde_json;
extern crate plotlib;
#[macro_use] extern crate serde_derive;

use curl::easy::{Easy2, Handler, WriteError};
use std::collections::HashMap;

use plotlib::style::Line;

use std::fs::File;
use std::io::Write;


fn main() {
    println!("Hello, world!");


    let mut curl = Easy2::new(Collector(Vec::new()));
    curl.get(true).unwrap();
    curl.url("https://statsapi.web.nhl.com/api/v1/schedule?startDate=2017-10-04&endDate=2018-02-12&filter=gameType,eq,'R'").unwrap();
    curl.perform().unwrap();

    let web = curl.get_ref();
    let json = String::from_utf8(web.0.as_slice().to_vec()).unwrap();

    let data: Api = serde_json::from_str(&json).unwrap();

    let mut teams = HashMap::new();

    for date in data.dates {
        println!("Date: {}", date.date);
        for game in date.games {
            if game.gameType != "R" { continue; }
            println!("    {} at {}", game.teams.away.team.name, game.teams.home.team.name);
            ingest_game(&mut teams, &game, &date.date);
        }
    }

    let mut file = File::create("data.txt").unwrap();
    for (team, record) in teams {
        println!("{}: {}-{}-{}", 
                 team, 
                 record.data.iter().filter(|e| e == &&2).count(), 
                 record.data.iter().filter(|e| e == &&0).count(),
                 record.data.iter().filter(|e| e == &&1).count());
        file.write_all(format!("{}\n", team).as_bytes());
        for score in record.acc {
            file.write_all(format!(",{}", score).as_bytes());
        }
        file.write_all(format!("\n").as_bytes());
    }

}

struct TeamRecord {
    name: String,
    data: Vec<usize>,
    acc: Vec<usize>,
    last_wins: usize,
    last_losses: usize,
}

fn ingest_game(teams: &mut HashMap<String, TeamRecord>, game: &ApiGame, date: &String) {
    let away_team = &game.teams.away.team.name.to_string();
    let home_team = &game.teams.home.team.name;

    if !teams.contains_key(away_team) { 
        teams.insert(away_team.to_string(), 
                     TeamRecord { 
                         name: away_team.to_string(), 
                         data: vec![],
                         acc: vec![],
                         last_wins: 0,
                         last_losses: 0,
                     });
    }
    if !teams.contains_key(home_team) { 
        teams.insert(home_team.to_string(), 
                     TeamRecord { 
                         name: home_team.to_string(), 
                         data: vec![],
                         acc: vec![],
                         last_wins: 0,
                         last_losses: 0,
                     });
    }

    update_team(teams, &game.teams.away, date);
    update_team(teams, &game.teams.home, date);
}

fn update_team(teams: &mut HashMap<String, TeamRecord>, team: &ApiTeamResult, date: &String) {
    let team_record = teams.get_mut(&team.team.name).unwrap();
    let score;

    if team_record.last_wins < team.leagueRecord.wins {
        score = 2;
    } else if team_record.last_losses < team.leagueRecord.losses {
        score = 0;
    } else {
        score = 1;
    }

    let acc = team_record.acc.last().unwrap_or(&0) + score;
    team_record.data.push(score);
    team_record.acc.push(acc);

    team_record.last_wins = team.leagueRecord.wins;
    team_record.last_losses = team.leagueRecord.losses;
}


struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

// Define the api datastructure
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Api {
    copyright: String,
    totalItems: usize,
    totalEvents: usize,
    totalGames: usize,
    totalMatches: usize,
    wait: usize,
    dates: Vec<ApiDate>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiDate {
    date: String,
    totalItems: usize,
    totalEvents: usize,
    totalGames: usize,
    totalMatches: usize,
    games: Vec<ApiGame>,
    events: Vec<()>,
    matches: Vec<()>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiGame {
    gamePk: usize,
    link: String,
    gameType: String,
    season: String,
    gameDate: String,
    status: ApiStatus,
    teams: ApiTeams,
    venue: ApiVenue,
    content: ApiContent,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiStatus {
    abstractGameState: String,
    codedGameState: String,
    detailedState: String,
    statusCode: String,
    startTimeTBD: bool,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiTeams {
    away: ApiTeamResult,
    home: ApiTeamResult,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiTeamResult {
    leagueRecord: ApiRecord,
    score: usize,
    team: ApiTeam,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiTeam {
    id: usize,
    name: String,
    link: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiRecord {
    wins: usize,
    losses: usize,
    ot: Option<usize>,
    #[serde(rename="type")]
    _type: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiVenue {
    name: String,
    link: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiContent {
    link: String,
}
