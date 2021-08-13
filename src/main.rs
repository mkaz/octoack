use reqwest::blocking::ClientBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;

#[derive(Deserialize)]
struct Config {
    Repos: Vec<String>,
    Users: Vec<User>,
}

#[derive(Deserialize)]
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Octoack!");
    let filename = "octoack.toml";
    let apiurl = "https://api.github.com/search/issues";

    let client = ClientBuilder::new().user_agent("Octoack/0.1.0").build()?;

    let config_file = fs::read_to_string(filename).unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();

    let mut query = HashMap::new();
    query.insert("type".to_string(), "pr".to_string());
    query.insert("is".to_string(), "open".to_string());
    query.insert("draft".to_string(), "false".to_string());

    for repo in config.Repos {
        query.insert("repo".to_string(), repo);

        // TODO: combine users into single query
        for user in &config.Users {
            query.insert("author".to_string(), user.github.clone());
            let query_args = hash_to_query(query.clone());

            let request_url = format!("{}?q={}", apiurl, query_args);
            // println!("Request URL: {}", request_url);
            //
            let response = client.get(&request_url).send()?;
            println!("Response: {:?}", response);
            let results = response.json::<GHQueryResult>()?;
            if results.total_count > 0 {
                for pr in results.items {
                    println!("Found: {} {}", pr.title, pr.url);
                }
            }
            break;
        }
    }

    Ok(())
}

fn hash_to_query(map: HashMap<String, String>) -> String {
    let mut q = "".to_string();

    for (key, val) in &map {
        q = format!("{}+{}:{}", q, key, val);
    }

    return q.to_string();
}
