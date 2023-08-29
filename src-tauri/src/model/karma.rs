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
    close_type: KarmaType,
    closed: bool,
}

impl KarmaPoint {
    pub fn new(purpose: KarmaType) -> KarmaPoint {
        // Initially we set the closing type to the purpose
        // assuming it will be closed correctly, and we change that at
        // closing time
        KarmaPoint {
            purpose: purpose.clone(),
            close_type: purpose,
            closed: false,
        }
    }

    pub fn get_purpose(&self) -> KarmaType {
        self.purpose.clone()
    }

    pub fn get_close_type(&self) -> KarmaType {
        self.close_type.clone()
    }

    pub fn get_closed_status(&self) -> bool {
        self.closed
    }

    pub fn close(&mut self, close_type: KarmaType) {
        self.close_type = close_type;
        self.closed = true;
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
            4 => Ok(KarmaType::Sport),
            5 => Ok(KarmaType::Sport),
            other_value => Err(KarmaError::InvalidNumericKarmaType(other_value)),
        }
    }
}
