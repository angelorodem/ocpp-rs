//! Lenient stringly OCPP enums: unknown wire values become `Unknown(String)` /
//! `Unrecognized(String)` instead of failing deserialize.

/// Define a string enum that preserves non-standard wire values.
///
/// # Syntax
///
/// ```ignore
/// lenient_str_enum! {
///     /// Enum docs
///     @default // optional: derive Default (requires #[default] on one unit variant)
///     pub enum Example {
///         /// Variant docs
///         #[default]
///         Known,
///         Renamed => "Wire.Name",
///         WithAlias => "Celsius" | "Celcius",
///     }
///     @unknown Unknown // or Unrecognized when schema already has Unknown
/// }
/// ```
///
/// Generates `as_str`, `Display`, `Serialize`, `Deserialize`, and `AsRef<str>`.
/// The catch-all variant is always a newtype `String`.
#[macro_export]
macro_rules! lenient_str_enum {
    (
        $(#[$enum_meta:meta])*
        @default
        $vis:vis enum $name:ident {
            $(
                $(#[$var_attr:meta])*
                $variant:ident $(=> $wire:literal $(| $alias:literal)*)?
            ),* $(,)?
        }
        @unknown $unknown:ident
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, PartialEq, Eq, Clone, Default)]
        $vis enum $name {
            $(
                $(#[$var_attr])*
                $variant,
            )*
            /// Non-standard / vendor-specific wire value; original string is preserved.
            $unknown(alloc::string::String),
        }
        $crate::lenient_str_enum!(@impls $name, [$({ $variant $(=> $wire $(| $alias)*)? })*], $unknown);
    };

    (
        $(#[$enum_meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$var_attr:meta])*
                $variant:ident $(=> $wire:literal $(| $alias:literal)*)?
            ),* $(,)?
        }
        @unknown $unknown:ident
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, PartialEq, Eq, Clone)]
        $vis enum $name {
            $(
                $(#[$var_attr])*
                $variant,
            )*
            /// Non-standard / vendor-specific wire value; original string is preserved.
            $unknown(alloc::string::String),
        }
        $crate::lenient_str_enum!(@impls $name, [$({ $variant $(=> $wire $(| $alias)*)? })*], $unknown);
    };

    (
        @impls $name:ident,
        [$({ $variant:ident $(=> $wire:literal $(| $alias:literal)*)? })*],
        $unknown:ident
    ) => {
        impl $name {
            /// OCPP wire spelling (or the preserved vendor string for the catch-all).
            #[must_use]
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        Self::$variant => {
                            $crate::lenient_str_enum!(@wire_name $variant $(=> $wire)?)
                        }
                    )*
                    Self::$unknown(s) => s.as_str(),
                }
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let s = alloc::string::String::deserialize(deserializer)?;
                Ok(match s.as_str() {
                    $(
                        $crate::lenient_str_enum!(@de_pat $variant $(=> $wire $(| $alias)*)?)
                            => Self::$variant,
                    )*
                    _ => Self::$unknown(s),
                })
            }
        }
    };

    (@wire_name $variant:ident) => {
        stringify!($variant)
    };
    (@wire_name $variant:ident => $wire:literal) => {
        $wire
    };

    // Match patterns for deserialize (keeps alias repetition at one depth).
    (@de_pat $variant:ident) => {
        stringify!($variant)
    };
    (@de_pat $variant:ident => $wire:literal $(| $alias:literal)*) => {
        $wire $(| $alias)*
    };
}
