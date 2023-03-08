use crate::{
    model::{Agent, HashedPassword},
    repository::AgentRepository,
    Error, Result,
};

pub struct AgentChangePasswordUseCaseInput {
    pub current_password: String,
    pub new_password: String,
}

pub trait AgentChangePasswordUseCase: Send + Sync + 'static {
    type Transaction;
    type AgentRepository: AgentRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        agent: &mut Agent,
        params: AgentChangePasswordUseCaseInput,
    ) -> Result<()>;
}

pub struct AgentChangePasswordUseCaseImpl<AR: AgentRepository<Err = Error>> {
    agent_repository: AR,
}

impl<AR: AgentRepository<Err = Error>> AgentChangePasswordUseCaseImpl<AR> {
    pub fn new(agent_repository: AR) -> Self {
        Self { agent_repository }
    }
}

impl<TX, AR: AgentRepository<Err = Error, Transaction = TX>> AgentChangePasswordUseCase
    for AgentChangePasswordUseCaseImpl<AR>
{
    type Transaction = TX;
    type AgentRepository = AR;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        agent: &mut Agent,
        params: AgentChangePasswordUseCaseInput,
    ) -> Result<()> {
        // check the current password is correct.
        if !agent.hashed_password.verify(&params.current_password) {
            return Err(Error::WrongPasswordForEditingSecuritySettings);
        }
        // compute hashed password & update
        let hashed_password = HashedPassword::new_from_plain_text(&params.new_password)?;
        self.agent_repository
            .update_hashed_password(tx, agent, hashed_password)?;
        Ok(())
    }
}
