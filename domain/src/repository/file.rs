use crate::{
    model::{File, FileId},
    Result,
};
use database::DbConnection;

pub fn create(db_connection: &mut DbConnection, file: &File) -> Result<()> {
    database::adapters::file::create(db_connection, file.into())?;
    Ok(())
}

pub fn get_by_id(db_connection: &mut DbConnection, id: &FileId) -> Result<Option<File>> {
    let entity = database::adapters::file::get_by_id(db_connection, &id)?;
    Ok(entity.map(Into::into))
}

pub fn get_list_by_ids(db_connection: &mut DbConnection, ids: &Vec<&FileId>) -> Result<Vec<File>> {
    let ids = ids.iter().map(AsRef::as_ref).collect();
    let entities = database::adapters::file::get_list_by_ids(db_connection, &ids)?;
    Ok(entities.into_iter().map(Into::into).collect())
}
