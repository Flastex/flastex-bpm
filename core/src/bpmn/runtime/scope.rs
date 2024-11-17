// This file is part of Flastex BPM, an AGPLv3 licensed project.
// See the LICENSE.md file at the root of the repository for details.

use std::collections::HashMap;
use uuid::Uuid;

use super::data::item::Item;

/// Represents a scope with a unique ID, name, and a set of items.
#[derive(Debug, Clone)]
pub struct Scope {
    id: String,
    name: String,
    items: HashMap<String, Item>,
    sub_scopes: Vec<Scope>,
}

impl Scope {
    /// Creates a new `Scope` with a generated unique ID.
    pub fn new(name: String, items: HashMap<String, Item>, sub_scopes: Vec<Scope>) -> Self {
        Scope {
            id: Uuid::new_v4().to_string().replace('-', ""),
            name,
            items,
            sub_scopes,
        }
    }

    /// Creates a new `Scope` with a specified ID, primarily for testing purposes.
    #[cfg(test)]
    pub fn new_with_id(
        id: String,
        name: String,
        items: HashMap<String, Item>,
        sub_scopes: Vec<Scope>,
    ) -> Self {
        Scope {
            id,
            name,
            items,
            sub_scopes,
        }
    }

    /// Returns the unique ID of the scope.
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Returns the name of the scope.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns a reference to the map of items within the scope.
    pub fn items(&self) -> &HashMap<String, Item> {
        &self.items
    }

    /// Returns a reference to the sub-scopes.
    pub fn sub_scopes(&self) -> &Vec<Scope> {
        &self.sub_scopes
    }
}
