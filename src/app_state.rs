use crate::db_connection::{init_pool, PgPool};
use slog::Logger;

pub struct AppState {
    pub logger: Logger,
    pub pool: PgPool,
}
