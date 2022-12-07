use crate::{DbConnection, SimpleConnection};

pub fn health_check(conn: &mut DbConnection) -> bool {
    let result = conn.batch_execute("SELECT 1");
    result.is_ok()
}
