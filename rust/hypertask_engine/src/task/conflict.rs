use super::Task;
use crate::error::*;
use chrono::prelude::*;

impl Task {
    pub fn resolve_task_conflict(
        lhs: Option<Task>,
        rhs: Option<Task>,
    ) -> HyperTaskResult<Option<Task>> {
        match (lhs, rhs) {
            (None, None) => Ok(None),
            (Some(task), None) => Ok(Some(task)),
            (None, Some(task)) => Ok(Some(task)),
            (Some(t1), Some(t2)) => {
                if t1.get_id() != t2.get_id() {
                    return Err(HyperTaskError::new(
                        HyperTaskErrorDomain::Task,
                        HyperTaskErrorAction::Compare,
                    )
                    .msg("tried to resolve a conflict between two different tasks"));
                };

                if t1.updated_at > t2.updated_at {
                    Ok(Some(t1))
                } else {
                    Ok(Some(t2))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::Id;
    use std::rc::Rc;
    mod both_none {
        use super::*;

        #[test]
        fn always_returns_none() {
            assert_eq!(Ok(None), Task::resolve_task_conflict(None, None),)
        }
    }

    mod one_none_one_some {
        use super::*;

        #[test]
        fn when_some_is_not_done_returns_task() {
            let task = Task {
                id: Rc::new(Id("test_id".to_owned())),
                description: Some("realy old test task".to_owned()),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            assert_eq!(
                Ok(Some(task.clone())),
                Task::resolve_task_conflict(None, Some(task.clone()))
            );
            assert_eq!(
                Ok(Some(task.clone())),
                Task::resolve_task_conflict(Some(task.clone()), None)
            );
        }
    }

    mod both_some {
        use super::*;

        #[test]
        fn returns_err_when_tasks_do_not_have_matching_ids() {
            let task_1 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            let task_2 = Task {
                id: Rc::new(Id("test_id_2".to_owned())),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            assert_eq!(
                Err(HyperTaskError::new(
                    HyperTaskErrorDomain::Task,
                    HyperTaskErrorAction::Compare
                )),
                Task::resolve_task_conflict(Some(task_2.clone()), Some(task_1.clone()))
            );
            assert_eq!(
                Err(HyperTaskError::new(
                    HyperTaskErrorDomain::Task,
                    HyperTaskErrorAction::Compare
                )),
                Task::resolve_task_conflict(Some(task_1.clone()), Some(task_2.clone()))
            );
        }

        #[test]
        fn when_both_are_not_done_returns_most_recently_updated() {
            let task_1 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),
                updated_at: Utc.ymd(2016, 11, 15).and_hms(9, 10, 11),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            let task_2 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),
                updated_at: Utc.ymd(2017, 11, 15).and_hms(9, 10, 11),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            assert_eq!(
                Ok(Some(task_2.clone())),
                Task::resolve_task_conflict(Some(task_2.clone()), Some(task_1.clone()))
            );
            assert_eq!(
                Ok(Some(task_2.clone())),
                Task::resolve_task_conflict(Some(task_1.clone()), Some(task_2.clone()))
            );
        }
    }
}
