use deadpool_postgres::{Object, Transaction};
use tokio_postgres::{Row, types::ToSql};
use uuid::Uuid;

use crate::error::Error;

// use super::{DBDelete, DBInsert, DBModel, DBSelectAll, DBSelectOne, DBUpdate};

pub trait DBModel: From<Row> {}

pub trait DBQuery {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>);
}

pub trait DBSubquery {
    fn get_subquery<'a>(&'a self, uuid: &'a Uuid) -> (String, Vec<&'a (dyn ToSql + Sync)>);
}

pub trait DBInsert {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<Row, Error>;
}

pub trait DBSelectOne {
    async fn query(&self, conn: &Object) -> Result<Option<Row>, Error>;
}

pub trait DBSelectAll {
    async fn query(&self, conn: &Object) -> Result<Vec<Row>, Error>;
}

pub trait DBUpdate {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<Option<Row>, Error>;
}

pub trait DBDelete {
    async fn query<'a>(&self, transaction: &Transaction<'a>) -> Result<bool, Error>;
}

pub async fn insert<I: DBInsert>(conn: &mut Object, object: &I) -> Result<Uuid, Error> {
    let transaction = conn.transaction().await?;

    // TODO: move towards this
    let row = object.query(&transaction).await?;

    transaction.commit().await?;

    let id = row.get("id");

    Ok(id)
}

pub async fn select_one<S: DBSelectOne, M: DBModel>(
    conn: &Object,
    object: &S,
) -> Result<Option<M>, Error> {
    let row_opt = object.query(conn).await?;

    Ok(row_opt.map(|r| M::from(r)))
}

pub async fn update<U: DBUpdate, M: DBModel>(
    conn: &mut Object,
    object: &U,
) -> Result<Option<M>, Error> {
    let transaction = conn.transaction().await?;

    let row_opt = object.query(&transaction).await?;

    transaction.commit().await?;

    Ok(row_opt.map(|r| M::from(r)))
}

pub async fn delete<D: DBDelete>(conn: &mut Object, object: &D) -> Result<bool, Error> {
    let transaction = conn.transaction().await?;

    let suc = object.query(&transaction).await?;

    transaction.commit().await?;

    Ok(suc)
}

pub async fn select_all<Q: DBSelectAll, M: DBModel>(
    conn: &Object,
    object: &Q,
) -> Result<Vec<M>, Error> {
    let rows = object.query(conn).await?;

    Ok(rows.iter().map(|r| M::from(r.to_owned())).collect())
}
