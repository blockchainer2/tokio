//! Spans represent periods of time in the execution of a program.

use {field, Metadata};

/// Identifies a span within the context of a subscriber.
///
/// They are generated by [`Subscriber`](::Subscriber)s for each span as it is
/// created, by the [`new_span`](::Subscriber::new_span) trait
/// method. See the documentation for that method for more information on span
/// ID generation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id(u64);

/// Attributes provided to a `Subscriber` describing a new span when it is
/// created.
#[derive(Debug)]
pub struct Attributes<'a> {
    metadata: &'a Metadata<'a>,
    values: &'a field::ValueSet<'a>,
    parent: Parent,
}

#[derive(Debug)]
enum Parent {
    /// The new span will be a root span.
    Root,
    /// The new span will be rooted in the current span.
    Current,
    /// The new span has an explicitly-specified parent.
    Explicit(Id),
}

// ===== impl Span =====

impl Id {
    /// Constructs a new span ID from the given `u64`.
    pub fn from_u64(u: u64) -> Self {
        Id(u)
    }

    /// Returns the span's ID as a  `u64`.
    pub fn into_u64(&self) -> u64 {
        self.0
    }
}

// ===== impl Attributes =====

impl<'a> Attributes<'a> {
    /// Returns `Attributes` describing a new child span of the current span,
    /// with the provided metadata and values.
    pub fn new(metadata: &'a Metadata<'a>, values: &'a field::ValueSet<'a>) -> Self {
        Attributes {
            metadata,
            values,
            parent: Parent::Current,
        }
    }

    /// Returns `Attributes` describing a new span at the root of its own trace
    /// tree, with the provided metadata and values.
    pub fn new_root(metadata: &'a Metadata<'a>, values: &'a field::ValueSet<'a>) -> Self {
        Attributes {
            metadata,
            values,
            parent: Parent::Root,
        }
    }

    /// Returns `Attributes` describing a new child span of the specified
    /// parent span, with the provided metadata and values.
    pub fn child_of(
        parent: Id,
        metadata: &'a Metadata<'a>,
        values: &'a field::ValueSet<'a>,
    ) -> Self {
        Attributes {
            metadata,
            values,
            parent: Parent::Explicit(parent),
        }
    }

    /// Returns a reference to the new span's metadata.
    pub fn metadata(&self) -> &Metadata<'a> {
        self.metadata
    }

    /// Returns a reference to a `ValueSet` containing any values the new span
    /// was created with.
    pub fn values(&self) -> &field::ValueSet<'a> {
        self.values
    }

    /// Returns true if the new span shoold be a root.
    pub fn is_root(&self) -> bool {
        match self.parent {
            Parent::Root => true,
            _ => false,
        }
    }

    /// Returns true if the new span's parent should be determined based on the
    /// current context.
    ///
    /// If this is true and the current thread is currently inside a span, then
    /// that span should be the new span's parent. Otherwise, if the current
    /// thread is _not_ inside a span, then the new span will be the root of its
    /// own trace tree.
    pub fn is_contextual(&self) -> bool {
        match self.parent {
            Parent::Current => true,
            _ => false,
        }
    }

    /// Returns the new span's explicitly-specified parent, if there is one.
    ///
    /// Otherwise (if the new span is a root or is a child of the current span),
    /// returns false.
    pub fn parent(&self) -> Option<&Id> {
        match self.parent {
            Parent::Explicit(ref p) => Some(p),
            _ => None,
        }
    }
}