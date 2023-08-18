use anyhow::Result;
use dotenv::dotenv;
use pg_listener::{start_listening, start_trigger};
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() {
  dotenv().ok();
  let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
  let pool = sqlx::PgPool::connect(&db_url).await.unwrap();

  let channels = vec!["nfts_change"];

  start_trigger(&pool).await.unwrap();

  start_listening(&pool, channels, update_nfts_poistion)
    .await
    .unwrap();
}

async fn update_nfts_poistion<'a>(
  payload: serde_json::Value,
  pool: &'a Pool<Postgres>,
) -> Result<()> {
  dbg!(payload);
  sqlx::query!(
    r#"
      WITH nft_ext AS (
        SELECT
        "nft"."id",
        row_number() OVER (
          ORDER BY "nft"."square_price" DESC NULLS LAST
        ) AS "index"
        FROM "nft"
        WHERE "nft"."is_active" = true
      )
      UPDATE nft
      SET
        "last_position" = "position",
        "position" = (
          SELECT "nft_ext"."index"
          FROM "nft_ext"
          WHERE "nft"."id" = "nft_ext"."id"
          LIMIT 1
        )
      WHERE is_active = true
  "#
  )
  .execute(pool)
  .await?;
  Ok(())
}