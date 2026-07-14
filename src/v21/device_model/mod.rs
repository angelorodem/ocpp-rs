//! OCPP 2.1 standardized device-model catalogs (feature `device_model_catalog`).
//!
//! Generated from Part 2 appendices CSVs. Names are case-insensitive per the spec.

pub mod components;
pub mod matrix;
pub mod variables;

pub use components::{STANDARD_COMPONENTS, is_standard_component};
pub use matrix::{COMPONENT_VARIABLE_MATRIX, ComponentVariableRow, rows_for_component};
pub use variables::{STANDARD_VARIABLES, VARIABLE_META, VariableMeta, is_standard_variable};
