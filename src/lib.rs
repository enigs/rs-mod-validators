use nulls::Null;
use regex::Regex;
use sizes::Size;
use serde_json::{Map, Value};

const MIN: usize = 8;
const MAX: usize = 64;

/// A versatile `Validator` for validating and enforcing constraints on various fields.
///
/// This struct provides a fluent interface to configure and validate fields of different types,
/// supporting constraints like case sensitivity, nullability, and length, as well as value-specific
/// validations such as email format and password strength.
#[derive(Clone, Default)]
pub struct Validator {
    pub field: String,
    pub min: Option<usize>,
    pub fmin: Option<f64>,
    pub fmax: Option<f64>,
    pub max: Option<usize>,
    pub len: Option<usize>,
    pub option_list_string: Option<Vec<String>>,
    pub is_case_sensitive: bool,
    pub is_null: bool,
    pub is_required: bool,
    pub i32_value: Option<i32>,
    pub i64_value: Option<i64>,
    pub f32_value: Option<f32>,
    pub f64_value: Option<f64>,
    pub string_value: String,
    pub parent_string: String,
    pub list_sizes_value: Vec<Size>
}


/// Creates a new `Validator` instance for the specified field.
///
/// # Arguments
/// * `field` - The field value to validate, convertible to a string.
pub fn new<T>(field: T) -> Validator
where
    T: ToString
{
    Validator::new(field)
}

impl Validator {
    /// Creates a new `Validator` instance for the specified field.
    ///
    /// # Arguments
    /// * `field` - The field value to validate, convertible to a string.
    pub fn new<T>(field: T) -> Self
    where
        T: ToString
    {
        Validator {
            field: field.to_string(),
            is_required: false,
            is_null: false,
            ..Default::default()
        }
    }

    /// Configures whether the validation should consider case sensitivity.
    ///
    /// # Arguments
    /// * `is_case_sensitive` - A boolean indicating if the validation is case-sensitive.
    pub fn set_as_case_sensitive(mut self, is_case_sensitive: bool) -> Self {
        self.is_case_sensitive = is_case_sensitive;
        self
    }

    /// Configures whether the field is allowed to be null.
    ///
    /// # Arguments
    /// * `is_null` - A boolean indicating if the field can be null.
    pub fn set_as_nullable(mut self, is_null: bool) -> Self {
        self.is_null = is_null;
        self
    }

    /// Configures whether the field is required.
    ///
    /// # Arguments
    /// * `is_required` - A boolean indicating if the field is mandatory.
    pub fn set_as_required(mut self, is_required: bool) -> Self {
        self.is_required = is_required;
        self
    }

    /// Sets the field's value as a nullable `i32`.
    ///
    /// # Arguments
    /// * `int32` - The nullable `i32` value.
    pub fn set_i32_value(mut self, int32: &Null<i32>) -> Self {
        self.i32_value = int32.take();
        self
    }

    /// Sets the `i64` value for the validator.
    ///
    /// # Arguments
    /// * `int64` - A nullable `i64` value to set. If `Null::Undefined` or `Null::Null`, it defaults to `0`.
    pub fn set_i64_value(mut self, int64: &Null<i64>) -> Self {
        self.i64_value = int64.take();
        self
    }

    /// Sets the field's value as a nullable `f32`.
    ///
    /// # Arguments
    /// * `float32` - The nullable `f32` value.
    pub fn set_f32_value(mut self, float32: &Null<f32>) -> Self {
        self.f32_value = float32.take();
        self
    }

    /// Sets the `f64` value for the validator.
    ///
    /// # Arguments
    /// * `float64` - A nullable `f64` value to set. If `Null::Undefined` or `Null::Null`, it defaults to `0`.
    pub fn set_f64_value(mut self, float64: &Null<f64>) -> Self {
        self.f64_value = float64.take();
        self
    }

    /// Sets a fixed length constraint for the field.
    ///
    /// # Arguments
    /// * `len` - The exact length the field must match.
    pub fn set_len(mut self, len: usize) -> Self {
        self.len = Some(len);
        self
    }

    /// Sets the minimum value constraint for the field.
    ///
    /// # Arguments
    /// * `min` - The minimum value allowed.
    pub fn set_min(mut self, min: usize) -> Self {
        self.min = Some(min);
        self
    }

