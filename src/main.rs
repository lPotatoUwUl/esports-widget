#![allow(unused)]
use reqwest;
use scraper::{Html, Selector, element_ref::Select};
use serde::{Serialize};
use std::{error::Error, fs::File, io::repeat};

#[derive(Debug, Serialize)]
struct GameCard {
    team_names: String,
    match_time: String,
}

#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {

    // Fetching and parsing HTML
    let url = "https://www.vlr.gg/matches";
    let mut gcards: Vec<GameCard> = Vec::new();
    let response = reqwest::get(url).await?; 
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    // Selecting CSS 
    let game_selector = Selector::parse(".match-item-vs-team").unwrap();
    let name_selector = Selector::parse(".match-item-vs-team-name").unwrap();
    let time_selector = Selector::parse(".match-item-time").unwrap();
    
    // Collect Scraped Data
     for gcard in document.select(&game_selector) {

        // Only Printing One Team Name at a time
        let team_names = gcard
        .select(&name_selector)
        .next()
        .map(|t| t.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default()
        .replace("\t", "")
        .replace("\n", "");
        
        // Currently Not Printing FIX
        let match_time = gcard
        .select(&time_selector)
        .next()
        .map(|t| t.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default();
       
        gcards.push(GameCard { team_names, match_time });
        
}
    let x = gcards; 
    println!("{:?}", x );

    Ok(())

}
