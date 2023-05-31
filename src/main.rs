use std::env;

use clap::Parser;

use crate::http::get_issues_from_github;
use crate::model::Issue;

mod http;
mod model;

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
