use super::Task;
use crate::error::*;
use chrono::prelude::*;

impl Task {
    fn is_still_worth_storing(&self, cutoff_date: &DateTime<Utc>) -> bool {
        if self.done.is_some() {
            self.updated_at > *cutoff_date
        } else {
            true
        }
    }

    pub fn resolve_task_conflict(
        cutoff_date: &DateTime<Utc>,
        lhs: Option<Task>,
        rhs: Option<Task>,
    ) -> HyperTaskResult<Option<Task>> {
        match (lhs, rhs) {
            (None, None) => Ok(None),
            (Some(task), None) => {
                if task.is_still_worth_storing(cutoff_date) {
                    Ok(Some(task))
                } else {
                    Ok(None)
                }
            }
            (None, Some(task)) => {
                if task.is_still_worth_storing(cutoff_date) {
                    Ok(Some(task))
                } else {
                    Ok(None)
                }
            }
            (Some(t1), Some(t2)) => {
                if t1.get_id() != t2.get_id() {
                    return Err(HyperTaskError::new(
                        HyperTaskErrorDomain::Task,
                        HyperTaskErrorAction::Compare,
                    )
                    .msg("tried to resolve a conflict between two different tasks"));
                };

                match (
                    t1.is_still_worth_storing(cutoff_date),
                    t2.is_still_worth_storing(cutoff_date),
                ) {
                    (false, false) => Ok(None),
                    (true, false) => Ok(Some(t1)),
                    (false, true) => Ok(Some(t2)),
                    (true, true) => {
                        if t1.updated_at > t2.updated_at {
                            Ok(Some(t1))
                        } else {
                            Ok(Some(t2))
                        }
                    }
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

    fn get_test_cutoff_date() -> DateTime<Utc> {
        Utc.ymd(2019, 11, 15).and_hms(9, 10, 11)
    }

    mod both_none {
        use super::*;

        #[test]
        fn always_returns_none() {
            assert_eq!(
                Ok(None),
                Task::resolve_task_conflict(&get_test_cutoff_date(), None, None),
            )
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
                Task::resolve_task_conflict(&get_test_cutoff_date(), None, Some(task.clone()))
            );
            assert_eq!(
                Ok(Some(task.clone())),
                Task::resolve_task_conflict(&get_test_cutoff_date(), Some(task.clone()), None)
            );
        }

        #[test]
        fn when_some_is_done_and_was_updated_since_cutoff_date_returns_task() {
            let task = Task {
                id: Rc::new(Id("test_id".to_owned())),
                updated_at: Utc.ymd(2019, 11, 20).and_hms(9, 10, 11),
                description: Some("more recent test task".to_owned()),
                done: Some(Utc.ymd(2019, 11, 20).and_hms(9, 10, 11)),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            assert_eq!(
                Ok(Some(task.clone())),
                Task::resolve_task_conflict(&get_test_cutoff_date(), None, Some(task.clone()))
            );
            assert_eq!(
                Ok(Some(task.clone())),
                Task::resolve_task_conflict(&get_test_cutoff_date(), Some(task.clone()), None)
            );
        }

        #[test]
        fn when_some_is_done_and_was_not_updated_since_cutoff_date_returns_none() {
            let task = Task {
                id: Rc::new(Id("test_id".to_owned())),
                updated_at: Utc.ymd(2019, 11, 10).and_hms(9, 10, 11),
                description: Some("task that's been done for a while".to_owned()),
                done: Some(Utc.ymd(2019, 11, 10).and_hms(9, 10, 11)),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            assert_eq!(
                Ok(None),
                Task::resolve_task_conflict(&get_test_cutoff_date(), None, Some(task.clone()))
            );
            assert_eq!(
                Ok(None),
                Task::resolve_task_conflict(&get_test_cutoff_date(), Some(task.clone()), None)
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
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_2.clone()),
                    Some(task_1.clone())
                )
            );
            assert_eq!(
                Err(HyperTaskError::new(
                    HyperTaskErrorDomain::Task,
                    HyperTaskErrorAction::Compare
                )),
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_1.clone()),
                    Some(task_2.clone())
                )
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
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_2.clone()),
                    Some(task_1.clone())
                )
            );
            assert_eq!(
                Ok(Some(task_2.clone())),
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_1.clone()),
                    Some(task_2.clone())
                )
            );
        }

        #[test]
        fn when_both_are_done_but_were_updated_since_cutoff_returns_most_recently_updated() {
            let task_1 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),
                updated_at: Utc.ymd(2019, 11, 20).and_hms(9, 10, 11),
                done: Some(Utc.ymd(2015, 11, 15).and_hms(9, 10, 11)),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            let task_2 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),
                updated_at: Utc.ymd(2019, 11, 21).and_hms(9, 10, 11),
                done: Some(Utc.ymd(2015, 11, 15).and_hms(9, 10, 11)),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            assert_eq!(
                Ok(Some(task_2.clone())),
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_2.clone()),
                    Some(task_1.clone())
                )
            );
            assert_eq!(
                Ok(Some(task_2.clone())),
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_1.clone()),
                    Some(task_2.clone())
                )
            );
        }

        #[test]
        fn when_both_are_done_and_was_not_updated_since_cutoff_returns_none() {
            let task_1 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),
                updated_at: Utc.ymd(2019, 11, 10).and_hms(9, 10, 11),
                done: Some(Utc.ymd(2019, 11, 10).and_hms(9, 10, 11)),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            let task_2 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),
                updated_at: Utc.ymd(2019, 11, 10).and_hms(9, 10, 11),
                done: Some(Utc.ymd(2019, 11, 10).and_hms(9, 10, 11)),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            assert_eq!(
                Ok(None),
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_2.clone()),
                    Some(task_1.clone())
                )
            );
            assert_eq!(
                Ok(None),
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_1.clone()),
                    Some(task_2.clone())
                )
            );
        }

        #[test]
        fn when_only_one_has_been_updated_since_cutoff_it_is_returned() {
            let task_1 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),
                updated_at: Utc.ymd(2019, 11, 20).and_hms(9, 10, 11),
                done: Some(Utc.ymd(2019, 11, 20).and_hms(9, 10, 11)),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            let task_2 = Task {
                id: Rc::new(Id("test_id_1".to_owned())),
                updated_at: Utc.ymd(2015, 11, 15).and_hms(9, 10, 11),
                done: Some(Utc.ymd(2015, 11, 15).and_hms(9, 10, 11)),

                ..Task::generate(&Utc.ymd(2015, 11, 15).and_hms(9, 10, 11))
            };

            assert_eq!(
                Ok(Some(task_1.clone())),
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_2.clone()),
                    Some(task_1.clone())
                )
            );
            assert_eq!(
                Ok(Some(task_1.clone())),
                Task::resolve_task_conflict(
                    &get_test_cutoff_date(),
                    Some(task_1.clone()),
                    Some(task_2.clone())
                )
            );
        }
    }
}
