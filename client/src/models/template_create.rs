/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// TemplateCreate : A parameter template in a given project, optionally instantiated against an environment.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TemplateCreate {
    /// The template name.
    #[serde(rename = "name")]
    pub name: String,
    /// ('A description of the template.  You may find it helpful to document how this template is used to assist others when they need to maintain software that uses this content.',)
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The content of the template.  Use mustache-style templating delimiters of `{{` and `}}` to reference parameter values by name for substitution into the template result.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl TemplateCreate {
    /// A parameter template in a given project, optionally instantiated against an environment.
    pub fn new(name: String) -> TemplateCreate {
        TemplateCreate {
            name,
            description: None,
            body: None,
        }
    }
}
