pub struct PagingResult<T: Sized> {
    pub total: u64,
    pub list: Vec<T>,
}

impl<T> Default for PagingResult<T> {
    fn default() -> Self {
        Self {
            total: 0,
            list: vec![],
        }
    }
}
