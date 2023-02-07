use cache::CacheConnectionPool;
use chrono::Duration;
use database::DbConnection;
use domain::model::{AgentAccessToken, AgentId};
use domain::repository::AgentLoginRepository;

const DEFAULT_FAILURE_RECORD_CACHE_MINUTES: i64 = 10;

pub struct AgentLoginRepositoryImpl {
    cache_connection_pool: CacheConnectionPool,
    failure_record_cache_ttl: Duration,
}

impl AgentLoginRepositoryImpl {
    pub fn new(cache_connection_pool: CacheConnectionPool) -> Self {
        Self {
            cache_connection_pool,
            failure_record_cache_ttl: Duration::minutes(DEFAULT_FAILURE_RECORD_CACHE_MINUTES),
        }
    }

    pub fn new_with_ttl(
        cache_connection_pool: CacheConnectionPool,
        failure_record_cache_ttl: Duration,
    ) -> Self {
        Self {
            cache_connection_pool,
            failure_record_cache_ttl,
        }
    }
}

impl AgentLoginRepository for AgentLoginRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn is_blocked(&self, username: &str, ip_address: &str) -> Result<bool, Self::Err> {
        let mut cache_conn = self.cache_connection_pool.get()?;
        if cache::adapters::login_blocked_by_username::is_blocked(&mut cache_conn, username)? {
            return Ok(true);
        }
        if cache::adapters::login_blocked_by_ip::is_blocked(&mut cache_conn, ip_address)? {
            return Ok(true);
        }
        Ok(false)
    }

    fn record_login_failed(
        &self,
        username: &str,
        ip_address: &str,
    ) -> Result<(i32, i32), Self::Err> {
        let mut cache_conn = self.cache_connection_pool.get()?;
        let username_failed_count = cache::adapters::login_failed_by_username::increment(
            &mut cache_conn,
            username,
            self.failure_record_cache_ttl,
        )?;
        let ip_failed_count = cache::adapters::login_failed_by_ip::increment(
            &mut cache_conn,
            ip_address,
            self.failure_record_cache_ttl,
        )?;
        Ok((username_failed_count, ip_failed_count))
    }

    fn clear_failed_count(&self, username: &str, ip_address: &str) -> Result<(), Self::Err> {
        let mut cache_conn = self.cache_connection_pool.get()?;
        cache::adapters::login_failed_by_username::delete(&mut cache_conn, username)?;
        cache::adapters::login_failed_by_ip::delete(&mut cache_conn, ip_address)?;
        Ok(())
    }

    fn set_login_blocked_by_username(&self, username: &str) -> Result<(), Self::Err> {
        let mut cache_conn = self.cache_connection_pool.get()?;
        cache::adapters::login_blocked_by_username::set_blocked(
            &mut cache_conn,
            username,
            self.failure_record_cache_ttl,
        )?;
        Ok(())
    }

    fn set_login_blocked_by_ip(&self, ip_address: &str) -> Result<(), Self::Err> {
        let mut cache_conn = self.cache_connection_pool.get()?;
        cache::adapters::login_blocked_by_ip::set_blocked(
            &mut cache_conn,
            ip_address,
            self.failure_record_cache_ttl,
        )?;
        Ok(())
    }

    fn set_access_token(
        &self,
        agent_id: &AgentId,
        token: &AgentAccessToken,
        ttl: Duration,
    ) -> Result<(), Self::Err> {
        let mut cache_conn = self.cache_connection_pool.get()?;
        cache::adapters::agent_access_token::set(&mut cache_conn, &agent_id, &token, ttl)?;
        Ok(())
    }
}
