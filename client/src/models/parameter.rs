/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

/// Parameter : A single parameter inside of a project.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "url")]
    pub url: String,
    /// A unique identifier for the parameter.
    #[serde(rename = "id")]
    pub id: String,
    /// The parameter name.
    #[serde(rename = "name")]
    pub name: String,
    /// A description of the parameter.  You may find it helpful to document how this parameter is used to assist others when they need to maintain software that uses this content.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates if this content is secret or not.  External values are inspected on-demand to ensure they align with the parameter's secret setting and if they do not, those external values are not allowed to be used.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<bool>,
    /// The type of this Parameter.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Rules applied to this parameter.
    #[serde(rename = "rules")]
    pub rules: Vec<crate::models::ParameterRule>,
    /// The project that the parameter is within.
    #[serde(rename = "project")]
    pub project: String,
    /// The project name that the parameter is within.
    #[serde(rename = "project_name")]
    pub project_name: String,
    /// Templates that reference this Parameter.
    #[serde(rename = "referencing_templates")]
    pub referencing_templates: Vec<String>,
    /// Dynamic values that reference this Parameter.
    #[serde(rename = "referencing_values")]
    pub referencing_values: Vec<String>,
    ///              This dictionary has keys that correspond to environment urls, and values             that correspond to the effective value for this parameter in that environment.             Each parameter has an effective value in every environment based on             project dependencies and environment inheritance.              The effective value is found by looking (within the keyed environment) up             the project dependencies by parameter name.  If a value is not found, the             parent environment is consulted with the same logic to locate a value.  It             is possible for there to be a `null` value record for an environment, which             means there is no value set; it is also possible for there to be a value record             with a `value` of `null`, which means the value was explicitly set to `null`.              If the value's parameter does not match the enclosing parameter (holding the             values array) then that value is flowing in through project dependencies.             Clients must recognize this in case the user asks to modify the value; in this             case the client must POST a new Value to the current parameter to override the             value coming in from the project dependency.              If the Value.environment matches the key, then it is an explicit value set for             that environment.  If they differ, the value was obtained from a parent             environment (directly or indirectly).  If the value is None then no value has             ever been set in any environment for this parameter within all the project             dependencies.         
    #[serde(rename = "values")]
    pub values: ::std::collections::HashMap<String, Option<crate::models::Value>>,
    /// If this parameter's project depends on another project which provides a parameter of the same name, this parameter overrides the one provided by the dependee.  You can use this field to determine if there will be side-effects the user should know about when deleting a parameter.  Deleting a parameter that overrides another one due to an identical name will uncover the one from the dependee project.
    #[serde(rename = "overrides")]
    pub overrides: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
}

impl Parameter {
    /// A single parameter inside of a project.
    pub fn new(
        url: String,
        id: String,
        name: String,
        rules: Vec<crate::models::ParameterRule>,
        project: String,
        project_name: String,
        referencing_templates: Vec<String>,
        referencing_values: Vec<String>,
        values: ::std::collections::HashMap<String, Option<crate::models::Value>>,
        overrides: Option<String>,
        created_at: String,
        modified_at: String,
    ) -> Parameter {
        Parameter {
            url,
            id,
            name,
            description: None,
            secret: None,
            _type: None,
            rules,
            project,
            project_name,
            referencing_templates,
            referencing_values,
            values,
            overrides,
            created_at,
            modified_at,
        }
    }
}
