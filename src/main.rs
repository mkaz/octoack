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

fn main() {
    println!("Octoack!");
    let filename = "octoack.toml";
    let apiurl = "https://api.github.com/search/issues";

    let config_file = fs::read_to_string(filename).unwrap();
    let config: Config = toml::from_str(&config_file).unwrap();

    let mut query = HashMap::new();
    query.insert("type".to_string(), "pr".to_string());
    query.insert("draft".to_string(), "false".to_string());

    for repo in config.Repos {
        query.insert("repo".to_string(), repo);

        for user in &config.Users {
            query.insert("author".to_string(), user.github.clone());

            println!("Query: {}", hash_to_query(query.clone()));
        }
    }
}

fn hash_to_query(map: HashMap<String, String>) -> String {
    let mut q = "".to_string();

    for (key, val) in &map {
        q = format!("{}+{}:{}", q, key, val);
    }

    return q.to_string();
}
