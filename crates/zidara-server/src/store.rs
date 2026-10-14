//! Database access for the app role. Every transaction starts by saying who is acting.

use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

/// Begins a transaction acting as `user`. The setting is `SET LOCAL`, so it ends with the
/// transaction and a pooled connection cannot carry it into the next request.
pub async fn begin_as(
    pool: &PgPool,
    user: Uuid,
) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("select set_config('app.user_id', $1, true)")
        .bind(user.to_string())
        .execute(&mut *tx)
        .await?;
    Ok(tx)
}
