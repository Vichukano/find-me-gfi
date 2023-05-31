use serde::Deserialize;

const API_PATH: &str = "https://api.github.com/search/issues?q=";

#[derive(Deserialize, Debug)]
pub struct GitHubIssueResponse {
    pub total_count: i32,
    pub items: Vec<IssueDto>,
}

#[derive(Deserialize, Debug)]
pub struct IssueDto {
    pub url: String,
    pub title: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub labels: Vec<IssueLabel>,
}

#[derive(Deserialize, Debug)]
pub struct IssueLabel {
    pub name: String,
}

pub fn get_issues_from_github(token: String, lang: String, label: String, page: i32) -> GitHubIssueResponse {
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

pub fn parse_response(response: String) -> GitHubIssueResponse {
    serde_json::from_str(response.as_str()).unwrap()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::http::parse_response;

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
}