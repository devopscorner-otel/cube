/*
 * Cube.js
 *
 * Cube.js Swagger Schema
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1CubeMetaDimension {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "granularities", skip_serializing_if = "Option::is_none")]
    pub granularities: Option<Vec<crate::models::V1CubeMetaDimensionGranularity>>,
}

impl V1CubeMetaDimension {
    pub fn new(name: String, _type: String) -> V1CubeMetaDimension {
        V1CubeMetaDimension {
            name,
            description: None,
            _type,
            granularities: None,
        }
    }
}
