//! `LocationEnumType`

crate::lenient_str_enum! {
    pub enum LocationEnumType {
        Body,
        Cable,
        EV,
        Inlet,
        Outlet,
        Upstream,
    }
    @unknown Unknown
}
