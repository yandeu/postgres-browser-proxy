use postgis::{ewkb, LineString};
use std::{error::Error, fmt::format, io::Read};

use chrono::{DateTime, Local};
use postgres::{
    types::{private::BytesMut, FromSql, IsNull, ToSql, Type},
    Row,
};

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
impl Point {
    pub fn from_ewkb_point(point: ewkb::Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}{}{}{}{}", "{\"x\":", self.x, ",\"y\":", self.y, "}")
        // format!("\{x:{},y:{}}", self.x, self.y)
    }
}

fn quotes<T: Into<String>>(str: T) -> String {
    format!("{}{}{}", "\"", str.into(), "\"")
}

fn parse_row<'a, T>(row: &'a Row, index: usize, wrap_into_quotes: bool) -> String
where
    T: postgres::types::FromSql<'a> + std::fmt::Display,
{
    match row.get::<usize, Option<T>>(index) {
        Some(a) => {
            if !wrap_into_quotes {
                a.to_string()
            } else {
                quotes(a.to_string())
            }
        }
        None => "null".to_string(),
    }
}

pub fn row_to_string(data: Vec<Row>) -> String {
    let mut json = String::from("[");

    for (i, row) in data.iter().enumerate() {
        json.push('{');
        let len = row.columns().len() - 1;
        for (j, col) in row.columns().iter().enumerate() {
            let key = quotes(col.name());
            let value: String = match *col.type_() {
                Type::DATE => parse_row::<chrono::NaiveDate>(row, j, true),
                Type::TIME => parse_row::<chrono::NaiveTime>(row, j, true),
                Type::TIMESTAMP => parse_row::<chrono::NaiveDateTime>(row, j, true),
                Type::TIMESTAMPTZ => parse_row::<DateTime<Local>>(row, j, true),
                Type::CHAR | Type::VARCHAR | Type::TEXT | Type::NAME => {
                    parse_row::<String>(row, j, true)
                }
                Type::INT4 => parse_row::<i32>(row, j, false),
                Type::BOOL => parse_row::<bool>(row, j, false),
                Type::FLOAT8 => parse_row::<f64>(row, j, false),
                _ => {
                    match col.type_().name() {
                        "geography" => {
                            let route = row.try_get::<_, Option<ewkb::Point>>(j);
                            let point: Point = match route {
                                Ok(Some(geom)) => {
                                    println!("{:?}", geom);
                                    Point::from_ewkb_point(geom)
                                }
                                _ => Point { x: 0f64, y: 0f64 }, // Ok(None) => { /* Handle NULL value */ }
                                                                 // Err(err) => {
                                                                 //     println!("Error: {}", err)
                                                                 // }
                            };

                            point.to_string()

                            //  parse_row::<Point>(row, j, false);
                        }
                        n => {
                            let r#type = format!("{}", *col.type_());
                            quotes(format!("type '{}' is unknown", r#type))
                        }
                    }
                }
            };
            json.push_str(&format!("{}:{}", key, value));
            if j < len {
                json.push(',');
            }
        }
        json.push('}');
        if i != data.len() - 1 {
            json.push(',');
        }
    }

    json.push(']');
    json
}
