use crate::{prelude::ConnectionLike, CacheConnection};

pub fn health_check(conn: &mut CacheConnection) -> bool {
    conn.check_connection()
}
