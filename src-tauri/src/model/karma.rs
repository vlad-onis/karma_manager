use sqlx::Encode;
use thiserror::Error;

//todo: add name in karma model

#[derive(Debug, Error)]
pub enum KarmaError {
    #[error("Could not convert {0} into a Karma Type")]
    InvalidNumericKarmaType(i32),
}

#[derive(Debug, Clone, Encode)]
pub struct KarmaPoint {
    purpose: KarmaType,
    name: String,
}

impl KarmaPoint {
    pub fn new(purpose: KarmaType, name: String) -> KarmaPoint {
        // Initially we set the closing type to the purpose
        // assuming it will be closed correctly, and we change that at
        // closing time
        KarmaPoint {
            purpose: purpose,
            name,
        }
    }

    pub fn get_purpose(&self) -> KarmaType {
        self.purpose.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub enum KarmaType {
    Work = 1,
    Social = 2,
    Sport = 3,
    Learning = 4,
    Sleeping = 5,
}

impl TryFrom<i32> for KarmaType {
    type Error = KarmaError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KarmaType::Work),
            2 => Ok(KarmaType::Social),
            3 => Ok(KarmaType::Sport),
            4 => Ok(KarmaType::Learning),
            5 => Ok(KarmaType::Sleeping),
            other_value => Err(KarmaError::InvalidNumericKarmaType(other_value)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Active,
    Closed,
}

impl From<String> for State {
    fn from(value: String) -> State {
        if value.to_lowercase() == *"active" {
            State::Active
        } else {
            State::Closed
        }
    }
}

#[derive(Debug, Clone)]
pub struct KarmaStatus {
    karma_id: u32,
    closed_with: KarmaType,
    state: State,
    timestamp: u64,
}

impl KarmaStatus {
    pub fn new(karma_id: u32, closed_with: KarmaType, state: State, timestamp: u64) -> KarmaStatus {
        KarmaStatus {
            karma_id,
            closed_with,
            state,
            timestamp,
        }
    }
}
