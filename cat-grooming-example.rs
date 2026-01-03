use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use specta::{DataType, Generics, Type, TypeMap};

// ---------------------------------------------------------------------
// 1️⃣  Struct that represents a cat grooming record (5 cat‑related fields)
// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct GroomingRecord {
    pub fur_length_cm: i32,   // measured length of the cat’s fur
    pub brush_type: String,   // e.g. “slicker”, “pin”, “metal”
    pub shedding_score: u8,   // 0‑10 rating of how much hair is shedding
    pub nail_trimmed: bool,   // was the nail trimming done?
    pub favorite_spot: String,// where the cat likes to be groomed
}

// ---------------------------------------------------------------------
// 2️⃣  Wrapper around a HashMap<String, Value>
// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GroomingStateMap(pub HashMap<String, Value>);

// Manual Specta implementation (unchanged)
impl Type for GroomingStateMap {
    fn inline(_type_map: &mut TypeMap, _generics: Generics) -> DataType {
        DataType::Primitive(specta::datatype::PrimitiveType::String)
    }
}

// ---------------------------------------------------------------------
// 3️⃣  Macro that creates a `to_typed` conversion method
// ---------------------------------------------------------------------
macro_rules! map_to_struct {
    (
        $map_type:ty => $struct_name:ident {
            $(
                $field:ident : $type:ty
            ),* $(,)?
        }
    ) => {
        impl $map_type {
            pub fn to_typed(&self) -> Result<$struct_name, String> {
                $(
                    let $field = extract_field::<$type>(&self.0, stringify!($field))?;
                )*

                Ok($struct_name {
                    $( $field, )*
                })
            }
        }
    };
}

// ---------------------------------------------------------------------
// 4️⃣  Helper that pulls a typed value out of the map
// ---------------------------------------------------------------------
fn extract_field<T>(map: &HashMap<String, Value>, key: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    map.get(key)
        .cloned()
        .ok_or_else(|| format!("Missing {}", key))
        .and_then(|v| {
            serde_json::from_value(v)
                .map_err(|e| format!("Invalid {}: {}", key, e))
        })
}

// ---------------------------------------------------------------------
// 5️⃣  Implementation of the map (populated with the 5 cat‑grooming keys)
// ---------------------------------------------------------------------
impl GroomingStateMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("fur_length_cm".to_string(), json!(2));               // centimeters
        map.insert("brush_type".to_string()
