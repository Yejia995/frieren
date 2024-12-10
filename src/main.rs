use qbit_rs::model::{Credential, GetTorrentListArg, TorrentFilter};
use qbit_rs::Qbit;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    server: String,
    username: String,
    password: String,
    category: Option<String>,
    add_tags: Option<Vec<String>>,
    rule: Vec<Rule>,
}

#[derive(Deserialize)]
struct Rule {
    enabled: Option<bool>,
    source: Source,
    target: Target,
}

#[derive(Deserialize)]
struct Source {
    spec: String,
}

#[derive(Deserialize)]
struct Target {
    path: String,
    spec: String,
    tune: Option<i8>,
}

async fn find_torrent(api: &Qbit, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let arg = GetTorrentListArg {
        filter: Some(TorrentFilter::Completed),
        category: config.category.clone(),
        ..Default::default()
    };
    let torrents = api.get_torrent_list(arg).await?;
    for torrent in torrents {
        let files = api.get_torrent_contents(torrent.hash.as_ref().unwrap(), None).await?;
        for file in files {
            edit_torrent(api, &torrent.hash.as_ref().unwrap(), &file.name, config).await?;
        }
    }
    Ok(())
}

async fn edit_torrent(api: &Qbit, hash: &str, file_path: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    for rule in &config.rule {
        if rule.enabled.unwrap_or(true) {
            let pattern = regex::escape(&rule.source.spec)
                .replace("\\*", ".*")
                .replace("\\#", "(?P<ep>\\d+)");
            let re = Regex::new(&pattern).unwrap();
            if let Some(caps) = re.captures(file_name) {
                let episode = caps.name("ep").map_or(0, |m| m.as_str().parse::<i8>().unwrap_or(0));
                let new_name = rule.target.spec.replace("#", &format!("{:02}", episode + rule.target.tune.unwrap_or(0)));
                api.set_torrent_location(vec![hash.to_string()], &rule.target.path).await?;
                api.rename_file(hash, file_path, &new_name).await?;
                for tag in config.add_tags.as_ref().unwrap_or(&Vec::new()) {
                    api.add_torrent_tags(vec![hash.to_string()], vec![tag.clone()]).await?;
                }
                println!("Moved file \"{}\" to \"{}\"", file_path, Path::new(&rule.target.path).join(&new_name).display());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = std::env::current_exe()?.parent().unwrap().join("config.toml");
    let config: Config = toml::from_str(&fs::read_to_string(config_path)?)?;
    let credential = Credential::new(&config.username, &config.password);
    let api = Qbit::new(url::Url::parse(&config.server)?, credential);
    find_torrent(&api, &config).await?;
    Ok(())
}
