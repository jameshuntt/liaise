#[macro_export]macro_rules! register_liaise_errors {
    (
        prefix: $prefix:expr,
        name: $name:ident,
        {
            $($variant:ident = $code:expr => $msg:expr),* $(,)?
        }
    ) => {
        #[derive($crate::RegisterErrors, Copy, Clone)]
        #[error_prefix = $prefix]
        pub enum $name {
            $($variant = $code),*
        }

        impl $crate::Liaise for $name {
            fn code_id(self) -> u16 { self as u16 }
            fn message(self) -> &'static str {
                match self {
                    $(Self::$variant => $msg),*
                }
            }
        }
    };
    (
        prefix: $prefix:expr,
        vis: $vis:vis,
        name: $name:ident,
        {
            $($variant:ident = $code:expr => $msg:expr),* $(,)?
        }
    ) => {
        #[derive($crate::RegisterErrors, Copy, Clone)]
        #[error_prefix = $prefix]
        $vis enum $name {
            $($variant = $code),*
        }

        impl liaise::Liaise for $name {
            fn code_id(self) -> u16 { self as u16 }
            fn message(self) -> &'static str {
                match self {
                    $(Self::$variant => $msg),*
                }
            }
        }
    };
}
