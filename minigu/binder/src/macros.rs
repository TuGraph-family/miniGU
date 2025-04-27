#[macro_export]
macro_rules! impl_binder_not_exist_error_display {
    ( $( ($ty:ty, $kind:expr) ),* $(,)? ) => {
        $(
            impl std::fmt::Display for BinderError<$ty> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let token = &self.input[self.span.clone()];
                    let (line, column) = self.position;
                    write!(
                        f,
                        "{} {} not exists at or near line {} column {}",
                        $kind, token, line, column
                    )
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! impl_binder_already_exist_error_display {
    ( $( ($ty:ty, $kind:expr) ),* $(,)? ) => {
        $(
            impl std::fmt::Display for BinderError<$ty> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let token = &self.input[self.span.clone()];
                    let (line, column) = self.position;
                    write!(
                        f,
                        "{} {} already exists at or near line {} column {}",
                        $kind, token, line, column
                    )
                }
            }
        )*
    }
}

use macro_rules_attribute::attribute_alias;

attribute_alias! {
    #[apply(base)] =
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))];
}