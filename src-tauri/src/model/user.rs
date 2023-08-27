use async_trait::async_trait;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use sqlx::{sqlite::SqliteRow, Error as SqlxError, FromRow, Row};
use thiserror::Error;

const MIN_USERNAME_SIZE: usize = 6;
const MIN_PASSWORD_SIZE: usize = 8;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Failed validate password: {0}")]
    PasswordValidation(#[from] PasswordError),

    #[error("Failed validate username: {0}")]
    UsernameValidation(#[from] UsernameError),
}

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Failed to hash the password")]
    PasswordHash(#[from] BcryptError),

    #[error("Password size should be at least {MIN_PASSWORD_SIZE}")]
    PasswordSize,

    #[error("Password should cointain at least 1 digit")]
    NoDigit,

    #[error("Password should cointain at least 1 special char")]
    NoSpecialChar,

    #[error("Password should cointain at least 1 uppercase digit")]
    NoUppercaseLetter,

    #[error("Failed to convert from db row")]
    DbRow(#[from] SqlxError),
}

#[derive(Error, Debug)]
pub enum UsernameError {
    #[error("Username size should be at least {MIN_USERNAME_SIZE}")]
    UsernameSize,

    #[error("Username should start with lowercase letter")]
    DoesntStartWithLowercaseLetter,

    #[error("Failed to convert from db row")]
    DbRow(#[from] SqlxError),
}

#[derive(Debug, PartialEq)]
pub struct User {
    pub username: Username,
    pub hashed_password: Password,
}

impl User {
    pub fn new(username: &str, password: &str) -> Result<User, UserError> {
        let username = Username::new(username)?;
        let hashed_password = Password::new(password)?;

        Ok(User {
            username,
            hashed_password,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Username(String);

#[derive(Debug, PartialEq)]
pub struct Password(String);

impl Username {
    pub fn new(username: &str) -> Result<Username, UsernameError> {
        if username.len() < MIN_USERNAME_SIZE {
            return Err(UsernameError::UsernameSize);
        }

        // This hsould not fail because of the above check
        let username_chrs: Vec<char> = username.chars().collect();
        let first = username_chrs[0];
        if !first.is_lowercase() {
            return Err(UsernameError::DoesntStartWithLowercaseLetter);
        }

        Ok(Username(username.to_string()))
    }

    pub fn get_username(&self) -> String {
        self.0.clone()
    }
}

impl Password {
    pub fn new(password: &str) -> Result<Password, PasswordError> {
        let special_characters = "!@#$%^&*().,:; ";
        let mut contains_special_char = false;
        let mut contains_uppercase_letter = false;
        let mut contains_digit = false;

        if password.len() < MIN_PASSWORD_SIZE {
            return Err(PasswordError::PasswordSize);
        }

        for ch in password.chars() {
            if special_characters.contains(ch) {
                contains_special_char = true;
            }

            if ch.is_uppercase() {
                contains_uppercase_letter = true;
            }

            if ch.is_digit(10) {
                contains_digit = true;
            }
        }

        if !contains_digit {
            return Err(PasswordError::NoDigit);
        }

        if !contains_special_char {
            return Err(PasswordError::NoSpecialChar);
        }

        if !contains_uppercase_letter {
            return Err(PasswordError::NoUppercaseLetter);
        }

        let hashed_password = bcrypt::hash(password, DEFAULT_COST)?;
        Ok(Password(hashed_password))
    }

    pub fn from_hashed(pass: &str) -> Password {
        Password(pass.to_string())
    }

    pub fn get_password(&self) -> String {
        self.0.clone()
    }
}

impl<'r> FromRow<'r, SqliteRow> for User {
    fn from_row(row: &'r SqliteRow) -> Result<Self, SqlxError> {
        let username = row.try_get("username")?;
        let password = row.try_get("password")?;

        let username = Username::new(username).map_err(|e| SqlxError::Decode(Box::new(e)))?;
        let hashed_password = Password::from_hashed(password);

        Ok(User {
            username,
            hashed_password,
        })
    }
}

#[cfg(test)]
mod user_tests {
    use super::*;
    use bcrypt;

    #[test]
    pub fn test_username_too_short() {
        let user = User::new("vlad", "V1@eflsjdfnsdf");
        match user {
            Err(UserError::UsernameValidation(UsernameError::UsernameSize)) => {
                assert!(true);
            }
            res => {
                let statement =
                    format!("Should have been a Username size error but it is {:?}", res);
                assert!(false, "{}", statement);
            }
        }
    }

    #[test]
    pub fn test_invalid_username() {
        let user = User::new("1vladonzis", "V1@eflsjdfnsdf");
        match user {
            Err(UserError::UsernameValidation(UsernameError::DoesntStartWithLowercaseLetter)) => {
                assert!(true);
            }
            res => {
                let statement = format!(
                    "Should have been a UsernameError::DoesntStartWithLowercaseLetter error
                        but it is a {:?}",
                    res
                );
                assert!(false, "{}", statement);
            }
        }
    }

    #[test]
    pub fn test_valid_username() {
        let user = User::new("vladonzis", "V1@eflsjdfnsdf").unwrap();
        assert_eq!(user.username, Username("vladonzis".to_string()));
        bcrypt::verify("V1@eflsjdfnsdf", &user.hashed_password.0).unwrap();
    }
}
