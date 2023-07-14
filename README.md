# Flatmod

Dirty shorthands for more flexible module organization.

## Examples

```rust
use flatmod::*

// Define a `pub mod`, optionally at a custom path:
module!(submod1);
module!(submod2 @ "./path/to/submod2.rs");

// Define a `mod` and reexport its contents:
module_flat!(submod3);
module_flat!(submod4 @ "./path/to/submod3.rs");

// Define a `pub mod`, but only if a certain Cargo feature is enabled:
optional_module!("enable-submod5": submod5);
optional_module!("enable-submod6": submod6 @ "./path/to/submod6.rs");

// Define a `mod` and reexport its contents if a Cargo feature is enabled:
optional_module!("enable-submod5": submod5);
optional_module!("enable-submod6": submod6 @ "./path/to/submod6.rs");
```
