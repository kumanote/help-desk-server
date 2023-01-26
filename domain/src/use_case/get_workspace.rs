use crate::{model::Workspace, repository::WorkspaceRepository, Error, Result};

pub type GetWorkspaceUseCaseOutput = Option<Workspace>;

pub trait GetWorkspaceUseCase: Send + Sync + 'static {
    type Transaction;
    type WorkspaceRepository: WorkspaceRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(&self, tx: &mut Self::Transaction) -> Result<GetWorkspaceUseCaseOutput>;
}

pub struct GetWorkspaceUseCaseImpl<WR: WorkspaceRepository<Err = Error>> {
    workspace_repository: WR,
}

impl<WR: WorkspaceRepository<Err = Error>> GetWorkspaceUseCaseImpl<WR> {
    pub fn new(workspace_repository: WR) -> Self {
        Self {
            workspace_repository,
        }
    }
}

impl<TX, WR: WorkspaceRepository<Err = Error, Transaction = TX>> GetWorkspaceUseCase
    for GetWorkspaceUseCaseImpl<WR>
{
    type Transaction = TX;
    type WorkspaceRepository = WR;

    fn execute(&self, tx: &mut Self::Transaction) -> Result<GetWorkspaceUseCaseOutput> {
        self.workspace_repository.get(tx)
    }
}
