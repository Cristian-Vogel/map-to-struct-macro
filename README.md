# The `map_to_struct!` Macro

## The Problem
**tauri-specta v1.0.2** doesn't support `serde_json::Value` - it can't generate TypeScript bindings for commands that use dynamic JSON values.

This broke our flexible state management pattern where we store settings as `HashMap<String, Value>` but need a strongly-typed struct for TypeScript bindings.

## The Solution
A macro that converts between the two representations:
```rust
map_to_struct! {
    GroomingStateMap => GroomingRecord {
        fur_length_cm: i32,
        brush_type: String,
        shedding_score: u8,
        nail_trimmed: bool,
        favorite_spot: String,
    }
}
```

This generates a `to_typed()` method that extracts and converts each field automatically.

```rust
// Granular getter with specta support
#[tauri::command]
#[specta::specta]
pub fn get_frontend_state_value(key: String) -> Result<JsonValue, String> {
    with_state(|st8| {
        st8.front_end_state.get(&key)
            .cloned()
            .map(JsonValue)
            .ok_or_else(|| format!("Key not found: {}", key))
    })
}

// Get entire state as strongly typed struct
#[tauri::command]
#[specta::specta]
pub fn fetch_cached_state() -> Result<FrontEndSt8, String> {
    with_state(|st8| {
        st8.front_end_state.to_typed()
    })
}
```

## Benefits
- **Type-safe**: Compiler verifies all conversions match the struct definition
- **DRY**: Field list appears once, conversion logic auto-generates
- **Zero boilerplate**: No manual extraction code for each field
- **Works with tauri-specta**: Commands accept/return the typed struct, which specta handles perfectly

**Result**: Flexible runtime storage with full TypeScript type safety in Tauri apps.
