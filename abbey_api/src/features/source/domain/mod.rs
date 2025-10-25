use std::{cell::RefCell, rc::Rc};

use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::{
    features::{
        output::domain::{resource::Resource, Output},
        process::domain::{CyclicProcess, Process},
        source::error::SourceError,
    },
    shared::error::DomainError,
};

/// Represents a source.
pub struct Source {
    /// The ID of this source.
    pub id: Uuid,

    /// The process of this source.
    process: Rc<RefCell<CyclicProcess>>,

    /// The last time a claim was made for the output of the completed cycles of this source.
    last_claim_at: Option<OffsetDateTime>,
}

impl Source {
    /// Creates a new source based on the provided parameters.
    pub fn new(
        possible_resources: Vec<Resource>,
        cycle_duration: Duration,
    ) -> Result<Self, Box<dyn DomainError>> {
        if possible_resources.is_empty() {
            return Err(Box::new(SourceError::NoPossibleResources));
        }

        let process = CyclicProcess::new(cycle_duration, possible_resources);

        Ok(Self {
            id: Uuid::new_v4(),
            process: Rc::new(RefCell::new(process)),
            last_claim_at: None,
        })
    }

    /// Starts the process of this source.
    pub fn start_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        return self.process.borrow_mut().start(now);
    }

    /// Pauses the process of this source.
    pub fn pause_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        return self.process.borrow_mut().pause(now);
    }

    /// Resumes the process of this source.
    pub fn resume_fetching(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        return self.process.borrow_mut().resume(now);
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
            .map(|_| self.process.borrow_mut().get_yield())
            .collect()
    }
}

#[cfg(test)]
mod source_tests {

    mod new_source {
        use time::Duration;

        use crate::{
            features::{
                output::domain::resource::{Category, Resource},
                source::{domain::Source, error::SourceError},
            },
            shared::error::DomainError,
        };

        #[test]
        fn a_new_source_has_an_id() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let source = Source::new(source_resources, cycle_duration).unwrap();

            // Then
            assert!(!source.id.to_string().is_empty());
        }

        #[test]
        fn a_new_source_has_a_process() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let source = Source::new(source_resources, cycle_duration).unwrap();

