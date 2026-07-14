//! OCPP 2.1 standardized device-model catalogs (feature `device_model_catalog`).
//!
//! Generated from Part 2 appendices CSVs. Names are case-insensitive per the spec.

pub mod components;
pub mod matrix;
pub mod variables;

pub use components::{is_standard_component, STANDARD_COMPONENTS};
pub use matrix::{rows_for_component, ComponentVariableRow, COMPONENT_VARIABLE_MATRIX};
pub use variables::{is_standard_variable, VariableMeta, STANDARD_VARIABLES, VARIABLE_META};
