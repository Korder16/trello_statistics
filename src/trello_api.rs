use std::fmt;
use std::env;

use serde::Deserialize;
use reqwest::header::ACCEPT;
use colored::Colorize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badges {
    check_items: u16,
    check_items_checked: u16,
}

fn parse_percent(percent: f32) -> String {
    let mut percent_str= format!("{:.0}%", percent);
    if percent >= 0.0 && percent < 30.0 {
        percent_str = percent_str.black().to_string();
    } else if percent >= 30.0 && percent < 50.0 {
        percent_str = percent_str.red().to_string();
    } else if percent >= 50.0 && percent < 80.0 {
        percent_str = percent_str.yellow().to_string();
    } else if percent >= 80.0 && percent < 100.0 {
        percent_str = percent_str.green().to_string();
    } else {
        percent_str = percent_str.cyan().to_string();
    }
    percent_str
}

impl fmt::Display for Badges {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let percent = (self.check_items_checked as f32 / self.check_items as f32) * 100.0;
        write!(f, "{}/{} ({})", self.check_items_checked, self.check_items, parse_percent(percent))
    }
}

#[derive(Debug, Deserialize)]
pub struct CardInfo {
    name: String,
    badges: Badges
}

impl fmt::Display for CardInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.badges)
    }
}

pub fn get_month_lists(list_id: &str) -> Result<Vec<CardInfo>, Box<dyn std::error::Error>> {
    let params = [
        ("key", env::var("API_KEY").expect("Expected an api key in the environment")),
        ("token", env::var("TOKEN").expect("Expected token in the environment"))
    ];

    let url = format!("https://api.trello.com/1/lists/{}/cards", list_id);
    let client = reqwest::blocking::Client::new();

    let res: Vec<CardInfo> = client.get(url)
        .header(ACCEPT, "application/json")
        .query(&params)
        .send()?
        .json()?;

    Ok(res)
}

pub fn parse_month_list(month_lists: Vec<CardInfo>) {
    println!("{}", month_lists.first().unwrap().name);
    let cards = &month_lists[1..];

    let mut n_all_tasks = 0;
    let mut n_done_tasks = 0;
    for card in cards.iter() {
        println!("{}", card);
        n_all_tasks += card.badges.check_items;
        n_done_tasks += card.badges.check_items_checked;
    }

    let all_tasks_percent = (n_done_tasks as f32 / n_all_tasks as f32) * 100.0;

    println!("---------------------------");
    println!("Всего за месяц: {}/{} ({})", n_done_tasks, n_all_tasks, parse_percent(all_tasks_percent));
}