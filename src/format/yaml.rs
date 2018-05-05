//! Contains an implementation of YAML serialization format.

use serde_yaml;

/// A representation of a YAML data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a Yaml
/// let dump = Yaml(&data);
///
/// // Converts YAML string to a data (lazy).
/// let Yaml(data) = dump;
/// ```
pub struct Yaml<T>(pub T);

impl_format!(Yaml based on serde_yaml);
