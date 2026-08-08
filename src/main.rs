// PROBLEMS
// ONLY GETS ONE TEAM NAME  [GameCard { team_names: "Team Secret0", teams: "" } when it should be 

// TODO ADD DATE MATCH GROUPING

#![allow(unused)]
use reqwest;
use scraper::{Html, Selector, element_ref::Select};
use serde::{Serialize};
use std::{error::Error, fs::File, io::repeat};


// struct DateGroup {
    
//     match_date: String,
// }


#[derive(Debug, Serialize)]
pub struct Match {
    pub team_a: String,
    pub team_b: String,
    pub match_time: String,     
    pub status: MatchStatus,    
    pub event_name: String,     
    pub match_url: String,      
}

pub enum MatchStatus {
    Live,
    Upcoming,
    Completed,
}

pub struct DateGroup {
    pub date: String,           // e.g., "Sat, August 8, 2026"
    pub matches: Vec<Match>,    // Vec<Match> holds the collection
}




// async fn main () -> Result<(), Box<dyn Error>> {

//     // Fetching and parsing HTML
//     let url = "https://www.vlr.gg/matches";
//     let mut gcards: Vec<GameCard> = Vec::new();
//     let mut dates: Vec<DateGroup> = Vec::new();
//     let response = reqwest::get(url).await?; 
//     let html = response.text().await?;
//     let document = Html::parse_document(&html);

//     // Selecting CSS 
//     let container_selector =Selector::parse(".wf-label").unwrap();
//     let teams_selector =Selector::parse(".match-item").unwrap();
//     // let game_selector = Selector::parse(".match-item-vs").unwrap();
//     let name_selector = Selector::parse(".match-item-vs-team").unwrap();
//     let time_selector = Selector::parse(".match-item-time").unwrap();
//     let date_selector = Selector::parse(".wf-label.mod-large").unwrap();

      
    
//     // Collect Scraped Data
//      for gcard in document.select(&teams_selector) {

//         //This setup to grab multiple elements like both team names 
//         let teams = gcard
//         .select(&name_selector)
//         .map(|t| t.text().collect::<String>().trim().to_string())
//         .collect::<Vec<_>>()
//         .join(" vs ")
//         .replace("\t", "")
//         .replace("–", "")
//         .replace("\n", "");
            
//         //this setup is to grab one element only like match time
//         let match_time = gcard
//         .select(&time_selector)
//         .next()
//         .map(|t| t.text().collect::<Vec<_>>().join(""))
//         .unwrap_or_default()
//         .replace("\t", "")
//         .replace("\n", "");

    
     
//         gcards.push(GameCard { teams , match_time,});
        
//         // gcards.push(GameCard { teams , team_names, match_time, });
// }

// for dateg in document.select(&container_selector) {
//         let match_date = dateg
//         .select(&date_selector)
//         .next()
//         .map(|t| t.text().collect::<Vec<_>>().join(""))
//         .unwrap_or_default();
       
//         dates.push(DateGroup {match_date});
     
//      }  

    
//     let s = dates;
//     // let x = gcards; 
//     println!("{:?},", s , );

//     Ok(())

// }

#[tokio::main]

async fn main () -> Result<(), Box<dyn Error>> {
    let url = "https://www.vlr.gg/matches";


}