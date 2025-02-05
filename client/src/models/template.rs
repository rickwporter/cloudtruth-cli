/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// Template : A parameter template in a given project, optionally instantiated against an environment.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Template {
    /// The templates this value references, if interpolated.
    #[serde(rename = "url")]
    pub url: String,
    /// A unique identifier for the template.
    #[serde(rename = "id")]
    pub id: String,
    /// The template name.
    #[serde(rename = "name")]
    pub name: String,
    /// ('A description of the template.  You may find it helpful to document how this template is used to assist others when they need to maintain software that uses this content.',)
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, the `body` field has undergone evaluation.
    #[serde(rename = "evaluated")]
    pub evaluated: bool,
    /// The content of the template.  Use mustache-style templating delimiters of `{{` and `}}` to reference parameter values by name for substitution into the template result.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Projects (other than this template's project) that this template referenced
    #[serde(rename = "referenced_projects")]
    pub referenced_projects: Vec<String>,
    /// Parameters that this template references.
    #[serde(rename = "referenced_parameters")]
    pub referenced_parameters: Vec<String>,
    /// Other templates that this template references.
    #[serde(rename = "referenced_templates")]
    pub referenced_templates: Vec<String>,
    /// Other templates that reference this template.
    #[serde(rename = "referencing_templates")]
    pub referencing_templates: Vec<String>,
    /// The dynamic values that reference this template.
    #[serde(rename = "referencing_values")]
    pub referencing_values: Vec<String>,
    /// If True, this template contains secrets.
    #[serde(rename = "has_secret")]
    pub has_secret: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
}

impl Template {
    /// A parameter template in a given project, optionally instantiated against an environment.
    pub fn new(
        url: String,
        id: String,
        name: String,
        evaluated: bool,
        referenced_projects: Vec<String>,
        referenced_parameters: Vec<String>,
        referenced_templates: Vec<String>,
        referencing_templates: Vec<String>,
        referencing_values: Vec<String>,
        has_secret: bool,
        created_at: String,
        modified_at: String,
    ) -> Template {
        Template {
            url,
            id,
            name,
            description: None,
            evaluated,
            body: None,
            referenced_projects,
            referenced_parameters,
            referenced_templates,
            referencing_templates,
            referencing_values,
            has_secret,
            created_at,
            modified_at,
        }
    }
}
