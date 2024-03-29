use dotenv::dotenv;
use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PRODUCTION: bool = set_production();
    pub static ref DATABASE_URL: String = set_database();

    pub static ref RAZORPAY_TEST_KEY: String = set_razorpat_test_key();
    pub static ref RAZORPAY_LIVE_KEY: String = set_razorpat_live_key();
    
    pub static ref RAZORPAY_TEST_SECRET_KEY: String = set_razorpay_test_secret_key();
    pub static ref RAZORPAY_LIVE_SECRET_KEY: String = set_razorpay_live_secret_key();


    pub static ref MAILER_EMAIL: String = set_mailer_email();
    pub static ref MAILER_PASSWORD: String = set_mailer_password();
    pub static ref MAILER_HOST: String = set_mailer_host();
    pub static ref MAILER_PORT: String = set_mailer_port();

    pub static ref APP_SECRET: String = set_app_secret();


}

fn set_production() -> bool {
    dotenv().ok();
    match env::var("PRODUCTION") {
        Ok(value) => value.parse().expect("Failed to parse PRODUCTION as bool"),
        Err(_) => false, // Default value if PRODUCTION is not set
    }
}

fn set_database() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

fn set_razorpat_test_key() -> String {
    dotenv().ok();
    env::var("RAZORPAY_TEST_KEY").unwrap()
}

fn set_razorpat_live_key() -> String {
    dotenv().ok();
    env::var("RAZORPAY_LIVE_KEY").unwrap()
}


fn set_razorpay_test_secret_key() -> String {
    dotenv().ok();
    env::var("RAZORPAY_TEST_SECRET_KEY").unwrap()
}

fn set_razorpay_live_secret_key()-> String {
    dotenv().ok();
    env::var("RAZORPAY_LIVE_SECRET_KEY").unwrap()
}

fn set_mailer_host()-> String {
    dotenv().ok();
    env::var("MAILER_HOST").unwrap()
}

fn set_mailer_password()-> String {
    dotenv().ok();
    env::var("MAILER_PASSWORD").unwrap()
}

fn set_mailer_email()-> String {
    dotenv().ok();
    env::var("MAILER_EMAIL").unwrap()
}

fn set_mailer_port()-> String {
    dotenv().ok();
    env::var("MAILER_PORT").unwrap()
}

fn set_app_secret() -> String {
    dotenv().ok();
    env::var("APP_SECRET").unwrap()
}