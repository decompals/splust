use std::path::Path;

use abi_stable::{
    library::{LibraryError, RootModule},
    package_version_strings, sabi_trait,
    sabi_types::VersionStrings,
    StableAbi,
};

#[sabi_trait]
pub trait Segment {
    /// Runs before split
    fn scan(&mut self) -> u32;

    /// Do the thing.
    #[sabi(last_prefix_field)]
    fn split(&mut self) -> u32;
}

/// This struct is the root module,
/// which must be converted to `ExampleSeg_Ref` to be passed through ffi.
///
/// The `#[sabi(kind(Prefix(prefix_ref = ExampleSeg_Ref)))]`
/// attribute tells `StableAbi` to create an ffi-safe static reference type
/// for `ExampleLib` called `ExampleSeg_Ref`.
///
/// The `#[sabi(missing_field(panic))]` attribute specifies that trying to
/// access a field that doesn't exist must panic with a message saying that
/// the field is inaccessible.
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ExampleSeg_Ref)))]
#[sabi(missing_field(panic))]
pub struct ExampleSeg {}

/// The RootModule trait defines how to load the root module of a segment.
impl RootModule for ExampleSeg_Ref {
    abi_stable::declare_root_module_statics! {ExampleSeg_Ref}

    const BASE_NAME: &'static str = "splat_segment";
    const NAME: &'static str = "splat_segment";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

/// This loads the root from the library in the `directory` folder.
pub fn load_root_module_in_directory(directory: &Path) -> Result<ExampleSeg_Ref, LibraryError> {
    ExampleSeg_Ref::load_from_directory(directory)
}
