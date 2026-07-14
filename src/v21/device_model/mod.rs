//! OCPP 2.1 standardized device-model catalogs (feature `device_model_catalog`).
//!
//! Lookup tables of common component and variable names. Matching is case-insensitive.

pub mod components;
pub mod matrix;
pub mod variables;

pub use components::{STANDARD_COMPONENTS, is_standard_component};
pub use matrix::{COMPONENT_VARIABLE_MATRIX, ComponentVariableRow, rows_for_component};
pub use variables::{STANDARD_VARIABLES, VARIABLE_META, VariableMeta, is_standard_variable};
