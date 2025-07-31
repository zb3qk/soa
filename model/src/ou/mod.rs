//! An `ou` or Organizational Unit, is a hierarchal component which ultimately composes a
//! system of agents and the goal they are working towards accomplishing.

use std::collections::BTreeMap;
use std::fmt::Debug;
use ulid::Ulid;
use crate::describe::describable::Describable;

mod task;

// pub trait OrganizationalUnit: Describable + Drop + Debug + Send + Sync {
//     fn init(self, organizational_unit_registry: &mut OrganizationalUnitRegistry) where Self: Sized {
//         organizational_unit_registry.put(Box::new(self))
//     }
//
//     // fn drop(&mut self, organizational_unit_registry: &mut OrganizationalUnitRegistry) {
//     //     organizational_unit_registry.remove()
//     // }
// }
//
// pub struct OrganizationalUnitRegistry {
//     registry: BTreeMap<u128, Box<dyn OrganizationalUnit>>
// }
//
// impl OrganizationalUnitRegistry {
//     pub(crate) fn put(&mut self, ou: Box<dyn OrganizationalUnit>) {
//         let id = Ulid::new().0; // Generate a new ULID as the key
//         self.registry.insert(id, ou);
//     }
//
//     pub(crate) fn remove(&mut self, ou: Box<dyn OrganizationalUnit>) {
//         let id = Ulid::new().0; // Generate a new ULID as the key
//         self.registry.insert(id, ou);
//     }
// }