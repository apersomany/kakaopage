use serde_derive::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct APIRequest<Req> {
    pub(crate) s_token: String,
    pub(crate) user_uid: u32,
    #[serde(flatten)]
    pub(crate) data: Req,
}
