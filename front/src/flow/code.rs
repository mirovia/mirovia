use crate::flow::format::format_flow_as_yaml;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MiroviaFlow {
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
pub fn flow_code(x: &str) -> MiroviaFlow {
    let flow_code: MiroviaFlow =
        serde_yaml::from_str::<MiroviaFlow>(&format_flow_as_yaml(x)).unwrap();
    return flow_code;
}
