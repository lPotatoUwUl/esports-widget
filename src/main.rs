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

    let name_selector = Selector::parse(".text-of").unwrap();
    let time_selector = Selector::parse(".match-item-time").unwrap();
    
    // Collect Scraped Data


     for gcard in document.select(&name_selector) {
        let team_names = gcard
        .select(&name_selector)
        .next()
        .map(|t| t.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default();

        let match_time = gcard
        .select(&time_selector)
        .next()
        .map(|t| t.text().collect::<Vec<_>>().join(""))
        .unwrap_or_default();
        
        // match_time.to_string();

        gcards.push(GameCard { team_names, match_time });
        

}

    
   

    // REDO THE SELECTORS ITS NOT STRUCTING / PRINTING RIGHT

    let x = gcards; 
    // println!("{:?}", x );

    Ok(())

}
