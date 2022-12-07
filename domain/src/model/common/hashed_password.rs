use crate::Result;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Params, Pbkdf2,
};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new_from_plain_text(plain_password: &str) -> Result<Self> {
        let hashed_string = hash_password(plain_password)?;
        Ok(Self(hashed_string))
    }

    pub fn verify(&self, plain_password: &str) -> bool {
        verify_password(self.as_str(), plain_password)
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for HashedPassword {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for HashedPassword {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for HashedPassword {
    fn from(value: String) -> Self {
        Self(value)
    }
}

fn verify_password(plain: &str, hashed: &str) -> bool {
    let password_hash = match PasswordHash::new(hashed) {
        Ok(parsed) => parsed,
        Err(_) => return false,
    };
    Pbkdf2
        .verify_password(plain.as_bytes(), &password_hash)
        .is_ok()
}

fn hash_password(plain: &str) -> Result<String> {
    let params = Params {
        rounds: 36000,
        output_length: 32,
    };
    let salt = SaltString::generate(&mut OsRng);
    Pbkdf2
        .hash_password_customized(plain.as_bytes(), None, None, params, &salt)
        .map(|hash_password| hash_password.to_string())
        .map_err(Into::into)
}
