use crate::flow::code::MiroviaFlow;
use std::collections::HashSet;
use std::iter::repeat;
use crate::log;
#[cfg(test)]
use std::fs::read_to_string;
#[cfg(test)]
use std::fs::write;
fn format_flow(x: &str) -> String {
    let mut reset_indent = HashSet::new();
    reset_indent.insert("display");
    reset_indent.insert("function");
    reset_indent.insert("graph");
    reset_indent.insert("display_input");
    let mut languages = HashSet::new();
    languages.insert("english");
    languages.insert("french");
    let mut func_in_out = HashSet::new();
    func_in_out.insert("input");
    func_in_out.insert("output");
    let mut section = "";
    let mut indent = 0;
    let indent_size = 3;
    let flat = flatten(x);
    let lines = flat.lines();
    let mut new_lines = vec![];
    for line in lines.into_iter() {
        if reset_indent.contains(line) {
            indent = 0;
            section = line;
        } else if section == "graph" {
            indent = 1;
        } else if section == "display" && languages.contains(line) {
            indent = 3;
        } else if section == "display_input" && languages.contains(line) {
            indent = 3;
        } else if section == "function" && languages.contains(line) {
            indent = 3;
        } else if section == "function" && func_in_out.contains(line) {
            indent = 2;
        } else {
            indent += 1;
        }
        new_lines.push(format!(
            "{}{}",
            repeat(' ').take(indent * indent_size).collect::<String>(),
            line
        ));
    }
    return new_lines
        .into_iter()
        .fold(String::new(), |a, b| a + &b + "\n");
}
fn flatten(x: &str) -> String {
    return x
        .lines()
        .map(|x| x.trim_start())
        .fold(String::new(), |a, b| a + b + "\n");
}
pub fn format_flow_as_yaml(x: &str) -> String {
    let x_formatted = format_flow(x);
    let lines: Vec<&str> = x_formatted.lines().collect();
    let mut new_lines = vec![];
    let mut sections = HashSet::new();
    sections.insert("display");
    sections.insert("function");
    sections.insert("graph");
    sections.insert("display_input");
    let mut languages = HashSet::new();
    languages.insert("english");
    languages.insert("french");
    let mut section = "";
    let mut i = 0;
    let indent_size = 3;
    while i < lines.len() {
        let line = lines[i];
        let previous_line = if i == 0 { "#start" } else { lines[i - 1] };
        if sections.contains(line) {
            section = line;
        }
        if section == "graph" && line != "graph" {
            new_lines.push(format!(
                "{}- {}",
                repeat(' ').take(indent_size).collect::<String>(),
                &line.trim_start()
            ));
        } else if languages.contains(previous_line.trim_start()) {
            new_lines.push(format!("{}", &line));
        } else {
            new_lines.push(format!("{}:", &line));
        }
        i += 1;
    }
    let first_pass: String = new_lines
        .into_iter()
        .fold(String::new(), |a, b| a + &b + "\n");
    let code: MiroviaFlow = match serde_yaml::from_str::<MiroviaFlow>(&first_pass) {
        Ok(x) => x,
        Err(e) => {
            log(&format!("first_pass: {}", &first_pass));
            panic!("error: {}", e)
        }
    };
    let second_pass = serde_yaml::to_string(&code).unwrap();
    return second_pass;
}
#[test]
fn test_format_flow() {
    let examples = vec!["000_hello_world", "001_hello_someone"];
    for example in examples {
        let content = read_to_string(format!("static/examples/{}.gf", example))
            .expect("Something went wrong reading the file");
        let content_flat = read_to_string(format!("examples_flat/{}.gf", example))
            .expect("Something went wrong reading the file");
        write(
            format!("examples_test/{}.test.gf", example),
            format_flow(&content_flat),
        )
        .unwrap();
        assert_eq!(format_flow(&content_flat), content);
    }
}
#[test]
fn test_flatten() {
    let examples = vec!["000_hello_world", "001_hello_someone"];
    for example in examples {
        let content = read_to_string(format!("static/examples/{}.gf", example))
            .expect("Something went wrong reading the file");
        let content_flat = read_to_string(format!("examples_flat/{}.gf", example))
            .expect("Something went wrong reading the file");
        write(
            format!("examples_flat/{}.gf.test", example),
            flatten(&content),
        )
        .unwrap();
        assert_eq!(flatten(&content), content_flat);
    }
}
#[test]
fn test_format_flow_as_yaml() {
    let examples = vec!["000_hello_world", "001_hello_someone"];
    for example in examples {
        let content = read_to_string(format!("static/examples/{}.gf", example))
            .expect("Something went wrong reading the file");
        let content_yaml = read_to_string(format!("examples_yaml/{}.yaml", example))
            .expect("Something went wrong reading the file");
        write(
            format!("examples_yaml/{}.yaml.test", example),
            format_flow_as_yaml(&content),
        )
        .unwrap();
        assert_eq!(format_flow_as_yaml(&content), content_yaml);
    }
}
