
use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use ic_cdk::export::Principal;


#[derive(Debug, Clone)]
pub struct EcoLegacy {
    users: Vec<User>,
    dao_proposals: Vec<Proposal>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub principal_id: Principal,
}

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub creator: Principal,
}

impl EcoLegacy {
    pub fn new() -> Self {
        EcoLegacy { 
            users: Vec::new(), 
            dao_proposals: Vec::new(),
        }
    }

    pub async fn register_user(&mut self, username: &str, password: &str, principal_id: Principal) -> bool {
        if self.users.iter().any(|user| user.username == username) {
            return false; // Username already exists
        }
        self.users.push(User {
            username: username.to_string(),
            password: password.to_string(),
            principal_id,
        });
        true
    }

    pub async fn login_user(&self, username: &str, password: &str) -> Option<Principal> {
        self.users.iter()
            .find(|user| user.username == username && user.password == password)
            .map(|user| user.principal_id)
    }

    pub async fn create_proposal(&mut self, title: &str, description: &str, creator: Principal) -> u64 {
        let new_id = self.dao_proposals.len() as u64 + 1;
        self.dao_proposals.push(Proposal {
            id: new_id,
            title: title.to_string(),
            description: description.to_string(),
            votes_for: 0,
            votes_against: 0,
            creator,
        });
        new_id
    }

    pub async fn vote_on_proposal(&mut self, proposal_id: u64, vote_for: bool) -> bool {
        if let Some(proposal) = self.dao_proposals.iter_mut().find(|p| p.id == proposal_id) {
            if vote_for {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }
            return true;
        }
        false
    }
}

lazy_static! {
    pub static ref ECO_LEGACY_INSTANCE: Arc<Mutex<EcoLegacy>> = Arc::new(Mutex::new(EcoLegacy::new()));
}
