use std::collections::BTreeMap;
use crate::describe::description::Description;
use crate::output::formatting::LevelOfDetail;

/// Anything that can be described. These include tasks, agents, systems, etc.
pub trait Describable<> {
    /// When a user seeks to track progress, debug control flow, or understand what components
    /// they have at their disposal, descriptors are their mechanism to get this information.
    ///
    /// To drive this point further, here is an example; A task can be described by:
    /// 1. The intent of the task
    /// 2. The agents that are working on it
    /// 3. The progress being made so far
    /// Each of these are descriptors of the task.
    ///
    /// Let's formalize how descriptors should be defined:
    ///
    /// Descriptors should be defined based on informative intent. What unit of information
    /// should be made available to the user? Define descriptors with these tenets:
    /// 1. Information should be specific to what is being described and not overlap with other
    /// things which are described. Information about other components can confuse the user.
    /// 2. Information across descriptors should not be repeated. Data presented must be discrete.
    /// It is the responsibility of the interface displaying these descriptions to gather useful
    /// descriptions and make the input meaningful.
    fn descriptors(&self) -> BTreeMap<String, Description>;
}

