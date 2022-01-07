/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "url")]
    pub url: String,
    /// The unique identifier of a user.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of user record.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The user's organization name.
    #[serde(rename = "organization_name")]
    pub organization_name: Option<String>,
    /// Membership identifier for user.
    #[serde(rename = "membership_id")]
    pub membership_id: Option<String>,
    /// The user's role in the current organization (defined by the request authorization header).
    #[serde(rename = "role")]
    pub role: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "picture_url")]
    pub picture_url: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
}

impl User {
    pub fn new(
        url: String,
        id: String,
        name: Option<String>,
        organization_name: Option<String>,
        membership_id: Option<String>,
        role: Option<String>,
        email: Option<String>,
        picture_url: Option<String>,
        created_at: String,
        modified_at: String,
    ) -> User {
        User {
            url,
            id,
            _type: None,
            name,
            organization_name,
            membership_id,
            role,
            email,
            picture_url,
            created_at,
            modified_at,
        }
    }
}
