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
pub struct StructsPeriodLocationMessagePayload {
    #[serde(rename = "location")]
    pub location: Box<crate::models::StructsLocationMessagePayloadLocation>,
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl StructsPeriodLocationMessagePayload {
    pub fn new(location: crate::models::StructsLocationMessagePayloadLocation, to: String) -> StructsPeriodLocationMessagePayload {
        StructsPeriodLocationMessagePayload {
            location: Box::new(location),
            to,
            url: None,
        }
    }
}


