#[macro_export]
macro_rules! run {
    ($addr:expr, $($module:ident),+) => {
        vial::run($addr, vec![$($module::vial_recognize),+]);
    };
    ($addr:expr) => {
        vial::run($addr, vec![vial_recognize]);
    };
}

#[macro_export]
macro_rules! vial {
    ( $($method:ident $path:expr => $body:expr;)* ) => {
        fn vial_check_method() {
            #[allow(non_snake_case)]
            fn GET() {}
            #[allow(non_snake_case)]
            fn POST() {}
            $($method();)*
        }


        pub(crate) fn vial_recognize(req: &::vial::Request) -> Option<fn(::vial::Request) -> ::vial::Response> {
            match (req.method(), req.path()) {
                $( (stringify!($method), $path) => Some($body), )*
                _ => None,
            }
        }
    };
}
