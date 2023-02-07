use crate::{
    model::{AgentAccessToken, Email},
    repository::{AgentLoginRepository, AgentRepository},
    Error, Result,
};
use chrono::Duration;
use std::str::FromStr;

pub struct AgentLoginUseCaseInput {
    /// login email
    pub username: String,
    pub password: String,
    /// remote ip address
    pub client_ip: String,
    /// server secret key to generate token
    pub secret: String,
    pub access_token_duration: Duration,
    pub username_failure_limit: i32,
    pub ip_failure_limit: i32,
}

pub type AgentLoginUseCaseOutput = AgentAccessToken;

pub trait AgentLoginUseCase: Send + Sync + 'static {
    type Transaction;
    type AgentRepository: AgentRepository<Err = Error, Transaction = Self::Transaction>;
    type AgentLoginRepository: AgentLoginRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: AgentLoginUseCaseInput,
    ) -> Result<AgentLoginUseCaseOutput>;
}

pub struct AgentLoginUseCaseImpl<
    AR: AgentRepository<Err = Error>,
    ALR: AgentLoginRepository<Err = Error>,
> {
    agent_repository: AR,
    agent_login_repository: ALR,
}

impl<AR: AgentRepository<Err = Error>, ALR: AgentLoginRepository<Err = Error>>
    AgentLoginUseCaseImpl<AR, ALR>
{
    pub fn new(agent_repository: AR, agent_login_repository: ALR) -> Self {
        Self {
            agent_repository,
            agent_login_repository,
        }
    }
}

impl<
        TX,
        AR: AgentRepository<Err = Error, Transaction = TX>,
        ALR: AgentLoginRepository<Err = Error, Transaction = TX>,
    > AgentLoginUseCase for AgentLoginUseCaseImpl<AR, ALR>
{
    type Transaction = TX;
    type AgentRepository = AR;
    type AgentLoginRepository = ALR;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: AgentLoginUseCaseInput,
    ) -> Result<AgentLoginUseCaseOutput> {
        let is_login_blocked = self
            .agent_login_repository
            .is_blocked(&params.username, &params.client_ip)?;
        if is_login_blocked {
            return Err(Error::LoginBlocked);
        }
        let login = |tx: &mut Self::Transaction| {
            // check user email & password
            let email = match Email::from_str(&params.username) {
                Ok(email) => email,
                Err(_) => return Err(Error::InvalidLoginCredential),
            };
            let agent = self.agent_repository.get_by_email(tx, &email)?;
            if agent.is_none() {
                return Err(Error::InvalidLoginCredential);
            }
            let agent = agent.unwrap();
            if !agent.hashed_password.verify(&params.password) {
                return Err(Error::InvalidLoginCredential);
            }
            if !agent.is_active {
                return Err(Error::InvalidLoginCredential);
            }
            let access_token = AgentAccessToken::generate(
                &params.secret,
                &agent.id,
                params.access_token_duration,
            )?;
            self.agent_login_repository.set_access_token(
                &agent.id,
                &access_token,
                params.access_token_duration,
            )?;
            Ok(access_token)
        };
        match login(tx) {
            Ok(access_token) => {
                self.agent_login_repository
                    .clear_failed_count(&params.username, &params.client_ip)?;
                Ok(access_token)
            }
            Err(err) => match err {
                Error::SystemError { cause: _ } => Err(err),
                _ => {
                    let (username_failed_count, ip_failed_count) = self
                        .agent_login_repository
                        .record_login_failed(&params.username, &params.client_ip)?;
                    if 0 <= params.username_failure_limit
                        && params.username_failure_limit < username_failed_count
                    {
                        self.agent_login_repository
                            .set_login_blocked_by_username(&params.username)?;
                    }
                    if 0 <= params.ip_failure_limit && params.ip_failure_limit < ip_failed_count {
                        self.agent_login_repository
                            .set_login_blocked_by_ip(&params.client_ip)?;
                    }
                    Err(err)
                }
            },
        }
    }
}