    /// Sets the f64 minimum value constraint for the field.
    ///
    /// # Arguments
    /// * `min` - The minimum value allowed.
    pub fn set_fmin(mut self, fmin: f64) -> Self {
        self.fmin = Some(fmin);
        self
    }

    /// Sets the list sizes value for the validator.
    ///
    /// # Arguments
    /// * `list_sizes` - A nullable vector of `Size` values. Defaults to an empty vector if `Null::Undefined` or `Null::Null`.
    pub fn set_list_sizes_value(&mut self, list_sizes: &Null<Vec<Size>>) -> &mut Self {
        self.list_sizes_value = list_sizes.clone().take().unwrap_or_default();
        self
    }

    /// Sets the maximum value constraint for the field.
    ///
    /// # Arguments
    /// * `max` - The maximum value allowed.
    pub fn set_max(mut self, max: usize) -> Self {
        self.max = Some(max);
        self
    }

    /// Sets the maximum f64 value constraint for the field.
    ///
    /// # Arguments
    /// * `max` - The maximum value allowed.
    pub fn set_fmax(mut self, max: f64) -> Self {
        self.fmax = Some(max);
        self
    }

    /// Sets a list of string options.
    ///
    /// # Arguments
    /// * `option_list_string` - A slice of items convertible to strings.
    pub fn set_option_list<T>(mut self, option_list_string: &[T]) -> Self
    where T: ToString
    {
        self.option_list_string = Some(option_list_string
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>());

        self
    }

    /// Sets a list of string options, converting all entries to lowercase.
    ///
    /// # Arguments
    /// * `option_list_string` - A slice of items convertible to strings.
    pub fn set_option_list_lower<T>(mut self, option_list_string: &[T]) -> Self
    where T: ToString
    {
        self.option_list_string = Some(option_list_string
            .iter()
            .map(|value| value.to_string().to_lowercase())
            .collect::<Vec<String>>());

        self
    }

    /// Sets a list of string options, preserving their original case.
    ///
    /// # Arguments
    /// * `option_list_string` - A slice of items convertible to strings.
    pub fn set_option_list_string<T>(mut self, option_list_string: &[T]) -> Self
    where T: ToString
    {
        self.option_list_string = Some(option_list_string
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>());

        self
    }

    /// Sets the string value for the validator, defaulting to an empty string if null or undefined.
    ///
    /// # Arguments
    /// * `string` - A nullable `String` value.
    pub fn set_string_value(mut self, string: &Null<String>) -> Self {
        self.string_value = string.clone().take().unwrap_or_default().to_string();
        self
    }

    /// Sets the string value for the validator, converting it to lowercase.
    ///
    /// # Arguments
    /// * `string` - A nullable `String` value.
    pub fn set_string_value_lower(mut self, string: &Null<String>) -> Self {
        self.string_value = string.clone().take().unwrap_or_default().to_string().to_lowercase();
        self
    }

    /// Sets the string value for the validator, defaulting to an empty string if null or undefined.
    ///
    /// # Arguments
    /// * `string` - A nullable `String` value.
    pub fn set_parent_string<T>(mut self, string: T) -> Self
    where T: ToString
    {
        self.parent_string = string.to_string();
        self
    }



    /// Validates that the string value is a valid Base64-encoded string of the specified length.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is required but empty, or if the Base64-decoded length does not match the specified length.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_b64_bytes(&self) -> Null<String> {
        if self.is_required && self.string_value.is_empty() {
            return nulls::new(i18n::get(format!("{}-invalid", self.field)));
        }

        if let Some(len) = self.len {
            if let Ok(signing) = base64_url::decode(&self.string_value) {
                if len != signing.len() {
                    return nulls::new(i18n::new(format!("{}-len", self.field))
                        .set_args("len", len)
                        .build());
                }
            }
        }

