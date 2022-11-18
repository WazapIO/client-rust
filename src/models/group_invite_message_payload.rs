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
pub struct GroupInviteMessagePayload {
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(rename = "expiry_minutes", skip_serializing_if = "Option::is_none")]
    pub expiry_minutes: Option<i32>,
    #[serde(rename = "invite_code", skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl GroupInviteMessagePayload {
    pub fn new() -> GroupInviteMessagePayload {
        GroupInviteMessagePayload {
            caption: None,
            expiry_minutes: None,
            invite_code: None,
            to: None,
        }
    }
}


