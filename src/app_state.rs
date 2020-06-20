use crate::db_connection::PgPool;
use slog::Logger;

pub struct AppState {
    pub logger: Logger,
    pub pool: PgPool,
}
