use crate::flow::format::format_flow_as_yaml;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GouttelettesFlow {
    pub display: BTreeMap<String, DisplayDefinition>,
    pub function: BTreeMap<String, FunctionDefinition>,
    pub display_input: Option<BTreeMap<String, DisplayInputDefinition>> ,
    graph: Vec<String>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayDefinition {
    pub title: BTreeMap<Language, String>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayInputDefinition {
    pub title: BTreeMap<Language, String>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub output: BTreeMap<Language, String>,
}
type Language = String;
pub fn flow_code(x: &str) -> GouttelettesFlow {
    let flow_code: GouttelettesFlow =
        serde_yaml::from_str::<GouttelettesFlow>(&format_flow_as_yaml(x)).unwrap();
    return flow_code;
}
