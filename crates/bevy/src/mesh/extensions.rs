use bevy::mesh::{Mesh, MeshVertexAttribute};
use expecters::{assertions::AssertionBuilder, metadata::Annotated};

use crate::mesh::{AttributeModifier, AttributeRef};

/// Modifiers for [`Mesh`]es.
pub trait MeshAssertions<'m, M> {
    /// Asserts that the subject has an attribute of the given type, then
    /// continues the assertion with the attribute's values.
    ///
    /// ```
    /// # use bevy::{asset::RenderAssetUsages, mesh::PrimitiveTopology, prelude::*};
    /// # use expecters::prelude::*;
    /// # use expecters_bevy::prelude::*;
    /// let mut mesh = Mesh::new(
    ///     PrimitiveTopology::TriangleList,
    ///     RenderAssetUsages::default(),
    /// );
    /// mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[1.0, 2.0, 3.0]]);
    ///
    /// expect!(
    ///     &mesh,
    ///     attribute::<[f32; 3]>(Mesh::ATTRIBUTE_POSITION),
    ///     count,
    ///     to_equal(1),
    /// );
    /// ```
    fn attribute<T>(
        self,
        attribute: Annotated<MeshVertexAttribute>,
    ) -> AssertionBuilder<&'m [T], AttributeModifier<M, T>>
    where
        T: AttributeRef;
}

impl<'m, M> MeshAssertions<'m, M> for AssertionBuilder<&'m Mesh, M> {
    fn attribute<T>(
        self,
        attribute: Annotated<MeshVertexAttribute>,
    ) -> AssertionBuilder<&'m [T], AttributeModifier<M, T>>
    where
        T: AttributeRef,
    {
        AssertionBuilder::modify(self, |prev| AttributeModifier::new(prev, attribute))
    }
}
