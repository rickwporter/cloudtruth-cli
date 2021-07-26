/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// PatchedTemplate : A parameter template in a given project, optionally instantiated against an environment.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedTemplate {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// A unique identifier for the template.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The template name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A description of the template.  You may find it helpful to document how this template is used to assist others when they need to maintain software that uses this content.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The content of the template.  Use mustache-style templating delimiters of `{{` and `}}` to reference parameter values by name for substitution into the template result.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    /// If True, this template contains secrets.
    #[serde(rename = "has_secret", skip_serializing_if = "Option::is_none")]
    pub has_secret: Option<bool>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

impl PatchedTemplate {
    /// A parameter template in a given project, optionally instantiated against an environment.
    pub fn new() -> PatchedTemplate {
        PatchedTemplate {
            url: None,
            id: None,
            name: None,
            description: None,
            body: None,
            parameters: None,
            has_secret: None,
            created_at: None,
            modified_at: None,
        }
    }
}
