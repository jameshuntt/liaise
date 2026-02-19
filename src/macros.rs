#[macro_export]macro_rules! register_liaise_errors {
    (
        prefix: $prefix:expr,
        name: $name:ident,
        {
            $($variant:ident = $code:expr => $msg:expr),* $(,)?
        }
    ) => {
        use liaise::{Liaise, RegisterErrors};

        #[derive(RegisterErrors)]
        #[error_prefix = $prefix]
        pub enum $name {
            $($variant = $code),*
        }

        impl Liaise for $name {
            fn code_id(self) -> u16 {
                self as u16
            }

            fn message(self) -> &'static str {
                match self {
                    $(Self::$variant => $msg),*
                }
            }
        }
    };
}

#[macro_export]macro_rules! register_liaise_errors_with_vis {
    (
        prefix: $prefix:expr,
        vis: $vis:vis,
        name: $name:ident,
        {
            $($variant:ident = $code:expr => $msg:expr),* $(,)?
        }
    ) => {
        #[derive(liaise::RegisterErrors)]
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