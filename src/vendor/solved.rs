use serde::{Deserialize, Serialize};
use ureq::Agent;

#[derive(Serialize, Deserialize)]
pub struct Problem {
    #[serde(rename(deserialize = "problemId"))]
    pub id: u32,
    #[serde(rename(deserialize = "titleKo"))]
    pub title: String,
    pub level: u32,
}

pub fn find_problem_by_id(agent: &Agent, id: u32) -> eyre::Result<Option<Problem>> {
    let id_str = id.to_string();
    let response = agent
        .get("https://solved.ac/api/v3/problem/show")
        .query("problemId", &id_str)
        .call()?;
    if response.status() == 404 {
        Ok(None)
    } else {
        Ok(Some(response.into_json()?))
    }
}
