use std::marker::PhantomData;

use bevy::mesh::{Mesh, MeshVertexAttribute, VertexAttributeValues};
use expecters::{
    assertions::{
        Assertion, AssertionContext, AssertionContextBuilder, AssertionModifier,
        general::IntoInitializableOutput,
    },
    metadata::Annotated,
};

/// Extracts an attribute's values from the subject mesh.
#[derive(Clone, Debug)]
pub struct AttributeModifier<M, T> {
    prev: M,
    attribute: Annotated<MeshVertexAttribute>,
    marker: PhantomData<fn(&[T])>,
}

impl<M, T> AttributeModifier<M, T> {
    pub(crate) fn new(prev: M, attribute: Annotated<MeshVertexAttribute>) -> Self {
        Self {
            prev,
            attribute,
            marker: PhantomData,
        }
    }
}

impl<M, T, A> AssertionModifier<A> for AttributeModifier<M, T>
where
    M: AssertionModifier<AttributeAssertion<A, T>>,
    T: AttributeRef,
{
    type Output = M::Output;

    fn apply(self, cx: AssertionContextBuilder, next: A) -> Self::Output {
        self.prev.apply(
            cx,
            AttributeAssertion {
                next,
                attribute: self.attribute,
                marker: PhantomData,
            },
        )
    }
}

/// Executes the inner assertion on the values of one the subject's attributes.
#[derive(Clone, Debug)]
pub struct AttributeAssertion<A, T> {
    next: A,
    attribute: Annotated<MeshVertexAttribute>,
    marker: PhantomData<fn(&[T])>,
}

impl<'m, A, T> Assertion<&'m Mesh> for AttributeAssertion<A, T>
where
    A: Assertion<&'m [T], Output: IntoInitializableOutput>,
    T: AttributeRef + 'm,
{
    type Output = <A::Output as IntoInitializableOutput>::Initialized;

    fn execute(self, mut cx: AssertionContext, subject: &'m Mesh) -> Self::Output {
        cx.annotate("attribute", self.attribute);
        cx.annotate("expected type", T::EXPECTED_VARIANTS.to_vec().join(", "));

        let Some(values) = subject.attribute(self.attribute.into_inner()) else {
            return cx.fail("attribute doesn't exist on mesh");
        };
        let Some(subject) = T::from_values(values) else {
            return cx.fail("attribute's values are not the expected type");
        };

        self.next.execute(cx, subject).into_initialized()
    }
}

/// A type which a slice can be extracted from a reference to an attribute's
/// [values][`VertexAttributeValues`].
///
/// For instance, `f32` implements this trait because a `&[f32]` can be
/// extracted from an attribute's values if the value type is `Float32`.
pub trait AttributeRef: Sized {
    /// The variants that this conversion supports.
    const EXPECTED_VARIANTS: &[&str];

    /// Try to convert the values to a slice of this type.
    fn from_values(value: &VertexAttributeValues) -> Option<&[Self]>;
}

macro_rules! impl_attrs {
    () => {};
    ($([$($variant:ident),+]($type:ty),)*) => {
        $(
            impl AttributeRef for $type {
                const EXPECTED_VARIANTS: &[&str] = &[
                    $(stringify!($variant),)+
                ];

                fn from_values(value: &VertexAttributeValues) -> Option<&[Self]> {
                    match value {
                        $(VertexAttributeValues::$variant(values) => Some(&values),)+
                        _ => None,
                    }
                }
            }
        )*
    };
}

impl_attrs!(
    [Float32](f32),
    [Sint32](i32),
    [Uint32](u32),
    [Float32x2]([f32; 2]),
    [Sint32x2]([i32; 2]),
    [Uint32x2]([u32; 2]),
    [Float32x3]([f32; 3]),
    [Sint32x3]([i32; 3]),
    [Uint32x3]([u32; 3]),
    [Float32x4]([f32; 4]),
    [Sint32x4]([i32; 4]),
    [Uint32x4]([u32; 4]),
    [Sint16x2, Snorm16x2]([i16; 2]),
    [Uint16x2, Unorm16x2]([u16; 2]),
    [Sint16x4, Snorm16x4]([i16; 4]),
    [Uint16x4, Unorm16x4]([u16; 4]),
    [Sint8x2, Snorm8x2]([i8; 2]),
    [Uint8x2, Unorm8x2]([u8; 2]),
    [Sint8x4, Snorm8x4]([i8; 4]),
    [Uint8x4, Unorm8x4]([u8; 4]),
);
