//! The model module is the schema for the data store.
//!
//! The datastore::serialization and datastore::deserialization modules make it easy to map between
//! Rust types and the datastore, and thus, all inputs and outputs are type-checked.

pub mod modeled_types;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::modeled_types::{Identifier, SingleLineString, Url};

///// Primary user-visible settings

// Note: fields are marked with skip_serializing_if=Option::is_none so that settings GETs don't
// show field=null for everything that isn't set in the relevant group of settings.

// Note: we have to use 'rename' here because the top-level Settings structure is the only one
// that uses its name in serialization; internal structures use the field name that points to it
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename = "settings", rename_all = "kebab-case")]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<SingleLineString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<SingleLineString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<UpdatesSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_containers: Option<HashMap<Identifier, ContainerImage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp: Option<NtpSettings>,
}

// Updog settings. Taken from userdata. The 'seed' setting is generated
// by the "Bork" settings generator at runtime.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct UpdatesSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_base_url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_base_url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct ContainerImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub superpowered: Option<bool>,
}

// NTP settings
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct NtpSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_servers: Option<Vec<Url>>,
}

///// Internal services

// Note: Top-level objects that get returned from the API should have a serde "rename" attribute
// matching the struct name, but in kebab-case, e.g. ConfigurationFiles -> "configuration-files".
// This lets it match the datastore name.
// Objects that live inside those top-level objects, e.g. Service lives in Services, should have
// rename="" so they don't add an extra prefix to the datastore path that doesn't actually exist.
// This is important because we have APIs that can return those sub-structures directly.

pub type Services = HashMap<String, Service>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename = "", rename_all = "kebab-case")]
pub struct Service {
    pub configuration_files: Vec<SingleLineString>,
    pub restart_commands: Vec<String>,
}

pub type ConfigurationFiles = HashMap<String, ConfigurationFile>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename = "", rename_all = "kebab-case")]
pub struct ConfigurationFile {
    pub path: SingleLineString,
    pub template_path: SingleLineString,
}

///// Metadata

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename = "metadata", rename_all = "kebab-case")]
pub struct Metadata {
    pub key: SingleLineString,
    pub md: SingleLineString,
    pub val: toml::Value,
}