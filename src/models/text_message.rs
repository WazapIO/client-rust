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
pub struct TextMessage {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "to")]
    pub to: String,
}

impl TextMessage {
    pub fn new(text: String, to: String) -> TextMessage {
        TextMessage {
            text,
            to,
        }
    }
}


