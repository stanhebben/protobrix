use std::mem;

use crate::proto::*;

/// Extension trait for creating TableCellValue instances with convenience methods
pub trait TableCellValueExt {
    /// Create a TableCellValue from a string
    fn string(value: impl Into<String>) -> TableCellValue;

    /// Create a TableCellValue from an integer
    fn int(value: i32) -> TableCellValue;

    /// Create a TableCellValue from a u64 (converts to i32)
    fn int_u64(value: u64) -> TableCellValue;

    /// Create a TableCellValue from a double (f64)
    fn double(value: f64) -> TableCellValue;

    /// Create a TableCellValue from a float (f32)
    fn float(value: f32) -> TableCellValue;

    /// Create a TableCellValue from a boolean
    fn bool(value: bool) -> TableCellValue;

    /// Create an empty TableCellValue (None)
    fn empty() -> TableCellValue;

    /// Create a TableCellValue from an Option<String>
    fn opt_string(value: Option<String>) -> TableCellValue;

    /// Create a TableCellValue from an Option<i32>
    fn opt_int(value: Option<i32>) -> TableCellValue;

    /// Create a TableCellValue from an Option<f64>
    fn opt_double(value: Option<f64>) -> TableCellValue;

    /// Create a TableCellValue from an Option<f32>
    fn opt_float(value: Option<f32>) -> TableCellValue;

    /// Create a TableCellValue from an Option<bool>
    fn opt_bool(value: Option<bool>) -> TableCellValue;

    /// Create a TableCellValue from a formatted datetime
    /// Requires the "chrono" feature to be enabled
    #[cfg(feature = "chrono")]
    fn datetime(dt: Option<chrono::NaiveDateTime>) -> TableCellValue;

    /// Create a TableCellValue from a string and take ownership of the string
    fn take_string(value: &mut String) -> TableCellValue;

    /// Create a TableCellValue from an Option<String> and take ownership of the string
    fn take_opt_string(value: &mut Option<String>) -> TableCellValue;

    // Conversion methods (extracting values from TableCellValue)

    /// Extract as Option<String>. Returns None if not a string value.
    fn as_string(&self) -> Option<String>;

    /// Extract as Option<i32>. Returns None if not an int value.
    fn as_int(&self) -> Option<i32>;

    /// Extract as Option<u64>. Returns None if not an int value or if negative.
    fn as_u64(&self) -> Option<u64>;

    /// Extract as Option<f64>. Returns None if not a double value.
    fn as_double(&self) -> Option<f64>;

    /// Extract as Option<f32>. Returns None if not a float value.
    fn as_float(&self) -> Option<f32>;

    /// Extract as Option<bool>. Returns None if not a boolean value.
    fn as_bool(&self) -> Option<bool>;

    /// Consume the value and extract as Option<String>. Returns None if not a string value.
    fn into_string(self) -> Option<String>;
}

impl TableCellValueExt for TableCellValue {
    fn string(value: impl Into<String>) -> TableCellValue {
        TableCellValue {
            value: Some(table_cell_value::Value::StringValue(value.into())),
        }
    }

    fn int(value: i32) -> TableCellValue {
        TableCellValue {
            value: Some(table_cell_value::Value::IntValue(value)),
        }
    }

    fn int_u64(value: u64) -> TableCellValue {
        TableCellValue {
            value: Some(table_cell_value::Value::IntValue(value as i32)),
        }
    }

    fn double(value: f64) -> TableCellValue {
        TableCellValue {
            value: Some(table_cell_value::Value::DoubleValue(value)),
        }
    }

    fn float(value: f32) -> TableCellValue {
        TableCellValue {
            value: Some(table_cell_value::Value::FloatValue(value)),
        }
    }

    fn bool(value: bool) -> TableCellValue {
        TableCellValue {
            value: Some(table_cell_value::Value::BooleanValue(value)),
        }
    }

    fn empty() -> TableCellValue {
        TableCellValue { value: None }
    }

    fn opt_string(value: Option<String>) -> TableCellValue {
        TableCellValue {
            value: value.map(table_cell_value::Value::StringValue),
        }
    }

    fn opt_int(value: Option<i32>) -> TableCellValue {
        TableCellValue {
            value: value.map(table_cell_value::Value::IntValue),
        }
    }

