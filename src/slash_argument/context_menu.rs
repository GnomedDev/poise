//! Contains a simple trait, implemented for all context menu command actionss
use crate::{ContextMenuCommandAction, MessageContextAction, UserContextAction};

/// Implemented for all function types which are valid context menu actions.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a valid context menu command signature",
    note = "try `fn(Context, &Message)` for a Message context command",
    note = "try `fn(Context, &User, Option<&PartialMember>)` for a User context command"
)]
pub trait ContextMenuCommandSignature<U, E> {
    /// Convert an action function pointer into a [`ContextMenuCommandAction`]
    fn to_action(self) -> ContextMenuCommandAction<U, E>;
}

impl<U, E> ContextMenuCommandSignature<U, E> for UserContextAction<U, E> {
    fn to_action(self) -> ContextMenuCommandAction<U, E> {
        ContextMenuCommandAction::User(self)
    }
}

impl<U, E> ContextMenuCommandSignature<U, E> for MessageContextAction<U, E> {
    fn to_action(self) -> ContextMenuCommandAction<U, E> {
        ContextMenuCommandAction::Message(self)
    }
}
