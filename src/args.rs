use clap::Parser;

#[derive(Parser, Clone)]
#[command(version)]
pub struct Args {
    #[arg(long, default_value_t = String::from("localhost"))]
    host: String,
    #[arg(long, default_value_t = String::from("postgres"))]
    user: String,
    #[arg(long, default_value_t = String::from("mysecretpassword"))]
    password: String,
    #[arg(long, default_value_t = String::from("3000"))]
    port: String,
    #[arg(long, default_value_t = String::from("5432"))]
    pg_port: String,
}
impl Args {
    pub fn port(&self) -> &str {
        &self.port
    }
    pub fn to_db_string(&self) -> String {
        format!(
            "host={} user={} password={}",
            self.host, self.user, self.password
        )
    }
}
