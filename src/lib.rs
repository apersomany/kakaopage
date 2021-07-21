use std::time::SystemTime;

pub const API_URL: &str = "https://api2-page.kakao.com";
pub const IMG_URL: &str = "http://dn-img-page.kakao.com";

pub mod api {
    pub mod inven {
        pub mod download_data;
        pub mod my_serieses;
        pub mod my_singles;
    }

    pub mod store {
        pub mod claim_gift;
        pub mod gift_list;
        pub mod home;
        pub mod single;
        pub mod singles;
        pub mod use_ticket;
        pub mod view_ticket;
    }
    pub mod search;
}

pub mod deserializer {
    pub mod yn_bool;
}

pub mod enums {
    pub mod category;
    pub mod direction;
}

pub mod models {
    pub mod account;
    pub mod file_id;
    pub mod gift_id;
    pub mod request;
    pub mod response;
    pub mod series_id;
    pub mod single_id;
    pub mod tokens;
}

pub mod client;

pub mod util {
    pub mod derive_key;
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
