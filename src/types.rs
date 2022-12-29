use postgis::ewkb;
use r2d2_postgres::postgres::NoTls;

pub type PgClient = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager<NoTls>>;

#[derive(Debug)]
pub struct LongLat {
    pub long: f64,
    pub lat: f64,
}
impl LongLat {
    pub fn from_ewkb_point(point: ewkb::Point) -> Self {
        Self {
            long: point.x,
            lat: point.y,
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}",
            "{\"long\":", self.long, ",\"lat\":", self.lat, "}"
        )
        // format!("\{x:{},y:{}}", self.x, self.y)
    }
}
