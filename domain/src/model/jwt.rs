use super::AgentId;
use crate::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub exp: i64,
    pub nbf: i64,
    pub id: String,
}

fn generate_token(secret: &str, subject: &str, exp: i64, nbf: i64, id: String) -> Result<String> {
    let sub = subject.to_owned();
    let claims = Claims { sub, exp, nbf, id };
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

fn decode_token(secret: &str, token: &str) -> Result<TokenData<Claims>> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?)
}

macro_rules! def_token {
    ($struct_name:ident, $id:ty, $subject:expr) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name(String);

        impl $struct_name {
            pub fn generate(
                secret: &str,
                id: &$id,
                valid_duration: chrono::Duration,
            ) -> crate::Result<Self> {
                let subject: &'static str = $subject;
                let now = chrono::Utc::now();
                let nbf = now.timestamp();
                let exp = (now + valid_duration).timestamp();
                let token = generate_token(secret, subject, exp, nbf, id.to_string())?;
                Ok(Self(token))
            }

            pub fn validate(secret: &str, token: &str) -> Option<String> {
                if let Ok(token_data) = decode_token(secret, token) {
                    let claims = token_data.claims;
                    let subject: &'static str = $subject;
                    if &claims.sub != subject {
                        None
                    } else {
                        let now = chrono::Utc::now().timestamp();
                        if now < claims.nbf || claims.exp < now {
                            None
                        } else {
                            Some(claims.id)
                        }
                    }
                } else {
                    None
                }
            }

            #[inline(always)]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $struct_name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = str;
            fn deref(&self) -> &str {
                &self.0
            }
        }

        impl Into<String> for $struct_name {
            fn into(self) -> String {
                self.0
            }
        }

        impl From<String> for $struct_name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

def_token!(AgentAccessToken, AgentId, "agent");
def_token!(ResetPasswordToken, AgentId, "reset_password");

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::AgentId;

    #[test]
    fn test_jwt_token() {
        let secret = "test";
        let agent_id = AgentId::generate();
        let valid_duration = chrono::Duration::seconds(1);
        let token = AgentAccessToken::generate(secret, &agent_id, valid_duration).unwrap();
        let token_string = token.to_string();
        let parsed = AgentAccessToken::validate(secret, &token_string);
        assert_eq!(parsed.unwrap(), agent_id.to_string());
    }

    #[test]
    fn test_wrong_token_value() {
        let secret = "test";
        let invalid_token = "invalidtoken";
        let parsed = AgentAccessToken::validate(secret, &invalid_token);
        assert!(parsed.is_none());
    }

    #[test]
    fn test_wrong_secret() {
        let secret = "test";
        let agent_id = AgentId::generate();
        let valid_duration = chrono::Duration::seconds(1);
        let token = AgentAccessToken::generate(secret, &agent_id, valid_duration).unwrap();
        let token_string = token.to_string();
        let parsed = AgentAccessToken::validate("test1", &token_string);
        assert!(parsed.is_none());
    }
}
