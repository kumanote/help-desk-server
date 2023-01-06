use crate::model::{File, FileId};

pub trait FileRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn create(&self, tx: &mut Self::Transaction, file: &File) -> Result<(), Self::Err>;
    fn get_by_id(&self, tx: &mut Self::Transaction, id: &FileId)
        -> Result<Option<File>, Self::Err>;
    fn get_list_by_ids(
        &self,
        tx: &mut Self::Transaction,
        ids: &Vec<&FileId>,
    ) -> Result<Vec<File>, Self::Err>;
}