        nulls::undefined()
    }

    /// Validates that the string value is a properly formatted email address.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is empty or the email format is invalid.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_email(&self) -> Null<String> {
        if self.string_value.is_empty() {
            return Null::Value(i18n::get(format!("{}-empty", self.field)));
        }

        if !mailchecker::is_valid(&self.string_value) {
            return Null::Value(i18n::get(format!("{}-invalid", self.field)));
        }

        Null::Undefined
    }

    /// Validates that the `i32` value meets the configured constraints.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is required but empty, or if it violates the minimum/maximum constraints.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_i32(&self) -> Null<String> {
        if self.is_required && self.i32_value.is_none() {
            return Null::Value(i18n::get(format!("{}-empty", self.field)));
        }

        if let (Some(min), Some(max), Some(value)) = (self.min, self.max, self.i32_value) {
            if self.is_required && value < min as i32 && value > max as i32 {
                return Null::Value(i18n::new(format!("{}-min-max", self.field))
                    .set_args("min", min.to_string())
                    .set_args("max", max.to_string())
                    .build());
            }
        }

        if let (Some(min), Some(value)) = (self.min, self.i32_value) {
            if self.is_required && value < min as i32 {
                return Null::Value(i18n::new(format!("{}-min", self.field))
                    .set_args("min", min.to_string())
                    .build());
            }
        }

        if let (Some(max), Some(value)) = (self.max, self.i32_value) {
            if self.is_required && value > max as i32 {
                return Null::Value(i18n::new(format!("{}-max", self.field))
                    .set_args("max", max.to_string())
                    .build());
            }
        }

        Null::Undefined
    }

    /// Validates that the `i64` value meets the configured constraints.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is required but empty, or if it violates the minimum/maximum constraints.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_i64(&self) -> Null<String> {
        if self.is_required && self.i64_value.is_none() {
            return Null::Value(i18n::get(format!("{}-empty", self.field)));
        }

        if let (Some(min), Some(max), Some(value)) = (self.min, self.max, self.i64_value) {
            if self.is_required && value < min as i64 && value > max as i64 {
                return Null::Value(i18n::new(format!("{}-min-max", self.field))
                    .set_args("min", min.to_string())
                    .set_args("max", max.to_string())
                    .build());
            }
        }

        if let (Some(min), Some(value)) = (self.min, self.i64_value) {
            if self.is_required && value < min as i64 {
                return Null::Value(i18n::new(format!("{}-min", self.field))
                    .set_args("min", min.to_string())
                    .build());
            }
        }

        if let (Some(max), Some(value)) = (self.max, self.i64_value) {
            if self.is_required && value > max as i64 {
                return Null::Value(i18n::new(format!("{}-max", self.field))
                    .set_args("max", max.to_string())
                    .build());
            }
        }

        Null::Undefined
    }

    /// Validates that the `f32` value meets the configured constraints.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is required but empty, or if it violates the minimum/maximum constraints.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_f32(&self) -> Null<String> {
        if self.is_required && self.f32_value.is_none() {
            return Null::Value(i18n::get(format!("{}-empty", self.field)));
        }

        if let (Some(min), Some(max), Some(value)) = (self.fmin, self.fmax, self.f32_value) {
            if self.is_required && value < min as f32 && value > max as f32 {
                return Null::Value(i18n::new(format!("{}-min-max", self.field))
                    .set_args("min", min.to_string())
                    .set_args("max", max.to_string())
                    .build());
            }
        }

        if let (Some(min), Some(value)) = (self.fmin, self.f32_value) {
            if self.is_required && value < min as f32 {
                return Null::Value(i18n::new(format!("{}-min", self.field))
                    .set_args("min", min.to_string())
                    .build());
            }
        }

        if let (Some(max), Some(value)) = (self.fmax, self.f32_value) {
            if self.is_required && value > max as f32 {
                return Null::Value(i18n::new(format!("{}-max", self.field))
                    .set_args("max", max.to_string())
                    .build());
            }
        }

        Null::Undefined
    }

    /// Validates that the `f64` value meets the configured constraints.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is required but empty, or if it violates the minimum/maximum constraints.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_f64(&self) -> Null<String> {
        if self.is_required && self.f64_value.is_none() {
            return Null::Value(i18n::get(format!("{}-empty", self.field)));
        }

        if let (Some(min), Some(max), Some(value)) = (self.fmin, self.fmax, self.f64_value) {
            if self.is_required && value < min && value > max {
                return Null::Value(i18n::new(format!("{}-min-max", self.field))
                    .set_args("min", min.to_string())
                    .set_args("max", max.to_string())
                    .build());
            }
        }

        if let (Some(min), Some(value)) = (self.fmin, self.f64_value) {
            if self.is_required && value < min {
                return Null::Value(i18n::new(format!("{}-min", self.field))
                    .set_args("min", min.to_string())
                    .build());
            }
        }

        if let (Some(max), Some(value)) = (self.fmax, self.f64_value) {
            if self.is_required && value > max {
                return Null::Value(i18n::new(format!("{}-max", self.field))
                    .set_args("max", max.to_string())
                    .build());
            }
        }

        Null::Undefined
    }

    /// Validates that the list of sizes meets the required format and constraints.
    ///
    /// # Returns
    /// * `Null::Value` - A list of error messages if the field is empty or contains invalid size entries.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_list_sizes(&self) -> Null<Vec<String>> {
        let mut errors = Vec::new();

        if self.is_required && self.list_sizes_value.is_empty() {
            errors.push(i18n::get(format!("{}-empty", self.field)).to_string());
        }

        if self.is_required && !self.list_sizes_value.is_empty() {
            for size in self.list_sizes_value.clone() {
                let size_scale = ["XXSM", "XSM", "SM", "MD", "LG", "XLG", "XXLG"];
                let size_type = ["THUMBNAIL", "LANDSCAPE", "PORTRAIT"];

                let has_scale = size_scale.contains(&size.scale.to_string().as_str());
                let has_type = size_type.contains(&size.orientation.to_string().as_str());
                let has_width = size.width > 0;
                let has_height = size.height > 0;

                if !has_scale || !has_type || !has_width || !has_height {
                    errors.push(i18n::new(format!("{}-invalid", self.field))
                        .set_args("entry", serde_json::to_string(&size).unwrap_or_default().as_str())
                        .build())
                }
            }
        }

        if errors.is_empty() {
            return Null::Undefined;
        }

        Null::Value(errors)
    }

    /// Validates that the string value matches one of the allowed options in the list.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is required but empty, or if the value is not in the list.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_list_string(&self) -> Null<String> {
        if self.is_required && self.string_value.is_empty() {
            return Null::Value(i18n::get(format!("{}-empty", self.field)));
        }

        if let Some(list) = self.option_list_string.clone() {
            match self.is_case_sensitive {
                true => if self.is_required && !list.contains(&self.string_value) {
                    return Null::Value(i18n::get(format!("{}-invalid", self.field)));
                }
                false => {
                    let list = list.iter()
                        .map(|value| value.to_lowercase())
                        .collect::<Vec<String>>();

                    if self.is_required && !list.contains(&self.string_value.to_lowercase()) {
                        return Null::Value(i18n::get(format!("{}-invalid", self.field)));
                    }
                }
            }
        }

        Null::Undefined
    }

    /// Validates that the string value matches one of the allowed options in the list.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is required but empty, or if the value is not in the list.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_list_options(&self) -> Null<String> {
        if self.is_required && self.string_value.is_empty()  {
            return Null::Value(i18n::get(format!("{}-empty", self.field)));
        }

        let wrapped_items: Vec<String> = self
            .option_list_string
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|item| format!("❛{}❜", item)) // Wrap each item in ❛❜
            .collect();

        let args = if wrapped_items.len() > 1 {
            let last = wrapped_items.last().unwrap(); // Get the last element
            let others = &wrapped_items[..wrapped_items.len() - 1]; // All but the last
            format!("{} and {}", others.join(", "), last)
        } else {
            wrapped_items.join("") // Handles single or empty case
        };

        let parent = self.parent_string.clone();

        if let Some(list) = self.option_list_string.clone() {
            match self.is_case_sensitive {
                true => if self.is_required && !list.contains(&self.string_value) {
                    return if parent.is_empty() {
                        nulls::new(i18n::new(format!("{}-invalid", self.field))
                            .set_args("options", args.clone())
                            .build())
                    } else {
                        nulls::new(i18n::new(format!("{}-invalid", self.field))
                            .set_args("options", args.clone())
                            .set_args("parent", parent.clone())
                            .build())
                    }
                }
                false => {
                    let list = list.iter()
                        .map(|value| value.to_lowercase())
                        .collect::<Vec<String>>();

                    if self.is_required && !list.contains(&self.string_value.to_lowercase()) {
                        return if parent.is_empty() {
                            nulls::new(i18n::new(format!("{}-invalid", self.field))
                                .set_args("options", args.clone())
                                .build())
                        } else {
                            nulls::new(i18n::new(format!("{}-invalid", self.field))
                                .set_args("options", args.clone())
                                .set_args("parent", parent.clone())
                                .build())
                        }
                    }
                }
            }
        }

        Null::Undefined
    }

    /// Validates that the string value is a valid name format, containing only letters, spaces, and certain special characters.
    ///
    /// # Returns
    /// * `Null::Value` - If the field is required but empty, or if the value does not match the valid name pattern.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_name(&self) -> Null<String> {
        let value = self.validate_string();
        if value.is_some() {
            return value;
        }

        match Regex::new(r"^[\p{L} \-・']+$") {
            Ok(re) => if !re.is_match(&self.string_value) {
                return Null::Value(i18n::get(format!("{}-invalid", self.field)));
            },
            _ => return Null::Value(i18n::get(format!("{}-invalid", self.field)))
        }

        Null::Undefined
    }

    /// Validates that the string value meets basic password requirements.
    ///
    /// # Returns
    /// * `Null::Value` - If the validation fails.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_password_simple(&self) -> Null<String> {
        self.validate_string()
    }

    /// Validates that the string value meets strict password complexity requirements.
    ///
    /// # Returns
    /// * `Null::Value` - A map of errors detailing which requirements (minimum length, maximum length, presence of uppercase, lowercase, numbers, or symbols) were not met.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_password_strict(&self) -> Null<Value> {
        let length = self.string_value.len();
        let mut errors = Map::new();

        if length < MIN {
            errors.insert(
                "minimum".into(),
                i18n::new(format!("{}-minimum", self.field))
                    .set_args("min", MIN)
                    .build()
                    .into()
            );
        }

        if length > MAX {
            errors.insert(
                "maximum".into(),
                i18n::new(format!("{}-maximum", self.field))
                    .set_args("max", MAX)
                    .build()
                    .into()
            );
        }

        if !self.string_value
            .clone()
            .bytes()
            .any(|b| b.is_ascii_lowercase()) {
            errors.insert(
                "lowercase".into(),
                i18n::get(format!("{}-lowercase", self.field)).into()
            );
        }

        if !self.string_value
            .clone()
            .bytes()
            .any(|b| b.is_ascii_uppercase()) {
            errors.insert(
                "uppercase".into(),
                i18n::get(format!("{}-uppercase", self.field)).into()
            );
        }

        if self.string_value
            .clone()
            .chars()
            .all(|x| x.is_ascii_alphabetic()) {
            errors.insert(
                "number".into(),
                i18n::get(format!("{}-number", self.field)).into()
            );
        }

        if self.string_value
            .clone()
            .chars()
            .all(|x| x.is_ascii_alphanumeric()) {
            errors.insert(
                "symbol".into(),
                i18n::get(format!("{}-symbol", self.field)).into()
            );
        }

        if !errors.is_empty() {
            return Null::Value(Value::Object(errors));
        }

        Null::Undefined
    }

    /// Validates that the string value meets length constraints and is not empty.
    ///
    /// # Returns
    /// * `Null::Value` - If the string is empty or violates the minimum/maximum length constraints.
    /// * `Null::Undefined` - If the validation passes successfully.
    pub fn validate_string(&self) -> Null<String> {
        // Check if string is empty
        if self.string_value.is_empty() {
            return Null::Value(i18n::get(format!("{}-empty", self.field)));
        }

        match () {
            _ if self.min.is_some() && self.max.is_some() => {
                let min = self.min.unwrap();
                let max = self.max.unwrap();
                let len = self.string_value.len();

                match () {
                    _ if len < min && len > max => {
                        Null::Value(i18n::new(format!("{}-min-max", self.field))
                            .set_args("min", min.to_string())
                            .set_args("max", max.to_string())
                            .build())
                    },
                    _ if len < min => {
                        Null::Value(i18n::new(format!("{}-min", self.field))
                            .set_args("min", min.to_string())
                            .build())
                    },
                    _ if len > max => {
                        Null::Value(i18n::new(format!("{}-max", self.field))
                            .set_args("max", max.to_string())
                            .build())
                    },
                    _ => Null::Undefined
                }
            },
            _ if self.min.is_some() && self.max.is_none() => {
                let min = self.min.unwrap();
                let len = self.string_value.len();

                if len < min {
                    return Null::Value(i18n::new(format!("{}-min", self.field))
                        .set_args("min", min.to_string())
                        .build());
                }

                Null::Undefined
            },
            _ if self.min.is_none() && self.max.is_some() => {
                let max = self.max.unwrap();
                let len = self.string_value.len();

                if len > max {
                    return Null::Value(i18n::new(format!("{}-max", self.field))
                        .set_args("max", max.to_string())
                        .build());
                };

                Null::Undefined
            },
            _ => Null::Undefined
        }
    }
}