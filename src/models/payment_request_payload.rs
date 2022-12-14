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
pub struct PaymentRequestPayload {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "expiry_minutes", skip_serializing_if = "Option::is_none")]
    pub expiry_minutes: Option<i32>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl PaymentRequestPayload {
    pub fn new() -> PaymentRequestPayload {
        PaymentRequestPayload {
            amount: None,
            currency: None,
            expiry_minutes: None,
            to: None,
        }
    }
}


