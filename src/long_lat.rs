use std::{
    error::Error,
    io::{Cursor, Read},
};

use postgres::types::{FromSql, Type};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LongLat {
    pub long: f64,
    pub lat: f64,
    pub srid: Option<i32>,
}
impl LongLat {
    pub fn new(long: f64, lat: f64, srid: Option<i32>) -> Self {
        Self { long, lat, srid }
    }
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}",
            "{\"long\":", self.long, ",\"lat\":", self.lat, "}"
        )
    }
}
impl<'a> FromSql<'a> for LongLat {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let mut buff = Cursor::new(raw);

        let mut _i8 = [0u8; 1]; // i8
        let mut _u32 = [0u8; 4]; // u32 - type (0x01 - 0x07)
        let mut _i32 = [0u8; 4]; // i32 - 4326
        let mut _long = [0u8; 8]; // f64 - long
        let mut _lat = [0u8; 8]; // f64 - lat

        buff.read_exact(&mut _i8).unwrap();
        buff.read_exact(&mut _u32).unwrap();

        let byte_order = i8::from_le_bytes(_i8);
        let _is_be = byte_order == 0i8;

        let type_id = u32::from_le_bytes(_u32);
        let mut srid: Option<i32> = None;

        if type_id & 0x20000000 == 0x20000000 {
            buff.read_exact(&mut _i32).unwrap();
            srid = Some(i32::from_le_bytes(_i32));
        }

        buff.read_exact(&mut _long).unwrap();
        buff.read_exact(&mut _lat).unwrap();

        let long = f64::from_le_bytes(_long);
        let lat = f64::from_le_bytes(_lat);

        // see: https://github.com/andelf/rust-postgis/blob/master/src/ewkb.rs#L1008
        let geo = match type_id & 0xff {
            0x01 => LongLat::new(long, lat, srid),
            _ => todo!(),
        };

        Ok(geo)
    }

    fn accepts(ty: &Type) -> bool {
        match ty.name() {
            "geography" => true,
            _ => false,
        }
    }
}
