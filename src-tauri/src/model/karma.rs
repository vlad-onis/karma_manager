use serde::Serialize;
use sqlx::Encode;
use sqlx::{sqlite::SqliteRow, Error as SqlxError, FromRow, Row};
use thiserror::Error;
//todo: add name in karma model

#[derive(Debug, Error, Serialize)]
pub enum KarmaError {
    #[error("Could not convert {0} into a Karma Type")]
    InvalidNumericKarmaType(i32),

    #[error("Failed to convert {0} into a State object")]
    UnsupportedStatus(String),
}

#[derive(Debug, Clone, Encode, Serialize)]
pub struct KarmaPoint {
    id: Option<i32>,
    purpose: KarmaType,
    name: String,
}

impl KarmaPoint {
    pub fn new(purpose: KarmaType, name: String) -> KarmaPoint {
        // Initially we set the closing type to the purpose
        // assuming it will be closed correctly, and we change that at
        // closing time
        KarmaPoint {
            id: None,
            purpose,
            name,
        }
    }

    pub fn with_id(id: i32, purpose: KarmaType, name: String) -> KarmaPoint {
        // Initially we set the closing type to the purpose
        // assuming it will be closed correctly, and we change that at
        // closing time
        KarmaPoint {
            id: Some(id),
            purpose,
            name,
        }
    }

    pub fn get_purpose(&self) -> KarmaType {
        self.purpose.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }
}

impl<'r> FromRow<'r, SqliteRow> for KarmaPoint {
    fn from_row(row: &'r SqliteRow) -> Result<Self, SqlxError> {
        let id = row.try_get("id")?;
        let name: String = row.try_get("name")?;

        let purpose: i32 = row.try_get("purpose")?;
        let purpose = purpose
            .try_into()
            .map_err(|e| SqlxError::Decode(Box::new(e)))?;

        Ok(KarmaPoint::with_id(id, purpose, name))
    }
}

impl<'r> FromRow<'r, SqliteRow> for KarmaStatus {
    fn from_row(row: &'r SqliteRow) -> Result<Self, SqlxError> {
        let karma_id = row.try_get("karma_id")?;
        let current_state: String = row.try_get("current_state")?;
        let timestamp: i64 = row.try_get("timestamp")?;

        let closed_with: i32 = row.try_get("closed_with")?;
        if closed_with == 0 {
            return Ok(KarmaStatus::new(karma_id, current_state.into(), timestamp));
        }

        let closed_with: KarmaType = closed_with
            .try_into()
            .map_err(|e| SqlxError::Decode(Box::new(e)))?;

        Ok(KarmaStatus::with_closed_reason(
            karma_id,
            current_state.into(),
            timestamp,
            closed_with,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct KarmaStatus {
    pub karma_id: i32,
    pub closed_with: Option<KarmaType>,
    pub state: State,
    pub timestamp: i64,
}

impl KarmaStatus {
    pub fn new(karma_id: i32, state: State, timestamp: i64) -> KarmaStatus {
        KarmaStatus {
            karma_id,
            closed_with: None,
            state,
            timestamp,
        }
    }

    pub fn with_closed_reason(
        karma_id: i32,
        state: State,
        timestamp: i64,
        closed_with: KarmaType,
    ) -> KarmaStatus {
        KarmaStatus {
            karma_id,
            closed_with: Some(closed_with),
            state,
            timestamp,
        }
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::Active => format!("active"),
            State::Closed => format!("closed"),
        }
    }
}

impl TryFrom<&str> for State {
    type Error = KarmaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(State::Active),
            "closed" => Ok(State::Closed),
            state => Err(KarmaError::UnsupportedStatus(state.to_string())),
        }
    }
}
