#[macro_export]
macro_rules! register_liaise_errors {
    (
        prefix: $prefix:expr,
        name: $name:ident,
        {
            $($variant:ident = $code:expr => $msg:expr),* $(,)?
        }
    ) => {
        #[derive(Debug, Copy, Clone, $crate::LiaiseCodes)]
        #[liaise(prefix = $prefix)]
        pub enum $name {
            $(#[liaise(code = $code, msg = $msg)] $variant),*
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
        #[derive(Debug, Copy, Clone, $crate::LiaiseCodes)]
        #[liaise(prefix = $prefix)]
        $vis enum $name {
            $(#[liaise(code = $code, msg = $msg)] $variant),*
        }
    };
}
