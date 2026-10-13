#[macro_export]
macro_rules! register_liaise_errors {
    (
        prefix: $prefix:literal,
        name: $name:ident,
        {
            $($variant:ident = $code:literal => $msg:literal),* $(,)?
        }
    ) => {
        #[derive(Debug, Copy, Clone, $crate::LiaiseCodes)]
        #[liaise(prefix = $prefix)]
        pub enum $name {
            $(#[liaise(code = $code, msg = $msg)] $variant),*
        }
    };
    (
        prefix: $prefix:literal,
        vis: $vis:vis,
        name: $name:ident,
        {
            $($variant:ident = $code:literal => $msg:literal),* $(,)?
        }
    ) => {
        #[derive(Debug, Copy, Clone, $crate::LiaiseCodes)]
        #[liaise(prefix = $prefix)]
        $vis enum $name {
            $(#[liaise(code = $code, msg = $msg)] $variant),*
        }
    };
}
