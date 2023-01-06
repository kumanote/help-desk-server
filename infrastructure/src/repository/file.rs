use database::DbConnection;
use domain::model::{File, FileId};
use domain::repository::FileRepository;

pub struct FileRepositoryImpl {}

impl FileRepository for FileRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn create(&self, tx: &mut Self::Transaction, file: &File) -> Result<(), Self::Err> {
        database::adapters::file::create(tx, file.into())?;
        Ok(())
    }

    fn get_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &FileId,
    ) -> Result<Option<File>, Self::Err> {
        let entity = database::adapters::file::get_by_id(tx, &id)?;
        Ok(entity.map(Into::into))
    }

    fn get_list_by_ids(
        &self,
        tx: &mut Self::Transaction,
        ids: &Vec<&FileId>,
    ) -> Result<Vec<File>, Self::Err> {
        let ids = ids.iter().map(AsRef::as_ref).collect();
        let entities = database::adapters::file::get_list_by_ids(tx, &ids)?;
        Ok(entities.into_iter().map(Into::into).collect())
    }
}
