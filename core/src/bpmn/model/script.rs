// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::default;

use super::errors::EnumParseError;

/// Enum representing the type of script.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum ScriptType {
    Groovy,
    Java,
    JavaScript,
    JUEL,
    Lua,
    Python,
    XPath,
}

impl default::Default for ScriptType {
    fn default() -> Self {
        ScriptType::XPath
    }
}

impl ScriptType {
    /// Get the schema reference for each script type.
    pub fn schema_ref(&self) -> &str {
        match self {
            ScriptType::Groovy => "http://groovy-lang.org",
            ScriptType::Java => "http://www.java.com",
            ScriptType::JavaScript => "http://www.javascript.com",
            ScriptType::JUEL => "http://www.juel.org",
            ScriptType::Lua => "http://www.lua.org",
            ScriptType::Python => "http://www.python.org",
            ScriptType::XPath => "http://www.w3.org/1999/XPath",
        }
    }

    pub fn from_schema_ref(schema_ref: &str) -> Result<Self, EnumParseError> {
        match schema_ref {
            "http://groovy-lang.org" => Ok(ScriptType::Groovy),
            "http://www.java.com" => Ok(ScriptType::Java),
            "http://www.javascript.com" => Ok(ScriptType::JavaScript),
            "http://www.juel.org" => Ok(ScriptType::JUEL),
            "http://www.lua.org" => Ok(ScriptType::Lua),
            "http://www.python.org" => Ok(ScriptType::Python),
            "http://www.w3.org/1999/XPath" => Ok(ScriptType::XPath),
            _ => Err(EnumParseError {
                message: format!("Unknown script type schema reference: {}", schema_ref),
            }),
        }
    }
}

/// Represents an executable script or expression.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum Script {
    Groovy(String),
    Java(String),
    JavaScript(String),
    JUEL(String),
    Lua(String),
    Python(String),
    XPath(String),
}

impl Script {
    /// Creates a new script.
    pub fn new(script: &str, script_type: &ScriptType) -> Self {
        match script_type {
            ScriptType::Groovy => Script::Groovy(script.to_string()),
            ScriptType::Java => Script::Java(script.to_string()),
            ScriptType::JavaScript => Script::JavaScript(script.to_string()),
            ScriptType::JUEL => Script::JUEL(script.to_string()),
            ScriptType::Lua => Script::Lua(script.to_string()),
            ScriptType::Python => Script::Python(script.to_string()),
            ScriptType::XPath => Script::XPath(script.to_string()),
        }
    }

    pub fn script(&self) -> &str {
        match self {
            Script::Groovy(script) => script,
            Script::Java(script) => script,
            Script::JavaScript(script) => script,
            Script::JUEL(script) => script,
            Script::Lua(script) => script,
            Script::Python(script) => script,
            Script::XPath(script) => script,
        }
    }

    /// Returns the `ScriptType` corresponding to the `Script` variant.
    pub fn script_type(&self) -> ScriptType {
        match self {
            Script::Groovy(_) => ScriptType::Groovy,
            Script::Java(_) => ScriptType::Java,
            Script::JavaScript(_) => ScriptType::JavaScript,
            Script::JUEL(_) => ScriptType::JUEL,
            Script::Lua(_) => ScriptType::Lua,
            Script::Python(_) => ScriptType::Python,
            Script::XPath(_) => ScriptType::XPath,
        }
    }
}
