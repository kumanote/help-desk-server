use crate::{
    model::{Agent, AgentAccessToken},
    repository::AgentLoginRepository,
    Error, Result,
};

pub struct AgentLogoutUseCaseInput {
    pub access_token: AgentAccessToken,
}

pub trait AgentLogoutUseCase: Send + Sync + 'static {
    type Transaction;
    type AgentLoginRepository: AgentLoginRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        agent: &Agent,
        params: AgentLogoutUseCaseInput,
    ) -> Result<()>;
}

pub struct AgentLogoutUseCaseImpl<ALR: AgentLoginRepository<Err = Error>> {
    agent_login_repository: ALR,
}

impl<ALR: AgentLoginRepository<Err = Error>> AgentLogoutUseCaseImpl<ALR> {
    pub fn new(agent_login_repository: ALR) -> Self {
        Self {
            agent_login_repository,
        }
    }
}

impl<TX, ALR: AgentLoginRepository<Err = Error, Transaction = TX>> AgentLogoutUseCase
    for AgentLogoutUseCaseImpl<ALR>
{
    type Transaction = TX;
    type AgentLoginRepository = ALR;
    fn execute(
        &self,
        _: &mut Self::Transaction,
        agent: &Agent,
        params: AgentLogoutUseCaseInput,
    ) -> Result<()> {
        self.agent_login_repository
            .delete_access_token(&agent.id, &params.access_token)?;
        Ok(())
    }
}
