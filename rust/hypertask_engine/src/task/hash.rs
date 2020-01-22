use super::Task;

use blake2::digest::{Input, VariableOutput};
use blake2::VarBlake2b;

impl Task {
    pub fn calculate_hash(&self) -> u64 {
        //TODO this is a BAD implementation of this logic,
        //it should be replaced as soon as we've got a good way to convert tasks to byte arrays
        let s = serde_json::to_string(&self).expect("could not serialise task");

        let mut hasher = VarBlake2b::new(4).unwrap();

        hasher.input(s);

        hasher
            .vec_result()
            .iter()
            .fold(0, |acc, dat| (acc * 256) + (*dat as u64))
    }
}

#[cfg(test)]
mod test {
    extern crate wasm_bindgen_test;

    use super::*;
    use crate::id::Id;
    use chrono::prelude::*;
    use std::collections::HashSet;
    use std::rc::Rc;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn does_hash_to_consistent_value() {
        let task = Task {
            created_at: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            blocked_by: None,
            description: None,
            done: None,
            due: None,
            id: Rc::new(Id("test_id".into())),
            recur: None,
            snooze: None,
            tags: HashSet::new(),
            updated_at: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            wait: None,
        };

        assert_eq!(task.calculate_hash(), 1218049881);
    }

    #[wasm_bindgen_test]
    fn hashes_different_tasks_to_different_values() {
        let task = Task {
            created_at: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            blocked_by: None,
            description: Some("with a description".into()),
            done: None,
            due: None,
            id: Rc::new(Id("test_id".into())),
            recur: None,
            snooze: None,
            tags: HashSet::new(),
            updated_at: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            wait: None,
        };

        assert_eq!(task.calculate_hash(), 1780513461);
    }
}
