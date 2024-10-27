/// Enum for process type.
#[derive(Clone, strum::Display, strum::EnumString, PartialEq, Debug)]
pub enum XmlElementType {
    /// Start tag (with attributes) `<tag attr="value">`.
    Start,
    /// End tag `</tag>`.
    End,
    /// Empty element tag (with attributes) `<tag attr="value" />`.
    Empty,
    /// Escaped character data between tags.
    Text,
    /// Unescaped character data stored in `<![CDATA[...]]>`.
    CData,
    /// Comment `<!-- ... -->`.
    Comment,
    /// XML declaration `<?xml ...?>`.
    Decl,
    /// Processing instruction `<?...?>`.
    PI,
    /// Document type definition data (DTD) stored in `<!DOCTYPE ...>`.
    DocType,
    /// End of XML document.
    Eof,
}
