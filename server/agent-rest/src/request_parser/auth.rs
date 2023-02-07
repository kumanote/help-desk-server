use crate::{config, schema::AGENT_ACCESS_TOKEN_SCHEME, AppState, HttpError, Result};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{
        header::{ACCEPT_LANGUAGE, AUTHORIZATION},
        request::Parts,
    },
};
use domain::model;
use std::str::FromStr;

/// Extract login agent from http request "AUTHORIZATION" header
#[derive(Debug, Clone)]
pub struct CurrentActiveAgent(pub model::Agent);

fn extract_access_token_string(parts: &mut Parts) -> Option<String> {
    if let Some(value) = parts.headers.get(AUTHORIZATION) {
        if let Ok(s) = value.to_str() {
            let mut split_token = s.split_whitespace();
            let scheme = split_token.next().unwrap_or_default();
            if scheme == AGENT_ACCESS_TOKEN_SCHEME {
                split_token.next().map(ToOwned::to_owned)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[async_trait]
impl FromRequestParts<AppState> for CurrentActiveAgent {
    type Rejection = HttpError;
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self> {
        let access_token = extract_access_token_string(parts);
        if access_token.is_none() {
            return Err(HttpError::new_unauthorized());
        }

        let access_token = access_token.unwrap();
        let app_config = config::app_config();
        let parsed_agent_id =
            model::AgentAccessToken::validate(&app_config.agent_rest.secret_key, &access_token);
        if parsed_agent_id.is_none() {
            return Err(HttpError::new_unauthorized());
        }
        let agent_id = model::AgentId::from(parsed_agent_id.unwrap());
        let mut db_conn = state.db_connection_pool.get()?;
        let mut cache_conn = state.cache_connection_pool.get()?;

        if !cache::adapters::agent_access_token::is_valid(
            &mut cache_conn,
            &agent_id,
            &access_token,
        )? {
            return Err(HttpError::new_unauthorized());
        }
        let agent = database::adapters::agent::get_by_id(&mut db_conn, &agent_id)?;
        if agent.is_none() || !agent.as_ref().unwrap().is_active {
            return Err(HttpError::new_unauthorized());
        }
        let mut agent = model::Agent::from(agent.unwrap());
        if let Some(locale_value) = parts.headers.get(ACCEPT_LANGUAGE) {
            if let Ok(s) = locale_value.to_str() {
                let locale = model::Locale::from_str(s).unwrap_or_default();
                if agent.locale != locale {
                    // if the agent locale differs from the requested one.
                    database::adapters::agent::update_locale_by_id(
                        &mut db_conn,
                        &locale,
                        &agent.id,
                    )?;
                    agent.locale = locale;
                }
            }
        }
        Ok(Self(agent))
    }
}

/// Extract agent access token from http request "AUTHORIZATION" header
#[derive(Debug, Clone)]
pub struct AgentAccessToken(pub Option<model::AgentAccessToken>);

#[async_trait]
impl<S> FromRequestParts<S> for AgentAccessToken
where
    S: Sized + Sync,
{
    type Rejection = HttpError;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self> {
        let access_token = extract_access_token_string(parts);
        Ok(Self(access_token.map(model::AgentAccessToken::from)))
    }
}
