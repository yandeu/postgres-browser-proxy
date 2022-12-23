use crate::types;
use postgres::Row;

pub fn make_query<T: Into<String>>(
    query: T,
    client: &mut types::PgClient,
) -> Result<Vec<Row>, String> {
    let data = match client.query(&query.into(), &[]) {
        Ok(data) => data,
        Err(e) => {
            let mut error = e.to_string();
            error = error.replace("db error: ERROR: ", "");
            println!("ERROR: {}", error);
            return Err(error);
        }
    };

    Ok(data)
}
