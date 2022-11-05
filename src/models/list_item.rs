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
pub struct ListItem {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "row_id", skip_serializing_if = "Option::is_none")]
    pub row_id: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
}

impl ListItem {
    pub fn new(title: String) -> ListItem {
        ListItem {
            description: None,
            row_id: None,
            title,
        }
    }
}


