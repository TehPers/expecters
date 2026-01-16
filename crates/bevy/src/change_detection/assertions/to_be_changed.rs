use bevy::ecs::change_detection::DetectChanges;
use expecters::{
    AssertionOutput,
    assertions::{Assertion, AssertionContext},
};

/// Asserts that the subject was added or changed since the last update.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ToBeChanged {}

impl<T> Assertion<T> for ToBeChanged
where
    T: DetectChanges,
{
    type Output = AssertionOutput;

    fn execute(self, cx: AssertionContext, subject: T) -> Self::Output {
        cx.pass_if(
            subject.is_changed(),
            "not added or changed since last update",
        )
    }
}
