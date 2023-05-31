use std::fmt::{Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::http::IssueDto;

struct Tags(Vec<String>);

impl Display for Tags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, tag| {
            result.and_then(|_| writeln!(f, "[{}]", tag))
        })
    }
}

pub struct Issue {
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

#[cfg(test)]
mod test {
    use std::fs;

    use crate::http::parse_response;
    use crate::model::Issue;

    fn read_file_to_string(path: impl Into<String>) -> String {
        let file_path = path.into();
        fs::read_to_string(file_path).expect("Failed to read file")
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