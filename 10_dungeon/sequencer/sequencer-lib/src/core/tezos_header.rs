use serde::Deserialize;

#[derive(Deserialize)]
pub struct TezosHeader {
    pub hash: String,
    pub level: u32,
    pub predecessor: String,
}
