/*
 * WhatsAPI Go
 *
 * The V2 of WhatsAPI Go
 *
 * The version of the OpenAPI document: 2.0
 * Contact: manjit@sponsorbook.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StructsPeriodSendMediaPayload {
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "media_data")]
    pub media_data: Box<crate::models::StructsPeriodFileUpload>,
    #[serde(rename = "media_type")]
    pub media_type: String,
    #[serde(rename = "to")]
    pub to: String,
}

impl StructsPeriodSendMediaPayload {
    pub fn new(media_data: crate::models::StructsPeriodFileUpload, media_type: String, to: String) -> StructsPeriodSendMediaPayload {
        StructsPeriodSendMediaPayload {
            caption: None,
            filename: None,
            media_data: Box::new(media_data),
            media_type,
            to,
        }
    }
}


