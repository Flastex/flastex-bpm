use uuid::Uuid;

use crate::bpmn::runtime::path_identifier::PathIdentifier;

/// Generate a unique token ID for a new token.
/// path_identifier: The identifier of the path the token is on
/// debug_identifier: An optional identifier to prepend to the token ID for debugging
/// Returns: A unique token ID
pub fn generate_token_id(
    path_identifier: PathIdentifier,
    debug_identifier: Option<String>,
) -> String {
    // Generate the UUID v4
    let uuid = Uuid::new_v4().to_string().replace('-', "");

    // Convert PathIdentifier to String
    let path_id = path_identifier.to_string();

    // Construct the token ID
    let token_id = format!("{}-{}", path_id, uuid);

    // If a debug identifier is present, prepend it
    if let Some(debug_id) = debug_identifier {
        return format!("{}-{}", debug_id, token_id);
    }

    token_id
}
