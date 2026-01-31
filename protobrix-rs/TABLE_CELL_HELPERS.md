# TableCellValue Helper Functions

This document describes the `TableCellValueExt` extension trait that provides convenient helper functions for creating `TableCellValue` instances.

## Overview

The `TableCellValueExt` trait extends `TableCellValue` with static methods that make it easier to create table cell values without dealing with the verbose protobuf enum syntax.

## Usage

First, import the trait:

```rust
use protobrix_rs::TableCellValueExt;
```

Then you can use the helper methods directly on `TableCellValue`:

```rust
// Create a string cell
let name_cell = TableCellValue::string("John Doe");

// Create an integer cell
let id_cell = TableCellValue::int(42);

// Create a cell from a u64 (automatically converts to i32)
let user_id_cell = TableCellValue::int_u64(user.id);

// Create a boolean cell
let active_cell = TableCellValue::bool(true);

// Create a double (f64) cell
let price_cell = TableCellValue::double(19.99);

// Create a float (f32) cell
let rating_cell = TableCellValue::float(4.5);

// Create an empty cell
let empty_cell = TableCellValue::empty();
```

## Optional Values

The trait also provides methods for working with `Option` types:

```rust
// Create a cell from Option<String>
let description_cell = TableCellValue::opt_string(user.description);

// Create a cell from Option<i32>
let age_cell = TableCellValue::opt_int(user.age);

// Create a cell from Option<f64>
let balance_cell = TableCellValue::opt_double(account.balance);

// Create a cell from Option<f32>
let score_cell = TableCellValue::opt_float(game.score);

// Create a cell from Option<bool>
let verified_cell = TableCellValue::opt_bool(user.verified);
```

If the `Option` is `None`, the cell will have `value: None`.

## DateTime Support

With the `chrono` feature enabled (default), you can create cells from `Option<chrono::NaiveDateTime>`:

```rust
// Create a cell from a datetime (formats as ISO 8601)
let created_at_cell = TableCellValue::datetime(record.created_at);
```

This will format the datetime as `%Y-%m-%dT%H:%M:%S` (e.g., "2024-01-31T14:30:00").

## Complete Example

Here's a complete example of using the helpers in a table row builder:

```rust
use protobrix_rs::*;
use protobrix_rs::TableCellValueExt;

fn user_to_row(user: &User, columns: &[UserColumn]) -> AdvancedTableRow {
    let mut cells = Vec::new();

    for column in columns {
        let cell = match column {
            UserColumn::Id => TableCellValue::int_u64(user.id),
            UserColumn::Name => TableCellValue::string(&user.name),
            UserColumn::Email => TableCellValue::string(&user.email),
            UserColumn::Active => TableCellValue::bool(user.active),
            UserColumn::Balance => TableCellValue::opt_double(user.balance),
            UserColumn::CreatedAt => TableCellValue::datetime(user.created_at),
        };
        cells.push(cell);
    }

    AdvancedTableRow {
        cells,
        action_buttons: vec![],
        row_action: None,
    }
}
```

## Before and After Comparison

### Before (verbose protobuf syntax):

```rust
let cell = TableCellValue {
    value: Some(table_cell_value::Value::StringValue(user.name.clone())),
};

let date_cell = TableCellValue {
    value: Some(table_cell_value::Value::StringValue(
        user.created_at
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string())
            .unwrap_or_default()
    )),
};
```

### After (using helpers):

```rust
let cell = TableCellValue::string(&user.name);

let date_cell = TableCellValue::datetime(user.created_at);
```

## Available Methods

### Creation Methods

| Method | Parameter Type | Description |
|--------|---------------|-------------|
| `string(value)` | `impl Into<String>` | Create a string cell |
| `int(value)` | `i32` | Create an integer cell |
| `int_u64(value)` | `u64` | Create an integer cell from u64 (converts to i32) |
| `double(value)` | `f64` | Create a double cell |
| `float(value)` | `f32` | Create a float cell |
| `bool(value)` | `bool` | Create a boolean cell |
| `empty()` | - | Create an empty cell (None) |
| `opt_string(value)` | `Option<String>` | Create a cell from optional string |
| `opt_int(value)` | `Option<i32>` | Create a cell from optional integer |
| `opt_double(value)` | `Option<f64>` | Create a cell from optional double |
| `opt_float(value)` | `Option<f32>` | Create a cell from optional float |
| `opt_bool(value)` | `Option<bool>` | Create a cell from optional boolean |
| `datetime(value)` | `Option<chrono::NaiveDateTime>` | Create a cell from optional datetime (requires `chrono` feature) |

### Conversion Methods

These methods extract values from a `TableCellValue`. They return `None` if the cell doesn't contain the expected type.

| Method | Return Type | Description |
|--------|------------|-------------|
| `as_string(&self)` | `Option<String>` | Extract as string (clones), returns None if not a string |
| `as_int(&self)` | `Option<i32>` | Extract as i32, returns None if not an int |
| `as_u64(&self)` | `Option<u64>` | Extract as u64, returns None if not an int or if negative |
| `as_double(&self)` | `Option<f64>` | Extract as f64, returns None if not a double |
| `as_float(&self)` | `Option<f32>` | Extract as f32, returns None if not a float |
| `as_bool(&self)` | `Option<bool>` | Extract as bool, returns None if not a boolean |
| `into_string(self)` | `Option<String>` | Consume and extract as string (no clone), returns None if not a string |

## Extracting Values from Cells

You can extract values from `TableCellValue` instances using the conversion methods:

```rust
use protobrix_rs::*;
use protobrix_rs::TableCellValueExt;

// Create a cell
let cell = TableCellValue::string("hello");

// Extract the value
if let Some(text) = cell.as_string() {
    println!("Cell contains: {}", text);
}

// Type safety - returns None for wrong types
let int_cell = TableCellValue::int(42);
assert_eq!(int_cell.as_string(), None);  // Wrong type
assert_eq!(int_cell.as_int(), Some(42)); // Correct type

// Working with u64
let id_cell = TableCellValue::int_u64(12345);
if let Some(id) = id_cell.as_u64() {
    println!("ID: {}", id);
}

// Negative values return None for u64
let negative = TableCellValue::int(-5);
assert_eq!(negative.as_u64(), None);
assert_eq!(negative.as_int(), Some(-5));
```

### Pattern Matching with Conversions

```rust
fn process_cell(cell: &TableCellValue) {
    if let Some(text) = cell.as_string() {
        println!("String: {}", text);
    } else if let Some(num) = cell.as_int() {
        println!("Number: {}", num);
    } else if let Some(flag) = cell.as_bool() {
        println!("Boolean: {}", flag);
    } else {
        println!("Empty or unknown type");
    }
}
```

### Round-Trip Conversions

```rust
// Create and extract in one flow
let original_value = 42u64;
let cell = TableCellValue::int_u64(original_value);
let extracted = cell.as_u64().unwrap();
assert_eq!(original_value, extracted);
```

### Borrowing vs Consuming

```rust
// as_string() borrows and clones the string
let cell = TableCellValue::string("hello");
let borrowed = cell.as_string().unwrap();
// Can still use cell after as_string
let borrowed_again = cell.as_string().unwrap();

// into_string() consumes the value (no clone)
let cell2 = TableCellValue::string("world");
let owned = cell2.into_string().unwrap();
// cell2 is now consumed and cannot be used again
// This is more efficient when you don't need the cell anymore
```

## Features

- **chrono**: Enables the `datetime()` helper method. Enabled by default.

To disable chrono support:

```toml
[dependencies]
protobrix-rs = { version = "0.1.0", default-features = false, features = ["actix"] }
```
