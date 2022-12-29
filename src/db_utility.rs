use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Album {
    #[serde(default)]
    id: i64,
    pub title: String,
}

pub async fn get_conn_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/hello_dev")
        .await?;
    Ok(pool)
}

pub async fn get_list_albums() -> Result<Vec<Album>, sqlx::Error> {
    let pool = get_conn_pool().await?;

    //sql_select_list
    let albums: Vec<Album> = sqlx::query_as::<_, Album>("SELECT * FROM albums_test")
        .fetch_all(&pool)
        .await?;

    Ok(albums)
}

pub async fn get_total_albums() -> Result<i64, sqlx::Error> {
    let pool = get_conn_pool().await?;

    //sql_count
    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM albums_test")
        .fetch_one(&pool)
        .await?;

    Ok(total)
}

pub async fn insert_album(title: &str) -> Result<i64, sqlx::Error> {
    let pool1 = get_conn_pool().await;

    match pool1 {
        Ok(pool) => {
            //sql_create  
            let row = sqlx::query_scalar::<_, i64>(
                "insert into albums_test (title) values ($1) returning id",
            )
            .bind(title)
            .fetch_one(&pool)
            .await;
            return row;
        }
        Err(e) => return Err(e),
    }
}

pub async fn delete_album(id: i64) -> Result<(), sqlx::Error> {
    let pool1 = get_conn_pool().await;

    match pool1 {
        Ok(pool) => {
            let _ = sqlx::query_scalar::<_, ()>("delete from albums_test where id = ($1)")
                .bind(id)
                .fetch_one(&pool)
                .await;
            return Ok(());
        }
        Err(e) => return Err(e),
    }
}

pub async fn update_album(id: i64, title: &str) -> Result<(), sqlx::Error> {
    let pool1 = get_conn_pool().await;

    match pool1 {
        Ok(pool) => {
            let _ =
                //sql_update  
                sqlx::query_scalar::<_, ()>("update albums_test set title = ($2) where id = ($1)")
                    .bind(id)
                    .bind(title)
                    .fetch_one(&pool)
                    .await;
            return Ok(());
        }
        Err(e) => return Err(e),
    }
}
