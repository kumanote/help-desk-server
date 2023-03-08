use crate::{
    model::{Agent, AgentName, Email},
    repository::AgentRepository,
    Error, Result,
};
use std::str::FromStr;

pub struct AgentUpdateProfileUseCaseInput {
    pub email: String,
    pub name: String,
}

pub trait AgentUpdateProfileUseCase: Send + Sync + 'static {
    type Transaction;
    type AgentRepository: AgentRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        agent: &mut Agent,
        params: AgentUpdateProfileUseCaseInput,
    ) -> Result<()>;
}

pub struct AgentUpdateProfileUseCaseImpl<AR: AgentRepository<Err = Error>> {
    agent_repository: AR,
}

impl<AR: AgentRepository<Err = Error>> AgentUpdateProfileUseCaseImpl<AR> {
    pub fn new(agent_repository: AR) -> Self {
        Self { agent_repository }
    }
}

impl<TX, AR: AgentRepository<Err = Error, Transaction = TX>> AgentUpdateProfileUseCase
    for AgentUpdateProfileUseCaseImpl<AR>
{
    type Transaction = TX;
    type AgentRepository = AR;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        agent: &mut Agent,
        params: AgentUpdateProfileUseCaseInput,
    ) -> Result<()> {
        // validate inputs
        let email = Email::from_str(&params.email)?;
        let name = AgentName::from_str(&params.name)?;

        if email != agent.email {
            // check the uniqueness
            if self.agent_repository.get_by_email(tx, &email)?.is_some() {
                // duplicated
                return Err(Error::DuplicatedEmail);
            }
        }

        self.agent_repository
            .update_profile(tx, agent, email, name)?;

        Ok(())
    }
}
