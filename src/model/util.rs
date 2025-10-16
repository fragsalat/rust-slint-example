#[macro_export]
macro_rules! with_getters_setters {
    (
        $(#[$meta:meta])*
        $inner_vis:vis struct $Inner:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident : $ty:ty,
            )*
        }

        $outer_vis:vis struct $Outer:ident {
            $(
                $(#[$outer_field_meta:meta])*
                $outer_field_vis:vis $outer_field:ident : $outer_ty:ty,
            )*
        }
    ) => {
        // ---- Inner struct definition ----
        $(#[$meta])*
        $inner_vis struct $Inner {
            $(
                $(#[$field_meta])*
                $field_vis $field : $ty,
            )*
        }

        // ---- Outer struct definition ----
        #[derive(Clone)]
        $outer_vis struct $Outer {
            pub inner: std::sync::Arc<std::sync::Mutex<$Inner>>,
            $(
                $(#[$outer_field_meta])*
                $outer_field_vis $outer_field : $outer_ty,
            )*
        }

        $outer_vis enum Field {
            $(
                $field($ty),
            )*
        }

        // ---- Impl for outer struct ----
        impl $Outer {
            $(
                pub fn $field(&self) -> $ty
                where
                    $ty: Clone,
                {
                    self.inner.lock().unwrap().$field.clone()
                }
            )*

            pub fn set(&self, field: Field) {
                match field {
                    $(Field::$field(value) => self.inner.lock().unwrap().$field = value,)*
                }
            }
        }
    };
}
