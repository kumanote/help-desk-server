use crate::model::Workspace;

pub trait WorkspaceRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn create(&self, tx: &mut Self::Transaction, workspace: &Workspace) -> Result<(), Self::Err>;
    fn update(&self, tx: &mut Self::Transaction, workspace: &Workspace) -> Result<(), Self::Err>;
    fn get(&self, tx: &mut Self::Transaction) -> Result<Option<Workspace>, Self::Err>;
}
