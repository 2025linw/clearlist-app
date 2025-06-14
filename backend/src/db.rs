use deadpool_postgres::{Manager, ManagerConfig, Object, Pool};
use tokio_postgres::{Config, NoTls, Row, types::ToSql};

use crate::error::Error;

#[derive(Clone)]
pub struct DatabaseConn {
    pool: Pool,
}

impl DatabaseConn {
    pub fn connect(
        host: String,
        port: u16,
        database: String,
        user: String,
        pass: String,
    ) -> Result<Self, Error> {
        let mut pg_config = Config::new();
        pg_config.host(host).port(port);
        pg_config.user(user).password(pass).dbname(database);

        let manager = Manager::from_config(pg_config, NoTls, ManagerConfig::default());

        let pool = Pool::builder(manager)
            .max_size(16)
            .build()
            .map_err(|e| Error::Internal(e.to_string()))?;

        Ok(Self { pool })
    }

    pub async fn is_active(&self) -> bool {
        if self.pool.is_closed() {
            return false;
        }

        let conn = match self.pool.get().await {
            Ok(c) => c,
            Err(_) => {
                // TODO: check if there are any errors that are benign
                return false;
            }
        };

        if conn.query_one("SELECT 1", &[]).await.is_err() {
            // TODO: check if there are any errors that are benign
            return false;
        }

        true
    }

    /// Get connection from pool
    ///
    /// This allows for more control over queries that are required
    /// to be atomic or able to be rolled-back.
    ///
    /// For simple queries, using the `query_*` methods is more desirable if not
    /// preferred.
    pub async fn get_conn(&self) -> Result<Object, Error> {
        Ok(self.pool.get().await?)
    }

    pub async fn query_select_one(
        &self,
        query: String,
        params: Vec<&(dyn ToSql + Sync)>,
    ) -> Result<Option<Row>, Error> {
        let conn = self.pool.get().await?;

        Ok(conn.query_opt(&query, &params).await?)
    }

    pub async fn query_select_many(
        &self,
        query: String,
        params: Vec<&(dyn ToSql + Sync)>,
    ) -> Result<Vec<Row>, Error> {
        let conn = self.pool.get().await?;

        Ok(conn.query(&query, &params).await?)
    }

    pub async fn query_insert(
        &self,
        query: String,
        params: Vec<&(dyn ToSql + Sync)>,
    ) -> Result<Row, Error> {
        let mut conn = self.pool.get().await?;
        let transaction = conn.transaction().await?;

        let row = transaction.query_one(&query, &params).await?;

        transaction.commit().await?;

        Ok(row)
    }

    pub async fn query_update(
        &self,
        query: String,
        params: Vec<&(dyn ToSql + Sync)>,
    ) -> Result<Option<Row>, Error> {
        let mut conn = self.pool.get().await?;
        let transaction = conn.transaction().await?;

        let row = transaction.query_opt(&query, &params).await?;

        transaction.commit().await?;

        Ok(row)
    }

    pub async fn query_delete(
        &self,
        query: String,
        params: Vec<&(dyn ToSql + Sync)>,
    ) -> Result<bool, Error> {
        let mut conn = self.pool.get().await?;
        let transaction = conn.transaction().await?;

        let res = transaction.execute(&query, &params).await? != 0;

        if !res {
            return Ok(false);
        }

        transaction.commit().await?;

        Ok(true)
    }

    pub async fn query_transaction(
        &self,
        queries: Vec<(String, Vec<&(dyn ToSql + Sync)>)>,
    ) -> Result<(), Error> {
        let mut conn = self.pool.get().await?;
        let transaction = conn.transaction().await?;

        for (query, params) in queries {
            transaction.query_opt(&query, &params).await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}
