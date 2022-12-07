use super::{FileId, MimeType};

pub const KB: usize = 1024;
pub const MB: usize = KB * 1024;
pub const GB: usize = MB * 1024;

#[derive(Debug, Clone)]
pub struct File {
    pub id: FileId,
    /// key in file storage. (e.g. S3 key name / GCS key name)
    pub stored_filename: String,
    /// original file name when uploaded
    pub original_filename: String,
    pub mime_type: MimeType,
}

impl<'a> Into<database::entities::NewFile<'a>> for &'a File {
    fn into(self) -> database::entities::NewFile<'a> {
        database::entities::NewFile {
            id: &self.id,
            stored_filename: &self.stored_filename,
            original_filename: &self.original_filename,
            mime_type: self.mime_type.as_str(),
        }
    }
}

impl From<database::entities::File> for File {
    fn from(value: database::entities::File) -> Self {
        Self {
            id: value.id.into(),
            stored_filename: value.stored_filename,
            original_filename: value.original_filename,
            mime_type: value.mime_type.into(),
        }
    }
}
