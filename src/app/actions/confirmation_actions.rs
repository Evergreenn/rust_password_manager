use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

use crate::inputs::key::Key;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ConfirmationAction {
    Cancel,
    Confirm,
}

impl ConfirmationAction {
    pub fn iterator() -> Iter<'static, ConfirmationAction> {
        static ACTIONS: [ConfirmationAction; 2] =
            [ConfirmationAction::Cancel, ConfirmationAction::Confirm];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> &[Key] {
        match self {
            ConfirmationAction::Cancel => &[Key::Char('c'), Key::Esc],
            ConfirmationAction::Confirm => &[Key::Char('y'), Key::Enter],
        }
    }
}

impl Display for ConfirmationAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfirmationAction::Cancel => write!(f, "Cancel"),
            ConfirmationAction::Confirm => write!(f, "Confirm"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ConfirmationActions(Vec<ConfirmationAction>);

impl ConfirmationActions {
    pub fn find(&self, key: Key) -> Option<&ConfirmationAction> {
        ConfirmationAction::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    pub fn actions(&self) -> &[ConfirmationAction] {
        self.0.as_slice()
    }
}
impl From<Vec<ConfirmationAction>> for ConfirmationActions {
    fn from(actions: Vec<ConfirmationAction>) -> Self {
        let mut map: HashMap<Key, Vec<ConfirmationAction>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1) // at least two EditingActions share same shortcut
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(ConfirmationAction::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {key} with action {actions}")
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        // Ok, we can create contextual actions
        Self(actions)
    }
}
