// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

// #[cfg(test)]
// mod tests {
//     use crate::bpmn::model::model_xml_parser::parse_bpmn_from_str;

//     use super::*;
//     use std::fs::File;
//     use std::io::Read;

//     #[test]
//     fn test_execute_simple_process() {
//         // Load the BPMN file
//         let mut file = File::open("../resources/tests/simple_process_v1.bpmn")
//             .expect("File not found");

//         let mut xml_content = String::new();
//         file.read_to_string(&mut xml_content).expect("Error reading file");

//         // Parse the BPMN XML
//         let process = parse_bpmn_from_str(&xml_content);

//         // Execute the parsed tasks
//         execute_tasks(process);

//         // Assert expected outcomes of the process execution
//         // For now, just checking that execution doesn't fail
//     }
// }
