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
pub struct StructsPeriodButtonMessageWithMediaPayload {
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<crate::models::StructsPeriodReplyButton>>,
    #[serde(rename = "footer_text", skip_serializing_if = "Option::is_none")]
    pub footer_text: Option<String>,
    #[serde(rename = "media_data", skip_serializing_if = "Option::is_none")]
    pub media_data: Option<Box<crate::models::StructsPeriodFileUpload>>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl StructsPeriodButtonMessageWithMediaPayload {
    pub fn new() -> StructsPeriodButtonMessageWithMediaPayload {
        StructsPeriodButtonMessageWithMediaPayload {
            buttons: None,
            footer_text: None,
            media_data: None,
            text: None,
            to: None,
        }
    }
}


