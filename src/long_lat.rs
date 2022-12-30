use std::{
    error::Error,
    fmt,
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
}

impl fmt::Display for LongLat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"long\":\"{}\",\"lat\":\"{}\"}}", self.long, self.lat)
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
        let is_be = byte_order == 0i8;

        let type_id = bytes_to_u32(&is_be, _u32);
        let mut srid: Option<i32> = None;

        if type_id & 0x20000000 == 0x20000000 {
            buff.read_exact(&mut _i32).unwrap();
            srid = Some(bytes_to_i32(&is_be, _i32));
        }

        buff.read_exact(&mut _long).unwrap();
        buff.read_exact(&mut _lat).unwrap();

        let long = bytes_to_f64(&is_be, _long);
        let lat = bytes_to_f64(&is_be, _lat);

        // println!("type_id {}", type_id);
        // println!("srid {}", srid.unwrap());
        // println!("long {}", long);
        // println!("lat {}", lat);

        // see:
        // - https://mariadb.com/kb/en/well-known-binary-wkb-format/
        // - https://postgis.net/docs/manual-dev/using_postgis_dbmanagement.html#OpenGISWKBWKT
        // - https://libgeos.org/specifications/wkb/
        let geo = match type_id & 0xff {
            0x01 => LongLat::new(long, lat, srid),
            _ => todo!(),
        };

        Ok(geo)
    }

    fn accepts(ty: &Type) -> bool {
        matches!(ty.name(), "geography")
    }
}

fn bytes_to_u32(is_be: &bool, bytes: [u8; 4]) -> u32 {
    if *is_be {
        u32::from_be_bytes(bytes)
    } else {
        u32::from_le_bytes(bytes)
    }
}

fn bytes_to_i32(is_be: &bool, bytes: [u8; 4]) -> i32 {
    if *is_be {
        i32::from_be_bytes(bytes)
    } else {
        i32::from_le_bytes(bytes)
    }
}

fn bytes_to_f64(is_be: &bool, bytes: [u8; 8]) -> f64 {
    if *is_be {
        f64::from_be_bytes(bytes)
    } else {
        f64::from_le_bytes(bytes)
    }
}
