use bevy::ecs::change_detection::DetectChanges;
use expecters::{
    AssertionOutput,
    assertions::{Assertion, AssertionContext},
};

/// Asserts that the subject was added since the last update.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ToBeAdded {}

impl<T> Assertion<T> for ToBeAdded
where
    T: DetectChanges,
{
    type Output = AssertionOutput;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        cx.pass_if(subject.is_added(), "not added since last update")
    }
}
