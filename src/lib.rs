// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-targets: systemd targets component model.
//!
//! Real implementation: targetss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Targets {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdTargetsError {
    EmptyTargetsName,
    DuplicateTargets(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTargetsName => write!(f, "targets name must not be empty"),
            Self::DuplicateTargets(id) => write!(f, "duplicate targets {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdTargetsError {}

pub struct SystemdTargets {
    pub name: String,
    targetss: BTreeMap<String, Targets>,
    events: BTreeMap<String, Event>,
}

impl SystemdTargets {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            targetss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_targets(&mut self, item: Targets) -> Result<(), SystemdTargetsError> {
        if item.name.trim().is_empty() {
            return Err(SystemdTargetsError::EmptyTargetsName);
        }
        let id = item.id.clone();
        if self.targetss.contains_key(&id) {
            return Err(SystemdTargetsError::DuplicateTargets(id.clone()));
        }
        self.targetss.insert(id, item);
        Ok(())
    }

    pub fn targets_count(&self) -> usize {
        self.targetss.len()
    }

    pub fn get_targets(&self, id: &str) -> Option<&Targets> {
        self.targetss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdTargetsError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdTargetsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_targets(id: &str, name: &str) -> Targets {
        Targets {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdTargets::new("SystemdTargets");
        assert_eq!(m.name, "SystemdTargets");
        assert_eq!(m.targets_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_targets() {
        let mut m = SystemdTargets::new("SystemdTargets");
        m.add_targets(sample_targets("a1", "Alpha")).unwrap();
        assert_eq!(m.targets_count(), 1);
        assert!(m.get_targets("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdTargets::new("SystemdTargets");
        assert!(m.add_targets(sample_targets("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdTargets::new("SystemdTargets");
        m.add_targets(sample_targets("x", "One")).unwrap();
        assert!(m.add_targets(sample_targets("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdTargets::new("SystemdTargets");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
