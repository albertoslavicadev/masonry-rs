use std::any::Any;
use std::collections::VecDeque;
use std::sync::Arc;

use crate::{WidgetId, WindowId};

// TODO - Rename
// TODO - Figure out what these actions should be

// TODO - TextCursor changed, ImeChanged, EnterKey, MouseEnter
#[non_exhaustive]
#[allow(missing_docs)]
/// Events from UI elements.
///
/// Note: Actions are still a WIP feature.
pub enum Action {
    ButtonPressed,
    TextChanged(String),
    TextEntered(String),
    CheckboxChecked(bool),
    // FIXME - This is a huge hack
    Other(Arc<dyn Any>),
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TextChanged(l0), Self::TextChanged(r0)) => l0 == r0,
            (Self::TextEntered(l0), Self::TextEntered(r0)) => l0 == r0,
            (Self::CheckboxChecked(l0), Self::CheckboxChecked(r0)) => l0 == r0,
            (Self::Other(val_l), Self::Other(val_r)) => Arc::ptr_eq(val_l, val_r),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Action {}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ButtonPressed => write!(f, "ButtonPressed"),
            Self::TextChanged(text) => f.debug_tuple("TextChanged").field(text).finish(),
            Self::TextEntered(text) => f.debug_tuple("TextEntered").field(text).finish(),
            Self::CheckboxChecked(b) => f.debug_tuple("CheckboxChecked").field(b).finish(),
            Self::Other(_) => write!(f, "Other(...)"),
        }
    }
}

/// Our queue type
pub(crate) type ActionQueue = VecDeque<(Action, WidgetId, WindowId)>;
