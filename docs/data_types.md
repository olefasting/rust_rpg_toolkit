# Data Types

Data types from Rust will be used for reference. They will have serde tags that give you any relevant
information on how to represent them in JSON.

In general, enums will have a `#[serde(rename_all = "snake_case")]` attribute, which means that they
will be serialized and deserialized using `snake_case`, instead of the `PascalCase` used when defining
rust enums. Values that take the form of `Option<T>` are optional. Some other tags:

|Tag|Type| |
|:--- |:--- |:--- |
|`alias = "<identifier>"` | field | This means that `<identifier` can be used as an alternative identifier|
|`flatten` |field| This means that the value will be flattened into the containing structure|
|`default` |struct or field| This means that values from `Default` impl will be used, if no value is defined|
|`default = "<method>"` |struct or field | This means that `<method>` will be called for a default value, if none|
|`skip_serializing_if = "<method>"` |field | This means that the field will only be serialized if `<method>`, called on the field, returns `true`|
|`tag = "<identifier>` |enum | This defines an identifier for an enum variant that holds data and is therefore internally tagged|