    fn opt_double(value: Option<f64>) -> TableCellValue {
        TableCellValue {
            value: value.map(table_cell_value::Value::DoubleValue),
        }
    }

    fn opt_float(value: Option<f32>) -> TableCellValue {
        TableCellValue {
            value: value.map(table_cell_value::Value::FloatValue),
        }
    }

    fn opt_bool(value: Option<bool>) -> TableCellValue {
        TableCellValue {
            value: value.map(table_cell_value::Value::BooleanValue),
        }
    }

    #[cfg(feature = "chrono")]
    fn datetime(dt: Option<chrono::NaiveDateTime>) -> TableCellValue {
        let date_str = dt
            .map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string())
            .unwrap_or_default();
        TableCellValue {
            value: Some(table_cell_value::Value::StringValue(date_str)),
        }
    }

    fn take_string(value: &mut String) -> TableCellValue {
        TableCellValue {
            value: Some(table_cell_value::Value::StringValue(mem::take(value))),
        }
    }

    fn take_opt_string(value: &mut Option<String>) -> TableCellValue {
        TableCellValue::opt_string(mem::take(value))
    }

    fn as_string(&self) -> Option<String> {
        match &self.value {
            Some(table_cell_value::Value::StringValue(s)) => Some(s.clone()),
            _ => None,
        }
    }

    fn as_int(&self) -> Option<i32> {
        match &self.value {
            Some(table_cell_value::Value::IntValue(i)) => Some(*i),
            _ => None,
        }
    }

    fn as_u64(&self) -> Option<u64> {
        match &self.value {
            Some(table_cell_value::Value::IntValue(i)) if *i >= 0 => Some(*i as u64),
            _ => None,
        }
    }

    fn as_double(&self) -> Option<f64> {
        match &self.value {
            Some(table_cell_value::Value::DoubleValue(d)) => Some(*d),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f32> {
        match &self.value {
            Some(table_cell_value::Value::FloatValue(f)) => Some(*f),
            _ => None,
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match &self.value {
            Some(table_cell_value::Value::BooleanValue(b)) => Some(*b),
            _ => None,
        }
    }

    fn into_string(self) -> Option<String> {
        match self.value {
            Some(table_cell_value::Value::StringValue(s)) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_cell() {
        let cell = TableCellValue::string("test");
        assert!(matches!(
            cell.value,
            Some(table_cell_value::Value::StringValue(_))
        ));
    }

    #[test]
    fn test_int_cell() {
        let cell = TableCellValue::int(42);
        assert_eq!(cell.value, Some(table_cell_value::Value::IntValue(42)));
    }

    #[test]
    fn test_int_u64_cell() {
        let cell = TableCellValue::int_u64(100);
        assert_eq!(cell.value, Some(table_cell_value::Value::IntValue(100)));
    }

    #[test]
    fn test_double_cell() {
        let cell = TableCellValue::double(3.14);
        assert_eq!(cell.value, Some(table_cell_value::Value::DoubleValue(3.14)));
    }

    #[test]
    fn test_float_cell() {
        let cell = TableCellValue::float(2.5);
        assert_eq!(cell.value, Some(table_cell_value::Value::FloatValue(2.5)));
    }

    #[test]
    fn test_bool_cell() {
        let cell = TableCellValue::bool(true);
        assert_eq!(
            cell.value,
            Some(table_cell_value::Value::BooleanValue(true))
        );
    }

    #[test]
    fn test_empty_cell() {
        let cell = TableCellValue::empty();
        assert_eq!(cell.value, None);
    }

    #[test]
    fn test_opt_string_some() {
        let cell = TableCellValue::opt_string(Some("test".to_string()));
        assert!(matches!(
            cell.value,
            Some(table_cell_value::Value::StringValue(_))
        ));
    }

    #[test]
    fn test_opt_string_none() {
        let cell = TableCellValue::opt_string(None);
        assert_eq!(cell.value, None);
    }

    // Tests for conversion methods

    #[test]
    fn test_as_string() {
        let cell = TableCellValue::string("hello");
        assert_eq!(cell.as_string(), Some("hello".to_string()));

        let int_cell = TableCellValue::int(42);
        assert_eq!(int_cell.as_string(), None);

        let empty_cell = TableCellValue::empty();
        assert_eq!(empty_cell.as_string(), None);
    }

    #[test]
    fn test_as_int() {
        let cell = TableCellValue::int(42);
        assert_eq!(cell.as_int(), Some(42));

        let string_cell = TableCellValue::string("42");
        assert_eq!(string_cell.as_int(), None);

        let empty_cell = TableCellValue::empty();
        assert_eq!(empty_cell.as_int(), None);
    }

    #[test]
    fn test_as_u64() {
        let cell = TableCellValue::int(42);
        assert_eq!(cell.as_u64(), Some(42u64));

        let negative_cell = TableCellValue::int(-5);
        assert_eq!(negative_cell.as_u64(), None);

        let string_cell = TableCellValue::string("42");
        assert_eq!(string_cell.as_u64(), None);

        let empty_cell = TableCellValue::empty();
        assert_eq!(empty_cell.as_u64(), None);
    }

    #[test]
    fn test_as_double() {
        let cell = TableCellValue::double(3.14);
        assert_eq!(cell.as_double(), Some(3.14));

        let int_cell = TableCellValue::int(42);
        assert_eq!(int_cell.as_double(), None);

        let empty_cell = TableCellValue::empty();
        assert_eq!(empty_cell.as_double(), None);
    }

    #[test]
    fn test_as_float() {
        let cell = TableCellValue::float(2.5);
        assert_eq!(cell.as_float(), Some(2.5));

        let int_cell = TableCellValue::int(42);
        assert_eq!(int_cell.as_float(), None);

        let empty_cell = TableCellValue::empty();
        assert_eq!(empty_cell.as_float(), None);
    }

    #[test]
    fn test_as_bool() {
        let cell_true = TableCellValue::bool(true);
        assert_eq!(cell_true.as_bool(), Some(true));

        let cell_false = TableCellValue::bool(false);
        assert_eq!(cell_false.as_bool(), Some(false));

        let int_cell = TableCellValue::int(1);
        assert_eq!(int_cell.as_bool(), None);

        let empty_cell = TableCellValue::empty();
        assert_eq!(empty_cell.as_bool(), None);
    }

    #[test]
    fn test_round_trip_conversions() {
        // String round trip
        let original_str = "test value";
        let cell = TableCellValue::string(original_str);
        assert_eq!(cell.as_string(), Some(original_str.to_string()));

        // Int round trip
        let original_int = 123;
        let cell = TableCellValue::int(original_int);
        assert_eq!(cell.as_int(), Some(original_int));

        // U64 round trip
        let original_u64 = 456u64;
        let cell = TableCellValue::int_u64(original_u64);
        assert_eq!(cell.as_u64(), Some(original_u64));

        // Double round trip
        let original_double = 3.14159;
        let cell = TableCellValue::double(original_double);
        assert_eq!(cell.as_double(), Some(original_double));

        // Float round trip
        let original_float = 2.718f32;
        let cell = TableCellValue::float(original_float);
        assert_eq!(cell.as_float(), Some(original_float));

        // Bool round trip
        let cell = TableCellValue::bool(true);
        assert_eq!(cell.as_bool(), Some(true));
    }

    #[test]
    fn test_into_string() {
        // Consuming string value
        let cell = TableCellValue::string("hello world");
        assert_eq!(cell.into_string(), Some("hello world".to_string()));

        // Consuming non-string value returns None
        let int_cell = TableCellValue::int(42);
        assert_eq!(int_cell.into_string(), None);

        // Consuming empty cell returns None
        let empty_cell = TableCellValue::empty();
        assert_eq!(empty_cell.into_string(), None);
    }

    #[test]
    fn test_into_string_vs_as_string() {
        // as_string borrows and clones
        let cell = TableCellValue::string("test");
        let borrowed = cell.as_string();
        assert_eq!(borrowed, Some("test".to_string()));
        // Can still use cell after as_string
        assert_eq!(cell.as_string(), Some("test".to_string()));

        // into_string consumes the value
        let cell2 = TableCellValue::string("test2");
        let owned = cell2.into_string();
        assert_eq!(owned, Some("test2".to_string()));
        // cell2 is now consumed and can't be used
    }
}
