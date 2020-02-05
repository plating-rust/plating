# Installation

Add the following to your Cargo.toml to install this crate from [crates.io](https://TODO)

```toml
[Dependencies]
plating = "^0.0.1"
```
We follow the [semver](https://semver.org/) specification.


Our git is usually a little ahead of the version published on crates.io. If you want to be on the bleeding edge,
then use
```toml
[Dependencies]
plating = { git = "https://github.com/plating-rust/plating.git" }
````
We only bumb the version when publishing on crates.io, so don't rely on semver if you're using the git version! You have been warned.

## Feature Flags
Here is a list of available Feature flags:

Feature Flag | Default |Description
------------ | ------- |-------------
logging | Off | Uses the log crate to log debug information
cocoa | On (On OSX) | The coocoa Backend


