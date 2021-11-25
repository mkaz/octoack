use rand::seq::SliceRandom;
use reqwest::blocking::{Client, ClientBuilder};
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;

#[derive(Clone, Deserialize)]
struct Config {
    repos: Vec<String>,
    users: Vec<User>,
    slack_url: String,
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
struct GHUser {
    login: String,
}

#[derive(Deserialize, Debug)]
struct PullRequest {
    title: String,
    #[serde(rename = "html_url")]
    url: String,
    user: GHUser,
}

fn main() {
    let filename = "octoack.toml";
    let config_file = fs::read_to_string(filename).unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();

    let prs = get_pull_requests(config.clone());
    let db = Connection::open("octoack.sqlite").unwrap();

    // Send Slack Alerts
    for pr in prs {
        let mut stmt = db.prepare("SELECT url FROM notices WHERE url = ?").unwrap();
        let mut rows = stmt.query(params![pr.url]).unwrap();
        if let Some(_row) = rows.next().unwrap() {
            // row already exists
        } else {
            // Does not exist, insert into datbaase
            db.execute(" INSERT INTO notices (url) VALUES (?)", params![pr.url])
                .unwrap();

            // Send Slack alert
            let msg = format!(
                "{} A new Tinker pull request by {}.\n{}\n{}",
                get_emoji(),
                pr.user.login,
                pr.title,
                pr.url
            );

            let mut body = HashMap::new();
            body.insert("text", msg);

            Client::new()
                .post(config.slack_url.clone())
                .json(&body)
                .send()
                .unwrap();
        }
    }
}

fn get_emoji() -> String {
    let emoji = [
        ":arthur_dance:",
        ":bananadance:",
        ":carlton_dance:",
        ":dance:",
        ":dance_s:",
        ":donut-dance:",
        ":hamsterdance:",
        ":lisa-dance:",
        ":megaman-dance-2:",
        ":pandance:",
        ":peanuts-dance:",
        ":totdance:",
        ":totoro-dance:",
    ];

    return emoji.choose(&mut rand::thread_rng()).unwrap().to_string();
}

fn get_pull_requests(config: Config) -> Vec<PullRequest> {
    let mut pulls: Vec<PullRequest> = Vec::new();

    let apiurl = "https://api.github.com/search/issues";
    let client = ClientBuilder::new()
        .user_agent("Octoack/0.1.0")
        .build()
        .unwrap();

    let mut query = "type:pr+is:open+draft:false".to_string();
    for user in &config.users {
        query = format!("{}+author:{}", query, user.github.clone());
    }

    for repo in config.repos {
        let request_url = format!("{}?q={}+repo:{}", apiurl, query, repo);
        let response = client.get(&request_url).send().unwrap();
        let results = response.json::<GHQueryResult>().unwrap();
        pulls.extend(results.items);
    }

    return pulls;
}
