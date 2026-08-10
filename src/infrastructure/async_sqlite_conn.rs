use diesel::SqliteConnection;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;

pub type AsyncSqliteConnection = SyncConnectionWrapper<SqliteConnection>;
