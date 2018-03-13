extern crate curl;
extern crate serde;
#[macro_use] extern crate chrono;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::io::{stdout, Write, Read};
use std::iter::repeat;
use std::time::Instant;
use curl::easy::{Easy2, Handler, WriteError};
use chrono::prelude::*;
use chrono::Date;

use serde_json::{Value, Error};

fn main() {
    println!("Hello, world!");


    let mut curl = Easy2::new(Collector(Vec::new()));
    curl.get(true).unwrap();
	curl.useragent("Chrome/41.0.2227.0").unwrap();
    curl.url("https://statsapi.web.nhl.com/api/v1/schedule?startDate=2017-12-12&endDate=2017-12-12").unwrap();
    curl.perform().unwrap();

    let web = curl.get_ref();

    let json = String::from_utf8(web.0.as_slice().to_vec()).unwrap();

	println!("{}", json);

    let data: Api = serde_json::from_str(&json).unwrap();
    //let v: Value = serde_json::from_str(&json).unwrap();

    println!("{}", data.copyright);

/*
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut web.0.as_slice())
        .unwrap();


    walk(0, dom.document);
*/
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
struct ApiStatus {
    abstractGameState: String,
    codedGameState: String,
    detailedState: String,
    statusCode: String,
    startTimeTBD: bool,
}

#[derive(Serialize, Deserialize)]
struct ApiTeams {
    away: ApiTeamResult,
    home: ApiTeamResult,
}

#[derive(Serialize, Deserialize)]
struct ApiTeamResult {
    leagueRecord: ApiRecord,
    score: usize,
    team: ApiTeam,
}

#[derive(Serialize, Deserialize)]
struct ApiTeam {
    id: usize,
    name: String,
    link: String,
}

#[derive(Serialize, Deserialize)]
struct ApiRecord {
    wins: usize,
    losses: usize,
    ot: usize,
    #[serde(rename="type")]
    _type: String,
}

#[derive(Serialize, Deserialize)]
struct ApiVenue {
    name: String,
    link: String,
}

#[derive(Serialize, Deserialize)]
struct ApiContent {
    link: String,
}
