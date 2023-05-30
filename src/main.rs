use std::env;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Deserialize;

const API_PATH: &str = "https://api.github.com/search/issues?q=";

struct Tags(Vec<String>);

impl Display for Tags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, tag| {
            result.and_then(|_| writeln!(f, "[{}]", tag))
        })
    }
}

struct Issue {
    description: String,
    tags: Tags,
    url: String,
    create_date_time: DateTime<Utc>,
    update_date_time: DateTime<Utc>,
}

impl Display for Issue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Description: {}\ncreated: {}\nLast updated: {}\nURL: {}\nTags: {}",
               self.description,
               self.create_date_time,
               self.update_date_time,
               self.url,
               self.tags
        )
    }
}

impl From<IssueDto> for Issue {
    fn from(value: IssueDto) -> Self {
        Issue {
            description: value.title,
            tags: Tags(value.labels.into_iter().map(|x| x.name).collect()),
            url: value.url.replace("api.", "").replace("/repos", ""),
            create_date_time: DateTime::from_str(value.created_at.as_str()).unwrap(),
            update_date_time: DateTime::from_str(value.updated_at.as_str()).unwrap(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct GitHubIssueResponse {
    total_count: i32,
    items: Vec<IssueDto>,
}

#[derive(Deserialize, Debug)]
struct IssueDto {
    url: String,
    title: String,
    state: String,
    created_at: String,
    updated_at: String,
    labels: Vec<IssueLabel>,
}

#[derive(Deserialize, Debug)]
struct IssueLabel {
    name: String,
}

fn parse_response(response: String) -> GitHubIssueResponse {
    serde_json::from_str(response.as_str()).unwrap()
}

fn get_issues_from_github(token: String, lang: String, label: String, page: i32) -> GitHubIssueResponse {
    //q=windows+label:bug+language:python+state:open&sort=created&order=asc
    let query = format!("{}label:{}+language:{}+state:open&sort=created&per_page=100&page={}", API_PATH, label, lang, page);
    let body: String = ureq::get(query.as_str())
        .set("Accept", "application/vnd.github+json")
        .set("Authorization", format!("Bearer {}", token).as_str())
        .set("X-GitHub-Api-Version", "2022-11-28")
        .call().unwrap()
        .into_string().unwrap();
    parse_response(body)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    language: String,
    #[arg(short, long)]
    tag: String,
    #[arg(short, long)]
    pages: Option<i32>,
}

fn main() {
    let env_load_result = dotenv::dotenv().ok();
    env_logger::init();
    log::info!("Result of dotenv: {}", env_load_result.is_some());
    let token = env::var("TOKEN").unwrap();
    let args = Args::parse();
    let lang = args.language;
    let label = args.tag;
    let pages = args.pages.unwrap_or(1);
    let mut count = 1;
    for p in 1..=pages {
        let issue_dtos = get_issues_from_github(
            token.clone(), lang.clone(), label.clone(), p,
        ).items;
        let issues: Vec<Issue> = issue_dtos.into_iter().map(|i| i.into()).collect();
        for i in issues.iter() {
            println!("{} {}", count, i);
            count = count + 1;
        }
    }
}


#[cfg(test)]
mod test {
    use std::fs;

    use crate::{Issue, parse_response};

    fn read_file_to_string(path: impl Into<String>) -> String {
        let file_path = path.into();
        fs::read_to_string(file_path).expect("Failed to read file")
    }

    #[test]
    fn should_construct_dto_from_json() {
        let result = std::panic::catch_unwind(|| {
            let content = read_file_to_string("resources/github-issue-response-example.json");
            let issue = parse_response(content);
            println!("{:?}", issue);
        }
        );
        assert!(result.is_ok());
    }

    #[test]
    fn should_map_from_dto_to_model() {
        let content = read_file_to_string("resources/github-issue-response-example.json");
        let response = parse_response(content);
        let model: Vec<Issue> = response.items.into_iter().map(|x| x.into()).collect();
        assert_eq!(model.len(), 1);
        model.into_iter().for_each(|x| println!("{}", x));
    }
}