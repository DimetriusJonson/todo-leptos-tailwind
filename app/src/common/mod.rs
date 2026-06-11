pub mod api_error;
pub mod app_state;
pub mod security_context;
pub mod validate_helper;

#[cfg(feature = "ssr")]
use sqlx::Pool;

#[cfg(feature = "ssr")]
pub type DbPool = Pool<sqlx::Sqlite>;
//pub type DbPool = Pool<sqlx::Postgres>;
