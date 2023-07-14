// Define a `pub mod`, optionally at a custom path
#[macro_export] macro_rules! module {
    ($name:ident) => {
        pub mod $name;
    };
    ($name:ident @ $path:literal) => {
        #[path = $path] pub mod $name;
    };
}

/// Define a `mod` and reexport its contents
#[macro_export] macro_rules! module_flat {
    ($name:ident) => {
        mod $name; pub use self::$name::*;
    };
    ($name:ident @ $path:literal) => {
        #[path = $path] mod $name; pub use self::$name::*;
    };
}

/// Define a `pub mod`, but only if a certain Cargo feature is enabled
#[macro_export] macro_rules! optional_module {
    ($feat:literal : $name:ident) => {
        #[cfg(feature=$feat)] pub mod $name;
    };
    ($feat:literal : $name:ident @ $path:literal) => {
        #[cfg(feature=$feat)] #[path=$path] pub mod $name;
    };
}

/// Define a `mod` and reexport its contents if a Cargo feature is enabled
#[macro_export] macro_rules! optional_module_flat {
    ($feat:literal : $name:ident) => {
        #[cfg(feature=$feat)] mod $name;
        #[cfg(feature=$feat)] pub use self::$name::*;
    };
    ($feat:literal : $name:ident @ $path:literal) => {
        #[cfg(feature=$feat)] #[path = $path] mod $name;
        #[cfg(feature=$feat)] pub use self::$name::*;
    };
}
