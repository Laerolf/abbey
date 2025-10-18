use std::{cell::RefCell, rc::Rc};

use rand::seq::SliceRandom;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::features::{
    output::domain::{resource::Resource, Output},
    process::domain::{CyclicProcess, Process},
};

/// Represents a source.
pub struct Source {
    /// The ID of this source.
    pub id: Uuid,

    /// The process of this source.
    process: Rc<RefCell<CyclicProcess>>,

    /// The possible resources outputted by this source.
    possible_resources: Vec<Resource>,

    /// The last time a claim was made for the output of the completed cycles of this source.
    last_claim_at: Option<OffsetDateTime>,

    /// The base out per person at the end of a process cycle.
    base_output_per_person: i32,
}

impl Source {
    /// Creates a new source based on the provided parameters.
    pub fn new(
        possible_resources: Vec<Resource>,
        cycle_duration: Duration,
        base_output_per_person: i32,
    ) -> Self {
        if possible_resources.is_empty() {
            panic!("A source needs possible resources.");
        }

        let process = CyclicProcess::new(cycle_duration);

        Self {
            id: Uuid::new_v4(),
            process: Rc::new(RefCell::new(process)),
            possible_resources,
            last_claim_at: None,
            base_output_per_person,
        }
    }

    /// Starts the process of this source.
    pub fn start_fetching(&mut self, now: OffsetDateTime) {
        self.process.borrow_mut().start(now);
    }

    /// Pauses the process of this source.
    pub fn pause_fetching(&mut self, now: OffsetDateTime) {
        self.process.borrow_mut().pause(now);
    }

    /// Resumes the process of this source.
    pub fn resume_fetching(&mut self, now: OffsetDateTime) {
        self.process.borrow_mut().resume(now);
    }

    /// Determines output.
    fn determine_output(&self) -> Option<Output> {
        let mut random_number_generator = rand::thread_rng();
        // TODO: Use weights
        let resource_range: Vec<usize> = (0..(self.possible_resources.len())).collect();

        match resource_range.choose(&mut random_number_generator) {
            None => None,
            Some(selected_resource_index) => self
                .possible_resources
                .get(*selected_resource_index)
                // TODO: Use dynamic amounts
                .map(|selected_resource| Output::new(selected_resource.clone(), 10)),
        }
    }

    /// Claims the output of this source's completed process cycles.
    pub fn claim(&mut self, now: OffsetDateTime) -> Vec<Option<Output>> {
        let completed_cycles_since_last_claim = self
            .process
            .borrow_mut()
            .completed_cycles(self.last_claim_at.unwrap_or(now));

        self.last_claim_at = Some(now);

        if completed_cycles_since_last_claim == 0 {
            return Vec::new();
        }

        (0..completed_cycles_since_last_claim)
            .map(|_| self.determine_output())
            .collect()
    }
}

#[cfg(test)]
mod source_tests {

    mod new_source {
        use time::Duration;

        use crate::features::{
            output::domain::resource::{Category, Resource},
            source::domain::Source,
        };

        #[test]
        fn a_new_source_has_an_id() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let source = Source::new(source_resources, cycle_duration, 1);

            // Then
            assert!(!source.id.to_string().is_empty());
        }

        #[test]
        fn a_new_source_has_a_process() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let source = Source::new(source_resources, cycle_duration, 1);

            // Then
            assert!(!source.process.borrow().id.to_string().is_empty());
        }

        #[test]
        fn a_new_source_has_possible_resources() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let source = Source::new(source_resources, cycle_duration, 1);

            // Then
            assert!(!source.possible_resources.is_empty());
        }

        #[test]
        fn a_new_source_with_no_possible_resources_panics() {
            // Given
            let source_resources: Vec<Resource> = vec![];
            let cycle_duration = Duration::minutes(1);

            // When
            let source = std::panic::catch_unwind(|| {
                Source::new(source_resources, cycle_duration, 1);
            });

            // Then
            assert!(source.is_err());
        }

        #[test]
        fn a_new_source_has_no_last_claim_time() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let source = Source::new(source_resources, cycle_duration, 1);

            // Then
            assert_eq!(None, source.last_claim_at);
        }
    }

    mod fetching_source {
        use std::{cell::RefCell, rc::Rc};

        use time::{Duration, OffsetDateTime};

        use crate::features::{
            actor::domain::{monk::Monk, person::Person},
            assignment::domain::ProcessAssignmentFactory,
            output::domain::resource::{Category, Resource},
            process::domain::{Process, Status},
            source::domain::Source,
        };

        #[test]
        fn a_fetching_source_needs_assigned_people_before_it_can_start() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let attempt = std::panic::catch_unwind(|| {
                let mut source = Source::new(source_resources, cycle_duration, 1);
                source.start_fetching(OffsetDateTime::now_utc());
            });

            // Then
            assert!(attempt.is_err());
        }

        #[test]
        fn a_fetching_source_has_a_progress_in_progress() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);
            let monk = Monk::new();

            let mut source = Source::new(source_resources, cycle_duration, 1);
            let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(monk));
            let process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

            ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&process_rc),
            );

            // When
            source.start_fetching(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, process_rc.borrow().status());
        }

        #[test]
        fn a_fetching_source_does_not_give_resources_immediately() {
            // Given
            let now = OffsetDateTime::now_utc();

            let wood = Resource::new("wood".into(), Category::Material);
            let source_resources = vec![wood];

            let mut source = Source::new(source_resources, Duration::minutes(1), 1);

            let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(Monk::new()));
            let process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

            ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&process_rc),
            );

            source.start_fetching(now);

            // When
            let claimed_resources = source.claim(now);

            // Then
            assert!(claimed_resources.is_empty());
        }

        #[test]
        fn a_paused_source_needs_assigned_people_before_it_can_resume() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let attempt = std::panic::catch_unwind(|| {
                let mut source = Source::new(source_resources, cycle_duration, 1);

                let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(Monk::new()));
                let process_rc: Rc<RefCell<dyn Process>> =
                    Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

                ProcessAssignmentFactory::assign_process_to_person(
                    Rc::clone(&monk_rc),
                    Rc::clone(&process_rc),
                );

                source.start_fetching(OffsetDateTime::now_utc());
                source.pause_fetching(OffsetDateTime::now_utc());

                ProcessAssignmentFactory::unassign_process_from_person(
                    Rc::clone(&monk_rc),
                    Rc::clone(&process_rc),
                );

                source.resume_fetching(OffsetDateTime::now_utc());
            });

            // Then
            assert!(attempt.is_err());
        }

        #[test]
        fn a_fetching_source_gives_resources_after_at_least_one_completed_cycle() {
            // Given
            let cycle_duration = Duration::minutes(1);

            let wood = Resource::new("wood".into(), Category::Material);
            let source_resources = vec![wood];

            let mut source = Source::new(source_resources, cycle_duration, 1);

            let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(Monk::new()));
            let process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

            ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&process_rc),
            );

            source.start_fetching(OffsetDateTime::now_utc());

            // When
            let claimed_resources = source.claim(
                OffsetDateTime::now_utc()
                    .checked_add(cycle_duration)
                    .expect("The claim time is unknown."),
            );

            // Then
            assert_eq!(1, claimed_resources.len());

            match claimed_resources.first().unwrap() {
                None => panic!("There should be at least one output."),
                Some(output) => {
                    assert_eq!("wood", output.resource.name);
                    assert_eq!(10, output.quantity);
                }
            }
        }
    }
}
