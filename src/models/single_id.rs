use crate::{models::account::Account, util::derive_key::derive_key};
use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use serde_derive::*;

type AES256CBC = Cbc<Aes256, Pkcs7>;

const PASSWORD: [u8; 16] = [
    45, 21, 27, 37, 48, 66, 35, 56, 44, 33, 50, 91, 36, 93, 55, 85,
];
const IV: [u8; 16] = [
    1, 10, 78, 62, 238, 178, 244, 92, 31, 22, 12, 55, 229, 27, 60, 22,
];

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SingleId(pub u32);

impl SingleId {
    pub fn decrypt_aid(&self, account: &Account, aid: &str) -> String {
        let salt = [
            account.id.to_string(),
            account.device_id.clone(),
            self.0.to_string(),
        ]
        .join("");
        let key = derive_key(&PASSWORD, salt.as_bytes());
        let aid = aid.replace("-", "+").replace("_", "/");
        let cipher = AES256CBC::new_from_slices(&key, &IV).unwrap();
        let ctext = base64::decode(aid).unwrap();
        let ptext = cipher.decrypt_vec(&ctext).unwrap();
        String::from_utf8(ptext).unwrap()
    }
}
