// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

mod connecting_objects;
mod flow_objects;
mod process_xml_parser;
mod sequence_flow_xml_parser;

use log::debug;
use process_xml_parser::parse_process_element;
use quick_xml::events::Event as XmlEvent;
use quick_xml::Reader;

use std::fs::File;
use std::io::{BufReader, Cursor};

use super::errors::BPMNParseError;
use super::process::Process;

/// Parse BPMN XML file
pub fn parse_bpmn_from_file(file_path: &str) -> Result<Process, BPMNParseError> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);
    parse_bpmn(Reader::from_reader(reader))
}

/// Parse BPMN XML string
pub fn parse_bpmn_from_str(xml_content: &str) -> Result<Process, BPMNParseError> {
    let reader = Cursor::new(xml_content);
    parse_bpmn(Reader::from_reader(reader))
}

fn parse_bpmn(mut xml_reader: Reader<impl std::io::BufRead>) -> Result<Process, BPMNParseError> {
    xml_reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut process = Process::new();

    loop {
        match xml_reader.read_event_into(&mut buf) {
            // Start tag (with attributes) `<tag attr="value">`.
            Ok(XmlEvent::Start(ref element)) => {
                debug!(
                    "Parsing element: <{}>",
                    std::str::from_utf8(element.name().as_ref())?
                );
                if let Ok(tag_name) = std::str::from_utf8(element.name().as_ref()) {
                    match tag_name {
                        "definitions" => {
                            debug!("Skipping <definitions> element");
                            // For now, we can just skip the <definitions> element
                            // It may contain additional attributes like namespace declarations
                            continue;
                        }
                        "process" => parse_process_element(&mut process, &mut xml_reader, element)?,
                        _ => Err(BPMNParseError::UnsupportedElement(tag_name.to_string()))?,
                    }
                }
            }
            Err(e) => Err(BPMNParseError::XmlParseError(format!(
                "Error at position {}: {:?}",
                xml_reader.error_position(),
                e
            )))?,
            Ok(XmlEvent::Eof) => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(process)
}
