# serini [![Crates.io](https://img.shields.io/crates/v/serini.svg)](https://crates.io/crates/serini)

A serde-based INI file parser for Rust that supports automatic serialization and deserialization of structs to INI format.

## Features

- 🚀 **Automatic serialization** - Convert Rust structs to INI format
- 🔧 **Type-safe deserialization** - Parse INI files into strongly-typed Rust structs
- 📁 **Section support** - Nested structs become INI sections automatically
- 💭 **Option handling** - `None` values are serialized as commented lines if serialization is not skipped using `#[serde(skip)]` or similiar
- 🛡️ **Escape sequences** - Properly handles special characters in values
- 🏷️ **Serde integration** - Supports serde attributes like `#[serde(rename = "...")]`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serini = "0.1"
```

## Quick Example

```rust
use serde::{Deserialize, Serialize};
use serini::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    debug: Option<usize>,
    database: Database,
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    host: String,
    username: String,
    password: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        name: "My App".to_string(),
        port: 8080,
        debug: None,
        database: Database {
            host: "localhost".to_string(),
            username: "admin".to_string(),
            password: None,
        },
    };

    // Serialize to INI
    let ini_string = to_string(&config)?;
    println!("{}", ini_string);

    // Deserialize from INI
    let parsed: Config = from_str(&ini_string)?;
    assert_eq!(config, parsed);
    Ok(())
}
```

Output:

```ini
name = My App
port = 8080

[database]
host = localhost
username = admin
; password = 
```

## Usage Guide

### Basic Types

serini supports all basic Rust types:

```rust
#[derive(Serialize, Deserialize)]
struct Settings {
    // Integers
    max_connections: u32,      // max_connections = 100
    retry_count: i8,          // retry_count = 3
    
    // Floats
    timeout: f64,             // timeout = 30.5
    
    // Booleans
    debug_mode: bool,         // debug_mode = true
    
    // Strings
    server_name: String,      // server_name = Production Server
    
    // Options
    description: Option<String>, // ; description =  (when None)
}
```

### Sections from Nested Structs

Nested structs automatically become INI sections:

```rust
#[derive(Serialize, Deserialize)]
struct App {
    version: String,
    
    server: ServerConfig,
    database: DbConfig,
}

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct DbConfig {
    url: String,
    pool_size: u32,
}
```

Produces:

```ini
version = 1.0.0

[server]
host = 0.0.0.0
port = 8080

[database]
url = postgres://localhost/mydb
pool_size = 10
```

### Self-Referential Structs

serini supports self-referential structs using `Option<Box<T>>`, allowing sections to override values from the root configuration:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    speed: f32,
    movie: Option<Box<Config>>,
    anime: Option<Box<Config>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse from INI
    let ini_str = r#"
speed = 1

[anime]
speed = 1.5

[movie]
speed = 2
"#;

    let config: Config = from_str(ini_str)?;
    
    println!("Default speed: {}", config.speed);
    if let Some(anime_config) = &config.anime {
        println!("Anime speed: {}", anime_config.speed);
    }
    if let Some(movie_config) = &config.movie {
        println!("Movie speed: {}", movie_config.speed);
    }

    // Serialize to INI
    let new_config = Config {
        speed: 1.0,
        anime: Some(Box::new(Config {
            speed: 1.5,
            anime: None,
            movie: None,
        })),
        movie: Some(Box::new(Config {
            speed: 2.0,
            anime: None,
            movie: None,
        })),
    };

    let ini_output = to_string(&new_config)?;
    println!("{}", ini_output);
    Ok(())
}
```

Output:

```ini
speed = 1

[anime]
speed = 1.5

[movie]
speed = 2
```

This pattern is useful for configuration files where different profiles or modes can override default settings.

### Field Renaming

Use serde's rename attribute:

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename = "app-name")]
    app_name: String,
    
    #[serde(rename = "log-level")]
    log_level: String,
}
```

### Escape Sequences

Special characters are automatically escaped:

| Character | Escaped |
|-----------|---------|
| `\` | `\\` |
| newline | `\n` |
| tab | `\t` |
| `"` | `\"` |
| `;` | `\;` |
| `#` | `\#` |

## Real-World Example

```rust
use serde::{Deserialize, Serialize};
use serini::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    #[serde(rename = "app-name")]
    app_name: String,
    environment: String,
    
    http: HttpConfig,
    database: DatabaseConfig,
    redis: Option<RedisConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HttpConfig {
    host: String,
    port: u16,
    #[serde(rename = "request-timeout")]
    request_timeout: u64,
    #[serde(rename = "enable-tls")]
    enable_tls: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    #[serde(rename = "max-connections")]
    max_connections: u32,
    #[serde(rename = "connection-timeout")]
    connection_timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RedisConfig {
    url: String,
    #[serde(rename = "connection-pool")]
    connection_pool: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig {
        app_name: "my-awesome-app".to_string(),
        environment: "production".to_string(),
        
        http: HttpConfig {
            host: "0.0.0.0".to_string(),
            port: 443,
            request_timeout: 30,
            enable_tls: true,
        },
        
        database: DatabaseConfig {
            url: "postgres://user:pass@localhost/mydb".to_string(),
            max_connections: 100,
            connection_timeout: 5,
        },
        
        redis: None,
    };

    let ini = to_string(&config)?;
    println!("{}", ini);
    Ok(())
}
```

Output:

```ini
app-name = my-awesome-app
environment = production

[http]
host = 0.0.0.0
port = 443
request-timeout = 30
enable-tls = true

[database]
url = postgres://user:pass@localhost/mydb
max-connections = 100
connection-timeout = 5

; redis = 
```

## API

```rust
// Serialize a value to INI string
pub fn to_string<T: Serialize>(value: &T) -> Result<String, Error>

// Deserialize from INI string
pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T, Error>
```

## Error Types

serini uses a custom error type with helpful error messages:

```rust
use serini::Error;

match result {
    Err(Error::InvalidValue { typ, value }) => {
        eprintln!("Invalid {} value: {}", typ, value);
    }
    Err(Error::UnsupportedFeature(feature)) => {
        eprintln!("Unsupported feature: {}", feature);
    }
    Err(e) => eprintln!("Error: {}", e),
    Ok(_) => {}
}
```

## Limitations

The following types are **not** supported:

- Sequences (Vec, arrays)
- Tuples and tuple structs  
- Enums with variants
- Maps (HashMap, BTreeMap)
- Nested arrays or complex data structures

## Why serini?

- **Simple** - Minimal API with just two functions
- **Type-safe** - Leverages Rust's type system and serde
- **Automatic** - No manual parsing or writing required
- **Flexible** - Supports Options, escaping, and field renaming
- **Fast** - Zero-copy deserialization where possible

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
