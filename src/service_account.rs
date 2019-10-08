use crate::resources::service_account;
use lazy_static::lazy_static;

fn get_service_account() -> service_account::ServiceAccount {
    dotenv::dotenv().ok();
    let path =
        dbg!(std::env::var("SERVICE_ACCOUNT").expect("SERVICE_ACCOUNT environment parameter required"));
    let file = std::fs::read_to_string(path).expect("SERVICE_ACCOUNT file not found");
    let account: service_account::ServiceAccount =
        serde_json::from_str(&file).expect("serivce account file not valid");
    if account._type != "service_account" {
        panic!("`type` paramter of `SERVICE_ACCOUNT` variable is not 'service_account'");
    }
    account
}

lazy_static! {
    pub static ref SERVICE_ACCOUNT: service_account::ServiceAccount = get_service_account();
}
