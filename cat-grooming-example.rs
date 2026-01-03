use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use specta::{DataType, Generics, Type, TypeMap};

// ---------------------------------------------------------------------
// 1️⃣  Struct that represents a grooming record (5 cat‑related fields)
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
        map.insert("brush_type".to_string(), json!("slicker"));
        map.insert("shedding_score".to_string(), json!(7));             // 0‑10
        map.insert("nail_trimmed".to_string(), json!(true));
        map.insert("favorite_spot".to_string(), json!("chin"));
        Self(map)
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.0.insert(key, value);
    }
}

// ---------------------------------------------------------------------
// 6️⃣  Generate the conversion for the five‑field struct using the new macro
// ---------------------------------------------------------------------
map_to_struct! {
    GroomingStateMap => GroomingRecord {
        fur_length_cm: i32,
        brush_type: String,
        shedding_score: u8,
        nail_trimmed: bool,
        favorite_spot: String,
    }
}

// ---------------------------------------------------------------------
// 7️⃣  Test that the round‑trip serialization matches the struct
// ---------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grooming_state_types_match() {
        let map = GroomingStateMap::new();
        let json = serde_json::to_value(&map).unwrap();

        // Will panic if any type mismatches
        let _: GroomingRecord = serde_json::from_value(json).unwrap();
    }
}
