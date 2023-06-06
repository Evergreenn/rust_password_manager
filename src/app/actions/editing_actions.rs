use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

use crate::inputs::key::Key;

/// We define all available action
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EditingAction {
    Quit,
    RemoveChar,
    Validate,
    Dismiss,
    WriteChar,
}

impl EditingAction {
    /// All available EditingActions
    pub fn iterator() -> Iter<'static, EditingAction> {
        static EDITING_ACTIONS: [EditingAction; 4] = [
            EditingAction::Quit,
            EditingAction::RemoveChar,
            EditingAction::Validate,
            EditingAction::Dismiss,
        ];
        EDITING_ACTIONS.iter()
    }

    /// List of key associated to action
    pub fn keys(&self) -> &[Key] {
        match self {
            EditingAction::Quit => &[Key::Ctrl('c')],
            EditingAction::RemoveChar => &[Key::Backspace],
            EditingAction::Validate => &[Key::Enter],
            EditingAction::Dismiss => &[Key::Esc],
            _ => &[Key::Null], // EditingAction::WriteChar => &[Key::Null],
        }
    }
}

/// Could display a user friendly short description of action
impl Display for EditingAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            EditingAction::Quit => "Quit",
            EditingAction::Dismiss => "Dismiss",
            _ => "",
        };
        write!(f, "{str}")
    }
}

/// The application should have some contextual actions.
#[derive(Default, Debug, Clone)]
pub struct EditingActions(Vec<EditingAction>);

impl EditingActions {
    /// Given a key, find the corresponding action
    pub fn find(&self, key: Key) -> Option<&EditingAction> {
        EditingAction::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    /// Get contextual actions.
    /// (just for building a help view)
    pub fn actions(&self) -> &[EditingAction] {
        self.0.as_slice()
    }
}

impl From<Vec<EditingAction>> for EditingActions {
    /// Build contextual action
    ///
    /// # Panics
    ///
    /// If two actions have same key
    fn from(actions: Vec<EditingAction>) -> Self {
        // Check key unicity
        let mut map: HashMap<Key, Vec<EditingAction>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
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
                    .map(EditingAction::to_string)
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn should_find_action_by_key() {
//         let actions: Actions = vec![Action::Quit, Action::Sleep].into();
//         let result = actions.find(Key::Ctrl('c'));
//         assert_eq!(result, Some(&Action::Quit));
//     }

//     #[test]
//     fn should_find_action_by_key_not_found() {
//         let actions: Actions = vec![Action::Quit, Action::Sleep].into();
//         let result = actions.find(Key::Alt('w'));
//         assert_eq!(result, None);
//     }

//     #[test]
//     fn should_create_actions_from_vec() {
//         let _actions: Actions = vec![
//             Action::Quit,
//             Action::Sleep,
//             Action::IncrementDelay,
//             Action::DecrementDelay,
//         ]
//         .into();
//     }

//     #[test]
//     #[should_panic]
//     fn should_panic_when_create_actions_conflict_key() {
//         let _actions: Actions = vec![
//             Action::Quit,
//             Action::DecrementDelay,
//             Action::Sleep,
//             Action::IncrementDelay,
//             Action::IncrementDelay,
//             Action::Quit,
//             Action::DecrementDelay,
//         ]
//         .into();
//     }
// }
