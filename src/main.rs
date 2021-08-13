use reqwest::blocking::ClientBuilder;
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Clone, Deserialize)]
struct Config {
    Repos: Vec<String>,
    Users: Vec<User>,
}

#[derive(Clone, Deserialize)]
struct User {
    github: String,
    slack: String,
}

#[derive(Deserialize, Debug)]
struct GHQueryResult {
    total_count: i32,
    items: Vec<PullRequest>,
}

#[derive(Deserialize, Debug)]
struct PullRequest {
    title: String,
    #[serde(rename = "html_url")]
    url: String,
}

fn main() {
    println!("Octoack!");
    let filename = "octoack.toml";

    let config_file = fs::read_to_string(filename).unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();

    let prs = get_pull_requests(config.clone());
    let db = Connection::open("octoack.sqlite").unwrap();

    // Send Slack Alerts
    for pr in prs {
        println!("Title: {}", pr.title);

        let mut stmt = db.prepare("SELECT url FROM notices WHERE url = ?").unwrap();
        let mut rows = stmt.query(params![pr.url]).unwrap();
        if let Some(row) = rows.next().unwrap() {
            println!("Already exists, ignore!");
        } else {
            println!("Does not exist, insert!");
            db.execute(" INSERT INTO notices (url) VALUES (?)", params![pr.url])
                .unwrap();
        }
    }
    // check if exists in datbase
    // - if not, send alert and add to database
}

fn get_pull_requests(config: Config) -> Vec<PullRequest> {
    let mut pulls: Vec<PullRequest> = Vec::new();

    let apiurl = "https://api.github.com/search/issues";
    let client = ClientBuilder::new()
        .user_agent("Octoack/0.1.0")
        .build()
        .unwrap();

    let mut query = "type:pr+is:open+draft:false".to_string();
    for user in &config.Users {
        query = format!("{}+author:{}", query, user.github.clone());
    }

    for repo in config.Repos {
        let request_url = format!("{}?q={}+repo:{}", apiurl, query, repo);
        let response = client.get(&request_url).send().unwrap();
        let results = response.json::<GHQueryResult>().unwrap();
        pulls.extend(results.items);
    }

    return pulls;
}
