use std::env;

pub const SESSION_DURATION_DAYS: i64 = 7;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub session_secret: String,
    pub aws_region: String,
    pub aws_s3_bucket: String,
    pub frontend_url: String,
    pub is_production: bool,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            session_secret: env::var("SESSION_SECRET").expect("SESSION_SECRET must be set"),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "ap-northeast-1".to_string()),
            aws_s3_bucket: env::var("AWS_S3_BUCKET").unwrap_or_else(|_| "my-hp-images".to_string()),
            frontend_url: env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string()),
            is_production: env::var("IS_PRODUCTION").unwrap_or_else(|_| "false".to_string()) == "true",
        }
    }
}
