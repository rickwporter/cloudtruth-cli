/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your parameters and secrets making them easier to manage and use.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvitationCreate {
    /// The email address of the user to be invited.
    #[serde(rename = "email")]
    pub email: String,
    /// The role that the user will have in the organization, should the user accept.
    #[serde(rename = "role")]
    pub role: Box<crate::models::RoleEnum>,
}

impl InvitationCreate {
    pub fn new(email: String, role: crate::models::RoleEnum) -> InvitationCreate {
        InvitationCreate {
            email,
            role: Box::new(role),
        }
    }
}
