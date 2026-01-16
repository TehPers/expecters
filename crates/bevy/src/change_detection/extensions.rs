use bevy::ecs::change_detection::DetectChanges;
use expecters::assertions::AssertionBuilder;

use crate::change_detection::{ToBeAdded, ToBeChanged};

/// Assertions for tracking changes to components and resources.
///
/// See [`DetectChangesMut`] for more information on change detection, including
/// its limitations.
///
/// [`DetectChangesMut`]: bevy::ecs::change_detection::DetectChangesMut
pub trait ChangeDetectionAssertions<M> {
    /// Asserts that the subject was added last tick.
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use expecters::prelude::*;
    /// # use expecters_bevy::prelude::*;
    /// #[derive(Default, Resource)]
    /// struct Foo;
    ///
    /// let mut app = App::new();
    /// app.init_resource::<Foo>();
    /// expect!(app.world().resource_ref::<Foo>(), to_be_added);
    /// ```
    ///
    /// The assertion fails if the subject was not added last tick:
    ///
    /// ```should_panic
    /// # use bevy::prelude::*;
    /// # use expecters::prelude::*;
    /// # use expecters_bevy::prelude::*;
    /// #[derive(Default, Resource)]
    /// struct Foo;
    ///
    /// let mut app = App::new();
    /// app.init_resource::<Foo>();
    /// app.update(); // run update tick
    /// expect!(app.world().resource_ref::<Foo>(), to_be_added);
    /// ```
    fn to_be_added(&self) -> ToBeAdded {
        ToBeAdded {}
    }

    /// Asserts that the subject was added or changed since the previous update.
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use expecters::prelude::*;
    /// # use expecters_bevy::prelude::*;
    /// #[derive(Default, Resource)]
    /// struct Foo(i32);
    ///
    /// let mut app = App::new();
    /// app.init_resource::<Foo>();
    /// expect!(app.world().resource_ref::<Foo>(), to_be_changed);
    ///
    /// // Resource is now unchanged
    /// app.update();
    /// expect!(app.world().resource_ref::<Foo>(), not, to_be_changed);
    ///
    /// // Change the resource
    /// app.world_mut().resource_mut::<Foo>().0 = 2;
    /// expect!(app.world().resource_ref::<Foo>(), to_be_changed);
    /// ```
    ///
    /// The assertion fails if the subject is unchanged since the last update:
    ///
    /// ```should_panic
    /// # use bevy::prelude::*;
    /// # use expecters::prelude::*;
    /// # use expecters_bevy::prelude::*;
    /// #[derive(Default, Resource)]
    /// struct Foo;
    ///
    /// let mut app = App::new();
    /// app.init_resource::<Foo>();
    /// app.update(); // run update tick
    /// expect!(app.world().resource_ref::<Foo>(), to_be_added);
    /// ```
    fn to_be_changed(&self) -> ToBeChanged {
        ToBeChanged {}
    }
}

impl<T, M> ChangeDetectionAssertions<M> for AssertionBuilder<T, M> where T: DetectChanges {}
