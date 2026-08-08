// PROBLEMS
// ONLY GETS ONE TEAM NAME  [GameCard { team_names: "Team Secret0", teams: "" } when it should be 

#![allow(unused)]
use reqwest;
use scraper::{Html, Selector, element_ref::Select};
use serde::{Serialize};
use std::{error::Error, fs::File, io::repeat};


#[derive(Debug, Serialize)]

struct GameCard {
    // team_names: String,
    match_time: String,
    teams: String,


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
    let container_selector =Selector::parse(".col.mod-1").unwrap();
    let teams_selector =Selector::parse(".match-item").unwrap();
    // let game_selector = Selector::parse(".match-item-vs").unwrap();
    let name_selector = Selector::parse(".match-item-vs-team").unwrap();
    let time_selector = Selector::parse(".match-item-time").unwrap();
    let date_selector = Selector::parse(".col.mod-1").unwrap();
    
    
    // Collect Scraped Data
     for gcard in document.select(&teams_selector) {

        //This setup to grab multiple elements like both team names 
        let teams = gcard
        .select(&name_selector)
        .map(|t| t.text().collect::<String>().trim().to_string())
        .collect::<Vec<_>>()
        .join(" vs ")
        .replace("\t", "")
        .replace("–", "")
        .replace("\n", "");
            
        //this setup is to grab one element only like match time
        let match_time = gcard
        .select(&time_selector)
        .next()
        .map(|t| t.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default()
        .replace("\t", "")
        .replace("\n", "");
     
        gcards.push(GameCard { teams , match_time});
        // gcards.push(GameCard { teams , team_names, match_time, });
}

    let x = gcards; 
    println!("{:?}", x );

    Ok(())

}
