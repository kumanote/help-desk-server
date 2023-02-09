use crate::{
    model::{Agent, Scope},
    repository::RoleRepository,
    Error, Result,
};

pub type AgentGetScopesUseCaseOutput = Vec<Scope>;

pub trait AgentGetScopesUseCase: Send + Sync + 'static {
    type Transaction;
    type RoleRepository: RoleRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        agent: &Agent,
    ) -> Result<AgentGetScopesUseCaseOutput>;
}

pub struct AgentGetScopesUseCaseImpl<RL: RoleRepository<Err = Error>> {
    role_repository: RL,
}

impl<RL: RoleRepository<Err = Error>> AgentGetScopesUseCaseImpl<RL> {
    pub fn new(role_repository: RL) -> Self {
        Self { role_repository }
    }
}

impl<TX, RL: RoleRepository<Err = Error, Transaction = TX>> AgentGetScopesUseCase
    for AgentGetScopesUseCaseImpl<RL>
{
    type Transaction = TX;
    type RoleRepository = RL;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        agent: &Agent,
    ) -> Result<AgentGetScopesUseCaseOutput> {
        self.role_repository
            .get_all_authorized_scopes_by_agent(tx, &agent.id)
    }
}
