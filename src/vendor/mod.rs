use ureq::Agent;

mod solved;
pub use solved::Problem;

pub struct Client {
    agent: Agent,
}

impl Client {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
        }
    }

    pub fn find_problem_by_id(&self, id: u32) -> eyre::Result<Option<Problem>> {
        solved::find_problem_by_id(&self.agent, id)
    }
}
