/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "url")]
    pub url: String,
    /// A unique identifier for the organization.
    #[serde(rename = "id")]
    pub id: String,
    /// The organization name.
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates if this Organization is the one currently targeted by the Bearer token used by the client to authorize.
    #[serde(rename = "current")]
    pub current: bool,
    #[serde(rename = "subscription_expires_at")]
    pub subscription_expires_at: Option<String>,
    #[serde(rename = "subscription_id")]
    pub subscription_id: String,
    #[serde(rename = "subscription_plan_id")]
    pub subscription_plan_id: String,
    #[serde(rename = "subscription_plan_name")]
    pub subscription_plan_name: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
}

impl Organization {
    pub fn new(
        url: String,
        id: String,
        name: String,
        current: bool,
        subscription_expires_at: Option<String>,
        subscription_id: String,
        subscription_plan_id: String,
        subscription_plan_name: String,
        created_at: String,
        modified_at: String,
    ) -> Organization {
        Organization {
            url,
            id,
            name,
            current,
            subscription_expires_at,
            subscription_id,
            subscription_plan_id,
            subscription_plan_name,
            created_at,
            modified_at,
        }
    }
}
