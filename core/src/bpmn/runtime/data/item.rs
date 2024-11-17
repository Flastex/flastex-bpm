// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

/// Represents an item in the scope, currently supporting only `DataObject`.
#[derive(Debug, Clone)]
pub enum Item {
    DataObject(DataObject),
}

/// Represents a data object within a scope.
#[derive(Debug, Clone)]
pub struct DataObject {
    pub name: String,
    pub value: String,
}

impl DataObject {
    /// Creates a new `DataObject`.
    pub fn new(name: String, value: String) -> Self {
        DataObject { name, value }
    }
}
