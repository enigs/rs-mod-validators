# rs-mod-validators

A Rust module providing a versatile validation library for enforcing constraints on various field types.

## Overview

The `validators` module offers a fluent interface to configure and validate fields of different types. It supports various validation constraints such as:

- Case sensitivity
- Nullability
- String length requirements
- Numeric range validation
- Special format validation (email, Base64)
- Password strength checking
- Option list validation

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
validators = { git = "https://github.com/enigs/rs-mod-validators" }
```

## Usage

### Basic String Validation

```rust
use validators::{new, Validator};

// Create a new validator for a username field
let result = new("username")
    .set_string_value(&username)
    .set_min(3)
    .set_max(20)
    .validate_string();

if result.is_some() {
    // Handle validation error
    println!("Error: {}", result.unwrap());
}
```

### Email Validation

```rust
let result = new("email")
    .set_string_value(&email)
    .validate_email();

if result.is_some() {
    // Handle invalid email
}
```

### Password Validation

```rust
// Simple password validation
let result = new("password")
    .set_string_value(&password)
    .set_min(8)
    .validate_password_simple();

// Strict password validation (checks complexity requirements)
let strict_result = new("password")
    .set_string_value(&password)
    .validate_password_strict();

if strict_result.is_some() {
    // Access specific validation errors
    let errors = strict_result.unwrap();
    // Check for specific errors like minimum length, uppercase, lowercase, etc.
}
```

### Numeric Validation

```rust
// Integer validation
let result = new("age")
    .set_i32_value(&age)
    .set_min(18)
    .set_max(150)
    .set_as_required(true)
    .validate_i32();

// Float validation
let price_result = new("price")
    .set_f64_value(&price)
    .set_fmin(0.0)
    .set_fmax(1000.0)
    .validate_f64();
```

### Option List Validation

```rust
// Validate against a list of allowed values
let result = new("country")
    .set_string_value(&country)
    .set_option_list(&["USA", "Canada", "Mexico"])
    .set_as_case_sensitive(true)
    .validate_list_string();

// Case-insensitive validation with custom error messages
let result = new("role")
    .set_string_value(&role)
    .set_option_list_lower(&["admin", "user", "guest"])
    .set_parent_string("user management")
    .validate_list_options();
```

## Features

- **Fluent Interface**: Chain method calls for concise and readable code
- **Type Safety**: Strong typing with support for nullability
- **Internationalization**: Error messages can be localized using the `i18n` module
- **Comprehensive Validations**: Cover common use cases out of the box
- **Extensible**: Can be easily extended for custom validation needs

## Examples

### Name Validation

```rust
let result = new("full_name")
    .set_string_value(&name)
    .validate_name();
```

### Base64 Validation

```rust
let result = new("signature")
    .set_string_value(&base64_string)
    .set_len(64)  // Expected decoded length
    .validate_b64_bytes();
```