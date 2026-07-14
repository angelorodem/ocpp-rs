//! `MessageFormatEnumType`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageFormatEnumType {
    #[serde(rename = "ASCII")]
    ASCII,
    #[serde(rename = "HTML")]
    HTML,
    #[serde(rename = "URI")]
    URI,
    #[serde(rename = "UTF8")]
    UTF8,
    #[serde(rename = "QRCODE")]
    QRCODE,
}
