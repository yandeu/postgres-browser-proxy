use r2d2_postgres::postgres::NoTls;

pub type PgClient = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>>;
