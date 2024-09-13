/// For including a tab-activated debug panel for a give resource
#[macro_export]
macro_rules! debug_resource {
    ($app:expr, $resource:ty) => {{
        $app.add_plugins(
            ResourceInspectorPlugin::<$resource>::new()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
    }};
}
pub use debug_resource;

/// Implements `get` for a field using copy
#[macro_export]
macro_rules! impl_get_copy {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<get_ $field>](&self) -> $type {
                self.$field
            }
        }
    };
}
pub use impl_get_copy;

/// Implements `get` for a field using reference
#[macro_export]
macro_rules! impl_get_ref {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<get_ $field>](&self) -> &$type {
                &self.$field
            }
        }
    };
}
pub use impl_get_ref;

/// Implements `get` for a field using clone
#[macro_export]
macro_rules! impl_get_clone {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<get_ $field>](&self) -> $type {
                self.$field.clone()
            }
        }
    };
}
pub use impl_get_clone;

/// Implements `set` for a field
#[macro_export]
macro_rules! impl_set {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<set_ $field>](&mut self, val: $type) {
                self.$field = val;
            }
        }
    };
}
pub use impl_set;

/// Implements `with` for a field
#[macro_export]
macro_rules! impl_with {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<with_ $field>](mut self, val: $type) -> Self {
                self.$field = val;
                self
            }
        }
    };
}
pub use impl_with;
