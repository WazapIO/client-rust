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
pub struct PollMessagePayload {
    #[serde(rename = "options")]
    pub options: Vec<String>,
    #[serde(rename = "selectable_options_count")]
    pub selectable_options_count: i32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "to")]
    pub to: String,
}

impl PollMessagePayload {
    pub fn new(options: Vec<String>, selectable_options_count: i32, title: String, to: String) -> PollMessagePayload {
        PollMessagePayload {
            options,
            selectable_options_count,
            title,
            to,
        }
    }
}

