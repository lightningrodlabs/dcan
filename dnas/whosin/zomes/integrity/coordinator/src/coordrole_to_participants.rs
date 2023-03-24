use hdi::prelude::*;
pub fn validate_create_link_coordrole_to_participants(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordrole: crate::Coordrole = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    if target_address != record.signed_action.hashed.author().clone().into() {
        return Ok(ValidateCallbackResult::Invalid(
            "Only the author of the Coordrole can link to it".into(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_coordrole_to_participants(
    _action: DeleteLink,
    _original_action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordrole: crate::Coordrole = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    if target_address != record.signed_action.hashed.author().clone().into() {
        return Ok(ValidateCallbackResult::Invalid(
            "Only the author of the Coordrole can link to it".into(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_participant_to_coordroles(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordrole: crate::Coordrole = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    if base_address != record.signed_action.hashed.author().clone().into() {
        return Ok(ValidateCallbackResult::Invalid(
            "Only the author of the Coordrole can link to it".into(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_participant_to_coordroles(
    _action: DeleteLink,
    _original_action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _coordrole: crate::Coordrole = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    if base_address != record.signed_action.hashed.author().clone().into() {
        return Ok(ValidateCallbackResult::Invalid(
            "Only the author of the Coordrole can link to it".into(),
        ));
    }
    Ok(ValidateCallbackResult::Valid)
}
