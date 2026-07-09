use std::sync::Arc;

use address_space::{AddressRange, Rom, Vram};

pub trait SectionTrait {
    #[must_use]
    fn name(&self) -> Arc<str>;

    #[must_use]
    fn section_type(&self) -> Arc<str>;

    #[must_use]
    fn rom(&self) -> Option<AddressRange<Rom>>;

    #[must_use]
    fn vram(&self) -> Option<AddressRange<Vram>>;
}
