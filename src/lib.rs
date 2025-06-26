//! # serini
//!
//! A serde-based INI file parser that supports serialization and deserialization of Rust structs.
//!
//! ## Features
//!
//! - **Serialize Rust structs to INI format** - Nested structs become sections
//! - **Deserialize INI files to Rust structs** - Type-safe parsing with automatic type conversion
//! - **Option handling** - [`None`][Option] values are serialized as commented lines
//! - **Escape sequences** - Properly handles special characters in values
//! - **Section support** - Nested structs are automatically converted to INI sections
//! - **Type safety** - Leverages serde's type system for safe conversions
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serini = "0.1"
//! ```
//!
//! ## Basic Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serini::{from_str, to_string};
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct Config {
//!     name: String,
//!     port: u16,
//!     #[serde(skip_serializing_if = "Option::is_none")]
//!     debug: Option<usize>,
//!     database: Database,
//! }
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct Database {
//!     host: String,
//!     port: u16,
//!     username: String,
//!     password: Option<String>,
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config {
//!         name: "My Application".to_string(),
//!         port: 8080,
//!         debug: None,
//!         database: Database {
//!             host: "localhost".to_string(),
//!             port: 5432,
//!             username: "admin".to_string(),
//!             password: None,
//!         },
//!     };
//!
//!     // Serialize to INI
//!     let ini_string = to_string(&config)?;
//!     println!("{}", ini_string);
//!     // Output:
//!     // name = My Application
//!     // port = 8080
//!     //
//!     // [database]
//!     // host = localhost
//!     // port = 5432
//!     // username = admin
//!     // ; password =
//!
//!     // Deserialize from INI
//!     let parsed: Config = from_str(&ini_string)?;
//!     assert_eq!(config, parsed);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Section Handling
//!
//! Nested structs automatically become INI sections:
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serini::to_string;
//!
//! #[derive(Serialize, Deserialize)]
//! struct ServerConfig {
//!     general: General,
//!     http: HttpConfig,
//!     database: DatabaseConfig,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct General {
//!     name: String,
//!     debug: bool,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct HttpConfig {
//!     host: String,
//!     port: u16,
//!     timeout: u64,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct DatabaseConfig {
//!     url: String,
//!     max_connections: u32,
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ServerConfig {
//!     general: General {
//!         name: "MyServer".to_string(),
//!         debug: false,
//!     },
//!     http: HttpConfig {
//!         host: "0.0.0.0".to_string(),
//!         port: 8080,
//!         timeout: 30,
//!     },
//!     database: DatabaseConfig {
//!         url: "postgres://localhost/mydb".to_string(),
//!         max_connections: 100,
//!     },
//! };
//!
//! let ini = to_string(&config)?;
//! # Ok(())
//! # }
//! ```
//!
//! Produces:
//!
//! ```ini
//! [general]
//! name = MyServer
//! debug = false
//!
//! [http]
//! host = 0.0.0.0
//! port = 8080
//! timeout = 30
//!
//! [database]
//! url = postgres://localhost/mydb
//! max_connections = 100
//! ```
//!
//! ## Option Handling
//!
//! `Option<T>` fields are handled specially:
//! - `Some(value)` is serialized normally
//! - `None` is serialized as a commented line
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serini::to_string;
//!
//! #[derive(Serialize, Deserialize)]
//! struct User {
//!     username: String,
//!     email: Option<String>,
//!     age: Option<u32>,
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let user = User {
//!     username: "alice".to_string(),
//!     email: Some("alice@example.com".to_string()),
//!     age: None,
//! };
//!
//! let ini = to_string(&user)?;
//! # Ok(())
//! # }
//! ```
//!
//! Produces:
//!
//! ```ini
//! username = alice
//! email = alice@example.com
//! ; age =
//! ```
//!
//! ## Escape Sequences
//!
//! Special characters in values are automatically escaped:
//!
//! | Character | Escaped |
//! |-----------|---------|
//! | `\` | `\\` |
//! | `\n` | `\n` |
//! | `\r` | `\r` |
//! | `\t` | `\t` |
//! | `"` | `\"` |
//! | `;` | `\;` |
//! | `#` | `\#` |
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serini::{from_str, to_string};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Message {
//!     text: String,
//!     note: String,
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let msg = Message {
//!     text: "Hello\nWorld!".to_string(),
//!     note: "This has \"quotes\" and a ; semicolon".to_string(),
//! };
//!
//! let ini = to_string(&msg)?;
//! // text = Hello\nWorld!
//! // note = This has \"quotes\" and a \; semicolon
//!
//! let parsed: Message = from_str(&ini)?;
//! assert_eq!(msg, parsed);
//! # Ok(())
//! # }
//! ```
//!
//! ## Supported Types
//!
//! The following types are supported for serialization and deserialization:
//!
//! - **Integers**: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`
//! - **Floats**: `f32`, `f64`
//! - **Boolean**: `bool` (serialized as `true`/`false`)
//! - **String**: `String`, `&str`
//! - **Option**: `Option<T>` where `T` is a supported type
//! - **Structs**: Custom structs with named fields
//!
//! ## Limitations
//!
//! The following serde types are **not** supported:
//!
//! - Sequences (Vec, arrays, etc.)
//! - Tuples and tuple structs
//! - Enums with variants
//! - Maps (HashMap, BTreeMap, etc.)
//! - Unit structs
//!
//! Attempting to serialize or deserialize these types will result in an error.
//!
//! ## Error Handling
//!
//! This crate uses an`Error` type using [thiserror] to provide granular error variants:
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serini::{from_str, Error};
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     port: u16,
//! }
//!
//! # fn main() {
//! let ini = "port = not_a_number";
//!
//! match from_str::<Config>(ini) {
//!     Ok(_) => println!("Parsed successfully"),
//!     Err(Error::InvalidValue { typ, value }) => {
//!         println!("Invalid {} value: {}", typ, value);
//!     }
//!     Err(e) => println!("Error: {}", e),
//! }
//! # }
//! ```
//!
//! ## API Reference
//!
//! ### Functions
//!
//! #### [`to_string`]
//!
//! Serializes a value to an INI string.
//!
//! #### [`from_str`]
//!
//! Deserializes an INI string to a value.
//!
//! ## Advanced Example
//!
//! Here's a complete example showing various features:
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serini::{from_str, to_string};
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct AppConfig {
//!     #[serde(rename = "app-name")]
//!     app_name: String,
//!     version: String,
//!     debug_mode: bool,
//!     max_connections: u32,
//!     timeout_seconds: Option<u64>,
//!     
//!     server: ServerSettings,
//!     database: DatabaseSettings,
//!     cache: Option<CacheSettings>,
//! }
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct ServerSettings {
//!     host: String,
//!     port: u16,
//!     #[serde(rename = "use-tls")]
//!     use_tls: bool,
//!     certificate_path: Option<String>,
//! }
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct DatabaseSettings {
//!     #[serde(rename = "connection-string")]
//!     connection_string: String,
//!     pool_size: u32,
//!     timeout: u32,
//! }
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct CacheSettings {
//!     backend: String,
//!     ttl_seconds: u64,
//!     max_entries: u64,
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a configuration
//! let config = AppConfig {
//!     app_name: "MyApp".to_string(),
//!     version: "1.0.0".to_string(),
//!     debug_mode: false,
//!     max_connections: 100,
//!     timeout_seconds: Some(30),
//!     
//!     server: ServerSettings {
//!         host: "0.0.0.0".to_string(),
//!         port: 8443,
//!         use_tls: true,
//!         certificate_path: Some("/etc/ssl/cert.pem".to_string()),
//!     },
//!     
//!     database: DatabaseSettings {
//!         connection_string: "postgres://user:pass@localhost/mydb".to_string(),
//!         pool_size: 20,
//!         timeout: 5,
//!     },
//!     
//!     cache: None,
//! };
//!
//! // Serialize to INI
//! let ini_string = to_string(&config)?;
//! println!("Generated INI:\n{}", ini_string);
//!
//! // Parse it back
//! let parsed: AppConfig = from_str(&ini_string)?;
//! assert_eq!(config, parsed);
//!
//! // Example INI file that could be parsed
//! let ini_file = r#"
//! app-name = MyApp
//! version = 1.0.0
//! debug_mode = false
//! max_connections = 100
//! timeout_seconds = 30
//!
//! [server]
//! host = 0.0.0.0
//! port = 8443
//! use-tls = true
//! certificate_path = /etc/ssl/cert.pem
//!
//! [database]
//! connection-string = postgres://user:pass@localhost/mydb
//! pool_size = 20
//! timeout = 5
//!
//! ; cache =
//! "#;
//!
//! let from_file: AppConfig = from_str(ini_file)?;
//! assert_eq!(config, from_file);
//! # Ok(())
//! # }
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the LICENSE file for details.

pub mod de;
pub mod error;
pub mod ser;

pub use de::from_str;
pub use error::Error;
pub use ser::to_string;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    mod nested {
        use super::*;

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Config {
            name: String,
            port: u16,
            enabled: bool,
            description: Option<String>,
            database: Database,
            cache: Cache,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Database {
            host: String,
            port: u16,
            username: String,
            password: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Cache {
            ttl: i32,
            max_size: Option<u64>,
        }

        #[test]
        fn test_serialize() {
            let config = Config {
                name: "My App".to_string(),
                port: 8080,
                enabled: true,
                description: Some("A test application".to_string()),
                database: Database {
                    host: "localhost".to_string(),
                    port: 5432,
                    username: "admin".to_string(),
                    password: None,
                },
                cache: Cache {
                    ttl: 300,
                    max_size: Some(1000000),
                },
            };

            let ini_str = to_string(&config).unwrap();
            println!("{}", ini_str);

            // Root level fields
            assert!(ini_str.contains("name = My App"));
            assert!(ini_str.contains("port = 8080"));
            assert!(ini_str.contains("enabled = true"));
            assert!(ini_str.contains("description = A test application"));

            // Database section
            assert!(ini_str.contains("[database]"));
            assert!(ini_str.contains("host = localhost"));
            assert!(ini_str.contains("; password ="));

            // Cache section
            assert!(ini_str.contains("[cache]"));
            assert!(ini_str.contains("ttl = 300"));
            assert!(ini_str.contains("max_size = 1000000"));

            // Verify proper structure
            let lines: Vec<&str> = ini_str.lines().collect();
            let db_idx = lines.iter().position(|&l| l == "[database]").unwrap();
            let cache_idx = lines.iter().position(|&l| l == "[cache]").unwrap();
            assert!(db_idx > 0); // Database section comes after root fields
            assert!(cache_idx > db_idx); // Cache section comes after database
        }

        #[test]
        fn test_serialize_skip_none() {
            #[derive(Debug, Serialize)]
            struct Config {
                #[serde(skip_serializing_if = "Option::is_none")]
                is_hidden: Option<bool>,
                is_none: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                is_some: Option<bool>,
            }

            let config = Config {
                is_hidden: None,
                is_none: None,
                is_some: Some(true),
            };

            let ini = to_string(&config).unwrap();
            let mut lines = ini.lines();

            assert_eq!(lines.next(), Some("; is_none = "));
            assert_eq!(lines.next(), Some("is_some = true"));
            assert!(lines.next().is_none());
        }

        #[test]
        fn test_deserialize_nested() {
            let ini_str = r#"
    name = My App
    port = 8080
    enabled = true
    description = A test application
    
    [database]
    host = localhost
    port = 5432
    username = admin
    
    [cache]
    ttl = 300
    max_size = 1000000
    "#;

            let config: Config = from_str(ini_str).unwrap();

            assert_eq!(config.name, "My App");
            assert_eq!(config.port, 8080);
            assert_eq!(config.enabled, true);
            assert_eq!(config.description, Some("A test application".to_string()));
            assert_eq!(config.database.host, "localhost");
            assert_eq!(config.database.port, 5432);
            assert_eq!(config.database.username, "admin");
            assert_eq!(config.database.password, None);
            assert_eq!(config.cache.ttl, 300);
            assert_eq!(config.cache.max_size, Some(1000000));
        }
    }

    mod boxed {
        use super::*;

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Config {
            speed: f32,
            anime: Option<Box<Config>>,
            movie: Option<Box<Config>>,
        }

        #[test]
        fn test_serialize_boxed() {
            let config = Config {
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

            let ini_str = to_string(&config).unwrap();
            println!("{}", ini_str);

            // Root level fields
            assert!(ini_str.contains("speed = 1"));
            assert!(ini_str.contains("[anime]"));
            assert!(ini_str.contains("[movie]"));

            // Verify proper structure
            let lines: Vec<&str> = ini_str.lines().collect();
            let anime_idx = lines.iter().position(|&l| l == "[anime]").unwrap();
            let movie_idx = lines.iter().position(|&l| l == "[movie]").unwrap();
            assert!(anime_idx > 0); // anime section comes after root fields
            assert!(movie_idx > anime_idx); // movie section comes after anime
            assert_eq!("speed = 1.5", lines[anime_idx + 1]);
            assert_eq!("speed = 2", lines[movie_idx + 1]);
        }

        #[test]
        fn test_deserialize_boxed() {
            let ini_str = r#"
    speed = 1
    
    [anime]
    speed = 1.5
    
    [movie]
    speed = 2
    "#;

            let config: Config = from_str(ini_str).unwrap();
            let anime = config.anime.unwrap();
            let movie = config.movie.unwrap();

            assert_eq!(config.speed, 1.0);
            assert_eq!(anime.speed, 1.5);
            assert_eq!(movie.speed, 2.0);
            assert!(anime.anime.is_none());
            assert!(anime.movie.is_none());
            assert!(movie.anime.is_none());
            assert!(movie.movie.is_none());
        }
    }

    #[test]
    fn test_escaping() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct EscapeTest {
            multiline: String,
            special_chars: String,
        }

        let test = EscapeTest {
            multiline: "Line 1\nLine 2\tTabbed".to_string(),
            special_chars: "Value with \"quotes\" and ; semicolon # hash".to_string(),
        };

        let ini_str = to_string(&test).unwrap();
        assert!(ini_str.contains(r"Line 1\nLine 2\tTabbed"));
        assert!(ini_str.contains(r#"Value with \"quotes\" and \; semicolon \# hash"#));

        let deserialized: EscapeTest = from_str(&ini_str).unwrap();
        assert_eq!(test, deserialized);
    }
}