            // Then
            assert!(!source.process.borrow().id.to_string().is_empty());
        }

        #[test]
        fn a_source_with_needs_possible_resources() {
            // Given
            let source_resources: Vec<Resource> = vec![];
            let cycle_duration = Duration::minutes(1);

            // When
            let creation_attempt = Source::new(source_resources, cycle_duration);

            // Then
            assert!(creation_attempt.is_err());
            assert_eq!(
                SourceError::NoPossibleResources.code(),
                creation_attempt.err().unwrap().code()
            )
        }

        #[test]
        fn a_new_source_has_no_last_claim_time() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let source = Source::new(source_resources, cycle_duration).unwrap();

            // Then
            assert_eq!(None, source.last_claim_at);
        }
    }

    mod fetching_source {
        use std::{cell::RefCell, rc::Rc};

        use time::{Duration, OffsetDateTime};

        use crate::{
            features::{
                actor::domain::{monk::Monk, person::Person},
                assignment::{domain::ProcessAssignmentFactory, error::AssignmentError},
                output::domain::resource::{Category, Resource},
                process::{
                    domain::{Process, Status},
                    error::ProcessError,
                },
                source::domain::Source,
            },
            shared::error::DomainError,
        };

        #[test]
        fn a_fetching_source_needs_assigned_people_before_it_can_start() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let mut source = Source::new(source_resources, cycle_duration).unwrap();

            let attempt = source.start_fetching(OffsetDateTime::now_utc());

            // Then
            assert!(attempt.is_err());
            assert_eq!(
                ProcessError::NoAssignedPeople.code(),
                attempt.err().unwrap().code()
            )
        }

        #[test]
        fn a_fetching_source_needs_available_people_before_it_can_start() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            let another_source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let another_source = Source::new(another_source_resources, cycle_duration).unwrap();

            let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(Monk::new()));
            let another_source_process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&another_source.process) as Rc<RefCell<dyn Process>>;

            ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&another_source_process_rc),
            )
            .expect("The monk should be assigned to the other source's process.");

            let source = Source::new(source_resources, cycle_duration).unwrap();

            let source_process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

            // When
            let assignment_attempt = ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&source_process_rc),
            );

            // Then
            assert!(assignment_attempt.is_err());
            assert_eq!(
                AssignmentError::ActorAssigned.code(),
                assignment_attempt.err().unwrap().code()
            )
        }

        #[test]
        fn a_fetching_source_has_a_process_in_progress() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);
            let monk = Monk::new();

            let mut source = Source::new(source_resources, cycle_duration).unwrap();
            let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(monk));
            let process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

            let assignment_attempt = ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&process_rc),
            );

            // When
            let attempt = source.start_fetching(OffsetDateTime::now_utc());

            // Then
            assert!(assignment_attempt.is_ok());
            assert!(attempt.is_ok());
            assert_eq!(Status::InProgress, process_rc.borrow().status());
        }

        #[test]
        fn a_fetching_source_does_not_give_resources_immediately() {
            // Given
            let now = OffsetDateTime::now_utc();

            let wood = Resource::new("wood".into(), Category::Material);
            let source_resources = vec![wood];

            let mut source = Source::new(source_resources, Duration::minutes(1)).unwrap();

            let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(Monk::new()));
            let process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

            let assignment_attempt = ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&process_rc),
            );

            let attempt = source.start_fetching(now);

            // When
            let claimed_resources = source.claim(now);

            // Then
            assert!(assignment_attempt.is_ok());
            assert!(attempt.is_ok());
            assert!(claimed_resources.is_empty());
        }

        #[test]
        fn a_paused_source_needs_assigned_people_before_it_can_resume() {
            // Given
            let source_resources = vec![Resource::new("wood".into(), Category::Material)];
            let cycle_duration = Duration::minutes(1);

            // When
            let mut source = Source::new(source_resources, cycle_duration).unwrap();

            let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(Monk::new()));
            let process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

            let assignment_attempt = ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&process_rc),
            );

            let start_attempt = source.start_fetching(OffsetDateTime::now_utc());
            assert!(start_attempt.is_ok());

            let pause_attempt = source.pause_fetching(OffsetDateTime::now_utc());
            assert!(pause_attempt.is_ok());

            ProcessAssignmentFactory::unassign_process_from_person(
                Rc::clone(&monk_rc),
                Rc::clone(&process_rc),
            );

            let attempt = source.resume_fetching(OffsetDateTime::now_utc());

            // Then
            assert!(assignment_attempt.is_ok());
            assert!(attempt.is_err());
            assert_eq!(
                ProcessError::NoAssignedPeople.code(),
                attempt.err().unwrap().code()
            )
        }

        #[test]
        fn a_fetching_source_gives_resources_after_at_least_one_completed_cycle() {
            // Given
            let cycle_duration = Duration::minutes(1);

            let wood = Resource::new("wood".into(), Category::Material);
            let source_resources = vec![wood];

            let mut source = Source::new(source_resources, cycle_duration).unwrap();

            let monk_rc: Rc<RefCell<dyn Person>> = Rc::new(RefCell::new(Monk::new()));
            let process_rc: Rc<RefCell<dyn Process>> =
                Rc::clone(&source.process) as Rc<RefCell<dyn Process>>;

            let assignment_attempt = ProcessAssignmentFactory::assign_process_to_person(
                Rc::clone(&monk_rc),
                Rc::clone(&process_rc),
            );

            let attempt = source.start_fetching(OffsetDateTime::now_utc());

            // When
            let claimed_resources = source.claim(
                OffsetDateTime::now_utc()
                    .checked_add(cycle_duration)
                    .expect("The claim time is unknown."),
            );

            // Then
            assert!(assignment_attempt.is_ok());
            assert!(attempt.is_ok());
            assert_eq!(1, claimed_resources.len());

            if let Some(first_claim) = claimed_resources.first() {
                assert_eq!("wood", first_claim.as_ref().unwrap().resource.name);
                assert_eq!(1, first_claim.as_ref().unwrap().quantity);
            }
        }
    }
}
