/*
 * CloudTruth Management API
 *
 * CloudTruth centralizes your configuration parameters and secrets making them easier to manage and use as a team.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@cloudtruth.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParameterTimelineEntryEnvironment {
    /// A unique identifier for the environment.
    #[serde(rename = "id")]
    pub id: String,
    /// The environment name.
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates if the value change was direct or if it flowed into the environment. If `true` then the value was actually set directly into this environment. If `false` then the environment has no value set directly so it inherited the value from its parent.
    #[serde(rename = "override")]
    pub _override: bool,
}

impl ParameterTimelineEntryEnvironment {
    pub fn new(id: String, name: String, _override: bool) -> ParameterTimelineEntryEnvironment {
        ParameterTimelineEntryEnvironment {
            id,
            name,
            _override,
        }
    }
}
