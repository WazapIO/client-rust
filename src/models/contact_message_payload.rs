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
pub struct ContactMessagePayload {
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "vcard")]
    pub vcard: Box<crate::models::ContactMessagePayloadVcard>,
}

impl ContactMessagePayload {
    pub fn new(to: String, vcard: crate::models::ContactMessagePayloadVcard) -> ContactMessagePayload {
        ContactMessagePayload {
            to,
            vcard: Box::new(vcard),
        }
    }
}


