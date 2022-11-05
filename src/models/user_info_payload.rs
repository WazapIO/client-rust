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
pub struct UserInfoPayload {
    #[serde(rename = "user_ids")]
    pub user_ids: Vec<String>,
}

impl UserInfoPayload {
    pub fn new(user_ids: Vec<String>) -> UserInfoPayload {
        UserInfoPayload {
            user_ids,
        }
    }
}

