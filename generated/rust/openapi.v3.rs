#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalPropertiesItem {
    #[prost(oneof = "additional_properties_item::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<additional_properties_item::Oneof>,
}
/// Nested message and enum types in `AdditionalPropertiesItem`.
pub mod additional_properties_item {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        SchemaOrReference(::prost::alloc::boxed::Box<super::SchemaOrReference>),
        #[prost(bool, tag = "2")]
        Boolean(bool),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Any {
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<::prost_types::Any>,
    #[prost(string, tag = "2")]
    pub yaml: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyOrExpression {
    #[prost(oneof = "any_or_expression::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<any_or_expression::Oneof>,
}
/// Nested message and enum types in `AnyOrExpression`.
pub mod any_or_expression {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        Any(super::Any),
        #[prost(message, tag = "2")]
        Expression(super::Expression),
    }
}
/// A map of possible out-of band callbacks related to the parent operation. Each value in the map is a Path Item Object that describes a set of requests that may be initiated by the API provider and the expected responses. The key value used to identify the callback object is an expression, evaluated at runtime, that identifies a URL to use for the callback operation.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Callback {
    #[prost(message, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<NamedPathItem>,
    #[prost(message, repeated, tag = "2")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallbackOrReference {
    #[prost(oneof = "callback_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<callback_or_reference::Oneof>,
}
/// Nested message and enum types in `CallbackOrReference`.
pub mod callback_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        Callback(super::Callback),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallbacksOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedCallbackOrReference>,
}
/// Holds a set of reusable objects for different aspects of the OAS. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Components {
    #[prost(message, optional, tag = "1")]
    pub schemas: ::core::option::Option<SchemasOrReferences>,
    #[prost(message, optional, tag = "2")]
    pub responses: ::core::option::Option<ResponsesOrReferences>,
    #[prost(message, optional, tag = "3")]
    pub parameters: ::core::option::Option<ParametersOrReferences>,
    #[prost(message, optional, tag = "4")]
    pub examples: ::core::option::Option<ExamplesOrReferences>,
    #[prost(message, optional, tag = "5")]
    pub request_bodies: ::core::option::Option<RequestBodiesOrReferences>,
    #[prost(message, optional, tag = "6")]
    pub headers: ::core::option::Option<HeadersOrReferences>,
    #[prost(message, optional, tag = "7")]
    pub security_schemes: ::core::option::Option<SecuritySchemesOrReferences>,
    #[prost(message, optional, tag = "8")]
    pub links: ::core::option::Option<LinksOrReferences>,
    #[prost(message, optional, tag = "9")]
    pub callbacks: ::core::option::Option<CallbacksOrReferences>,
    #[prost(message, repeated, tag = "10")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// Contact information for the exposed API.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultType {
    #[prost(oneof = "default_type::Oneof", tags = "1, 2, 3")]
    pub oneof: ::core::option::Option<default_type::Oneof>,
}
/// Nested message and enum types in `DefaultType`.
pub mod default_type {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(double, tag = "1")]
        Number(f64),
        #[prost(bool, tag = "2")]
        Boolean(bool),
        #[prost(string, tag = "3")]
        String(::prost::alloc::string::String),
    }
}
/// When request bodies or response payloads may be one of a number of different schemas, a `discriminator` object can be used to aid in serialization, deserialization, and validation.  The discriminator is a specific object in a schema which is used to inform the consumer of the specification of an alternative schema based on the value associated with it.  When using the discriminator, _inline_ schemas will not be considered.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Discriminator {
    #[prost(string, tag = "1")]
    pub property_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub mapping: ::core::option::Option<Strings>,
    #[prost(message, repeated, tag = "3")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    #[prost(string, tag = "1")]
    pub openapi: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<Info>,
    #[prost(message, repeated, tag = "3")]
    pub servers: ::prost::alloc::vec::Vec<Server>,
    #[prost(message, optional, tag = "4")]
    pub paths: ::core::option::Option<Paths>,
    #[prost(message, optional, tag = "5")]
    pub components: ::core::option::Option<Components>,
    #[prost(message, repeated, tag = "6")]
    pub security: ::prost::alloc::vec::Vec<SecurityRequirement>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    #[prost(message, optional, tag = "8")]
    pub external_docs: ::core::option::Option<ExternalDocs>,
    #[prost(message, repeated, tag = "9")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// A single encoding definition applied to a single schema property.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Encoding {
    #[prost(string, tag = "1")]
    pub content_type: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub headers: ::core::option::Option<HeadersOrReferences>,
    #[prost(string, tag = "3")]
    pub style: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub explode: bool,
    #[prost(bool, tag = "5")]
    pub allow_reserved: bool,
    #[prost(message, repeated, tag = "6")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Encodings {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedEncoding>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Example {
    #[prost(string, tag = "1")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<Any>,
    #[prost(string, tag = "4")]
    pub external_value: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleOrReference {
    #[prost(oneof = "example_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<example_or_reference::Oneof>,
}
/// Nested message and enum types in `ExampleOrReference`.
pub mod example_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        Example(super::Example),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExamplesOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedExampleOrReference>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expression {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedAny>,
}
/// Allows referencing an external resource for extended documentation.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalDocs {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// The Header Object follows the structure of the Parameter Object with the following changes:  1. `name` MUST NOT be specified, it is given in the corresponding `headers` map. 1. `in` MUST NOT be specified, it is implicitly in `header`. 1. All traits that are affected by the location MUST be applicable to a location of `header` (for example, `style`).
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub required: bool,
    #[prost(bool, tag = "3")]
    pub deprecated: bool,
    #[prost(bool, tag = "4")]
    pub allow_empty_value: bool,
    #[prost(string, tag = "5")]
    pub style: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub explode: bool,
    #[prost(bool, tag = "7")]
    pub allow_reserved: bool,
    #[prost(message, optional, tag = "8")]
    pub schema: ::core::option::Option<SchemaOrReference>,
    #[prost(message, optional, tag = "9")]
    pub example: ::core::option::Option<Any>,
    #[prost(message, optional, tag = "10")]
    pub examples: ::core::option::Option<ExamplesOrReferences>,
    #[prost(message, optional, tag = "11")]
    pub content: ::core::option::Option<MediaTypes>,
    #[prost(message, repeated, tag = "12")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderOrReference {
    #[prost(oneof = "header_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<header_or_reference::Oneof>,
}
/// Nested message and enum types in `HeaderOrReference`.
pub mod header_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        Header(super::Header),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeadersOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedHeaderOrReference>,
}
/// The object provides metadata about the API. The metadata MAY be used by the clients if needed, and MAY be presented in editing or documentation generation tools for convenience.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Info {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub terms_of_service: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub contact: ::core::option::Option<Contact>,
    #[prost(message, optional, tag = "5")]
    pub license: ::core::option::Option<License>,
    #[prost(string, tag = "6")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "7")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
    #[prost(string, tag = "8")]
    pub summary: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemsItem {
    #[prost(message, repeated, tag = "1")]
    pub schema_or_reference: ::prost::alloc::vec::Vec<SchemaOrReference>,
}
/// License information for the exposed API.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct License {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// The `Link object` represents a possible design-time link for a response. The presence of a link does not guarantee the caller's ability to successfully invoke it, rather it provides a known relationship and traversal mechanism between responses and other operations.  Unlike _dynamic_ links (i.e. links provided **in** the response payload), the OAS linking mechanism does not require link information in the runtime response.  For computing links, and providing instructions to execute them, a runtime expression is used for accessing values in an operation and using them as parameters while invoking the linked operation.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Link {
    #[prost(string, tag = "1")]
    pub operation_ref: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub operation_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub parameters: ::core::option::Option<AnyOrExpression>,
    #[prost(message, optional, tag = "4")]
    pub request_body: ::core::option::Option<AnyOrExpression>,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub server: ::core::option::Option<Server>,
    #[prost(message, repeated, tag = "7")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkOrReference {
    #[prost(oneof = "link_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<link_or_reference::Oneof>,
}
/// Nested message and enum types in `LinkOrReference`.
pub mod link_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        Link(super::Link),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinksOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedLinkOrReference>,
}
/// Each Media Type Object provides schema and examples for the media type identified by its key.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaType {
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<SchemaOrReference>,
    #[prost(message, optional, tag = "2")]
    pub example: ::core::option::Option<Any>,
    #[prost(message, optional, tag = "3")]
    pub examples: ::core::option::Option<ExamplesOrReferences>,
    #[prost(message, optional, tag = "4")]
    pub encoding: ::core::option::Option<Encodings>,
    #[prost(message, repeated, tag = "5")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaTypes {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedMediaType>,
}
/// Automatically-generated message used to represent maps of Any as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedAny {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Any>,
}
/// Automatically-generated message used to represent maps of CallbackOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedCallbackOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<CallbackOrReference>,
}
/// Automatically-generated message used to represent maps of Encoding as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedEncoding {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Encoding>,
}
/// Automatically-generated message used to represent maps of ExampleOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedExampleOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ExampleOrReference>,
}
/// Automatically-generated message used to represent maps of HeaderOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedHeaderOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<HeaderOrReference>,
}
/// Automatically-generated message used to represent maps of LinkOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedLinkOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<LinkOrReference>,
}
/// Automatically-generated message used to represent maps of MediaType as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedMediaType {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<MediaType>,
}
/// Automatically-generated message used to represent maps of ParameterOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedParameterOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ParameterOrReference>,
}
/// Automatically-generated message used to represent maps of PathItem as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedPathItem {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<PathItem>,
}
/// Automatically-generated message used to represent maps of RequestBodyOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedRequestBodyOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<RequestBodyOrReference>,
}
/// Automatically-generated message used to represent maps of ResponseOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedResponseOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ResponseOrReference>,
}
/// Automatically-generated message used to represent maps of SchemaOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedSchemaOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<SchemaOrReference>,
}
/// Automatically-generated message used to represent maps of SecuritySchemeOrReference as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedSecuritySchemeOrReference {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<SecuritySchemeOrReference>,
}
/// Automatically-generated message used to represent maps of ServerVariable as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedServerVariable {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ServerVariable>,
}
/// Automatically-generated message used to represent maps of string as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedString {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Automatically-generated message used to represent maps of StringArray as ordered (name,value) pairs.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedStringArray {
    /// Map key
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapped value
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<StringArray>,
}
/// Configuration details for a supported OAuth Flow
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OauthFlow {
    #[prost(string, tag = "1")]
    pub authorization_url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub refresh_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub scopes: ::core::option::Option<Strings>,
    #[prost(message, repeated, tag = "5")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// Allows configuration of the supported OAuth Flows.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OauthFlows {
    #[prost(message, optional, tag = "1")]
    pub implicit: ::core::option::Option<OauthFlow>,
    #[prost(message, optional, tag = "2")]
    pub password: ::core::option::Option<OauthFlow>,
    #[prost(message, optional, tag = "3")]
    pub client_credentials: ::core::option::Option<OauthFlow>,
    #[prost(message, optional, tag = "4")]
    pub authorization_code: ::core::option::Option<OauthFlow>,
    #[prost(message, repeated, tag = "5")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedAny>,
}
/// Describes a single API operation on a path.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    #[prost(string, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub external_docs: ::core::option::Option<ExternalDocs>,
    #[prost(string, tag = "5")]
    pub operation_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub parameters: ::prost::alloc::vec::Vec<ParameterOrReference>,
    #[prost(message, optional, tag = "7")]
    pub request_body: ::core::option::Option<RequestBodyOrReference>,
    #[prost(message, optional, tag = "8")]
    pub responses: ::core::option::Option<Responses>,
    #[prost(message, optional, tag = "9")]
    pub callbacks: ::core::option::Option<CallbacksOrReferences>,
    #[prost(bool, tag = "10")]
    pub deprecated: bool,
    #[prost(message, repeated, tag = "11")]
    pub security: ::prost::alloc::vec::Vec<SecurityRequirement>,
    #[prost(message, repeated, tag = "12")]
    pub servers: ::prost::alloc::vec::Vec<Server>,
    #[prost(message, repeated, tag = "13")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// Describes a single operation parameter.  A unique parameter is defined by a combination of a name and location.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#in: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub required: bool,
    #[prost(bool, tag = "5")]
    pub deprecated: bool,
    #[prost(bool, tag = "6")]
    pub allow_empty_value: bool,
    #[prost(string, tag = "7")]
    pub style: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub explode: bool,
    #[prost(bool, tag = "9")]
    pub allow_reserved: bool,
    #[prost(message, optional, tag = "10")]
    pub schema: ::core::option::Option<SchemaOrReference>,
    #[prost(message, optional, tag = "11")]
    pub example: ::core::option::Option<Any>,
    #[prost(message, optional, tag = "12")]
    pub examples: ::core::option::Option<ExamplesOrReferences>,
    #[prost(message, optional, tag = "13")]
    pub content: ::core::option::Option<MediaTypes>,
    #[prost(message, repeated, tag = "14")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterOrReference {
    #[prost(oneof = "parameter_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<parameter_or_reference::Oneof>,
}
/// Nested message and enum types in `ParameterOrReference`.
pub mod parameter_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        Parameter(super::Parameter),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParametersOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedParameterOrReference>,
}
/// Describes the operations available on a single path. A Path Item MAY be empty, due to ACL constraints. The path itself is still exposed to the documentation viewer but they will not know which operations and parameters are available.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathItem {
    #[prost(string, tag = "1")]
    pub r#ref: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub get: ::core::option::Option<Operation>,
    #[prost(message, optional, tag = "5")]
    pub put: ::core::option::Option<Operation>,
    #[prost(message, optional, tag = "6")]
    pub post: ::core::option::Option<Operation>,
    #[prost(message, optional, tag = "7")]
    pub delete: ::core::option::Option<Operation>,
    #[prost(message, optional, tag = "8")]
    pub options: ::core::option::Option<Operation>,
    #[prost(message, optional, tag = "9")]
    pub head: ::core::option::Option<Operation>,
    #[prost(message, optional, tag = "10")]
    pub patch: ::core::option::Option<Operation>,
    #[prost(message, optional, tag = "11")]
    pub trace: ::core::option::Option<Operation>,
    #[prost(message, repeated, tag = "12")]
    pub servers: ::prost::alloc::vec::Vec<Server>,
    #[prost(message, repeated, tag = "13")]
    pub parameters: ::prost::alloc::vec::Vec<ParameterOrReference>,
    #[prost(message, repeated, tag = "14")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// Holds the relative paths to the individual endpoints and their operations. The path is appended to the URL from the `Server Object` in order to construct the full URL.  The Paths MAY be empty, due to ACL constraints.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paths {
    #[prost(message, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<NamedPathItem>,
    #[prost(message, repeated, tag = "2")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Properties {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedSchemaOrReference>,
}
/// A simple object to allow referencing other components in the specification, internally and externally.  The Reference Object is defined by JSON Reference and follows the same structure, behavior and rules.   For this specification, reference resolution is accomplished as defined by the JSON Reference specification and not by the JSON Schema specification.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reference {
    #[prost(string, tag = "1")]
    pub r#ref: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub summary: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBodiesOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedRequestBodyOrReference>,
}
/// Describes a single request body.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBody {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<MediaTypes>,
    #[prost(bool, tag = "3")]
    pub required: bool,
    #[prost(message, repeated, tag = "4")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBodyOrReference {
    #[prost(oneof = "request_body_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<request_body_or_reference::Oneof>,
}
/// Nested message and enum types in `RequestBodyOrReference`.
pub mod request_body_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        RequestBody(super::RequestBody),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
/// Describes a single response from an API Operation, including design-time, static  `links` to operations based on the response.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub headers: ::core::option::Option<HeadersOrReferences>,
    #[prost(message, optional, tag = "3")]
    pub content: ::core::option::Option<MediaTypes>,
    #[prost(message, optional, tag = "4")]
    pub links: ::core::option::Option<LinksOrReferences>,
    #[prost(message, repeated, tag = "5")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseOrReference {
    #[prost(oneof = "response_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<response_or_reference::Oneof>,
}
/// Nested message and enum types in `ResponseOrReference`.
pub mod response_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        Response(super::Response),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
/// A container for the expected responses of an operation. The container maps a HTTP response code to the expected response.  The documentation is not necessarily expected to cover all possible HTTP response codes because they may not be known in advance. However, documentation is expected to cover a successful operation response and any known errors.  The `default` MAY be used as a default response object for all HTTP codes  that are not covered individually by the specification.  The `Responses Object` MUST contain at least one response code, and it  SHOULD be the response for a successful operation call.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Responses {
    #[prost(message, optional, tag = "1")]
    pub default: ::core::option::Option<ResponseOrReference>,
    #[prost(message, repeated, tag = "2")]
    pub response_or_reference: ::prost::alloc::vec::Vec<NamedResponseOrReference>,
    #[prost(message, repeated, tag = "3")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsesOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedResponseOrReference>,
}
/// The Schema Object allows the definition of input and output data types. These types can be objects, but also primitives and arrays. This object is an extended subset of the JSON Schema Specification Wright Draft 00.  For more information about the properties, see JSON Schema Core and JSON Schema Validation. Unless stated otherwise, the property definitions follow the JSON Schema.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    #[prost(bool, tag = "1")]
    pub nullable: bool,
    #[prost(message, optional, tag = "2")]
    pub discriminator: ::core::option::Option<Discriminator>,
    #[prost(bool, tag = "3")]
    pub read_only: bool,
    #[prost(bool, tag = "4")]
    pub write_only: bool,
    #[prost(message, optional, tag = "5")]
    pub xml: ::core::option::Option<Xml>,
    #[prost(message, optional, tag = "6")]
    pub external_docs: ::core::option::Option<ExternalDocs>,
    #[prost(message, optional, tag = "7")]
    pub example: ::core::option::Option<Any>,
    #[prost(bool, tag = "8")]
    pub deprecated: bool,
    #[prost(string, tag = "9")]
    pub title: ::prost::alloc::string::String,
    #[prost(double, tag = "10")]
    pub multiple_of: f64,
    #[prost(double, tag = "11")]
    pub maximum: f64,
    #[prost(bool, tag = "12")]
    pub exclusive_maximum: bool,
    #[prost(double, tag = "13")]
    pub minimum: f64,
    #[prost(bool, tag = "14")]
    pub exclusive_minimum: bool,
    #[prost(int64, tag = "15")]
    pub max_length: i64,
    #[prost(int64, tag = "16")]
    pub min_length: i64,
    #[prost(string, tag = "17")]
    pub pattern: ::prost::alloc::string::String,
    #[prost(int64, tag = "18")]
    pub max_items: i64,
    #[prost(int64, tag = "19")]
    pub min_items: i64,
    #[prost(bool, tag = "20")]
    pub unique_items: bool,
    #[prost(int64, tag = "21")]
    pub max_properties: i64,
    #[prost(int64, tag = "22")]
    pub min_properties: i64,
    #[prost(string, repeated, tag = "23")]
    pub required: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "24")]
    pub r#enum: ::prost::alloc::vec::Vec<Any>,
    #[prost(string, tag = "25")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "26")]
    pub all_of: ::prost::alloc::vec::Vec<SchemaOrReference>,
    #[prost(message, repeated, tag = "27")]
    pub one_of: ::prost::alloc::vec::Vec<SchemaOrReference>,
    #[prost(message, repeated, tag = "28")]
    pub any_of: ::prost::alloc::vec::Vec<SchemaOrReference>,
    #[prost(message, optional, boxed, tag = "29")]
    pub not: ::core::option::Option<::prost::alloc::boxed::Box<Schema>>,
    #[prost(message, optional, tag = "30")]
    pub items: ::core::option::Option<ItemsItem>,
    #[prost(message, optional, tag = "31")]
    pub properties: ::core::option::Option<Properties>,
    #[prost(message, optional, boxed, tag = "32")]
    pub additional_properties: ::core::option::Option<
        ::prost::alloc::boxed::Box<AdditionalPropertiesItem>,
    >,
    #[prost(message, optional, tag = "33")]
    pub default: ::core::option::Option<DefaultType>,
    #[prost(string, tag = "34")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "35")]
    pub format: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "36")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaOrReference {
    #[prost(oneof = "schema_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<schema_or_reference::Oneof>,
}
/// Nested message and enum types in `SchemaOrReference`.
pub mod schema_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        Schema(::prost::alloc::boxed::Box<super::Schema>),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemasOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedSchemaOrReference>,
}
/// Lists the required security schemes to execute this operation. The name used for each property MUST correspond to a security scheme declared in the Security Schemes under the Components Object.  Security Requirement Objects that contain multiple schemes require that all schemes MUST be satisfied for a request to be authorized. This enables support for scenarios where multiple query parameters or HTTP headers are required to convey security information.  When a list of Security Requirement Objects is defined on the OpenAPI Object or Operation Object, only one of the Security Requirement Objects in the list needs to be satisfied to authorize the request.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityRequirement {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedStringArray>,
}
/// Defines a security scheme that can be used by the operations. Supported schemes are HTTP authentication, an API key (either as a header, a cookie parameter or as a query parameter), mutual TLS (use of a client certificate), OAuth2's common flows (implicit, password, application and access code) as defined in RFC6749, and OpenID Connect.   Please note that currently (2019) the implicit flow is about to be deprecated OAuth 2.0 Security Best Current Practice. Recommended for most use case is Authorization Code Grant flow with PKCE.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityScheme {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub r#in: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub scheme: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub bearer_format: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub flows: ::core::option::Option<OauthFlows>,
    #[prost(string, tag = "8")]
    pub open_id_connect_url: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "9")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecuritySchemeOrReference {
    #[prost(oneof = "security_scheme_or_reference::Oneof", tags = "1, 2")]
    pub oneof: ::core::option::Option<security_scheme_or_reference::Oneof>,
}
/// Nested message and enum types in `SecuritySchemeOrReference`.
pub mod security_scheme_or_reference {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(message, tag = "1")]
        SecurityScheme(super::SecurityScheme),
        #[prost(message, tag = "2")]
        Reference(super::Reference),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecuritySchemesOrReferences {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedSecuritySchemeOrReference>,
}
/// An object representing a Server.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Server {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub variables: ::core::option::Option<ServerVariables>,
    #[prost(message, repeated, tag = "4")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// An object representing a Server Variable for server URL template substitution.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerVariable {
    #[prost(string, repeated, tag = "1")]
    pub r#enum: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub default: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerVariables {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedServerVariable>,
}
/// Any property starting with x- is valid.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecificationExtension {
    #[prost(oneof = "specification_extension::Oneof", tags = "1, 2, 3")]
    pub oneof: ::core::option::Option<specification_extension::Oneof>,
}
/// Nested message and enum types in `SpecificationExtension`.
pub mod specification_extension {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Oneof {
        #[prost(double, tag = "1")]
        Number(f64),
        #[prost(bool, tag = "2")]
        Boolean(bool),
        #[prost(string, tag = "3")]
        String(::prost::alloc::string::String),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringArray {
    #[prost(string, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Strings {
    #[prost(message, repeated, tag = "1")]
    pub additional_properties: ::prost::alloc::vec::Vec<NamedString>,
}
/// Adds metadata to a single tag that is used by the Operation Object. It is not mandatory to have a Tag Object per tag defined in the Operation Object instances.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub external_docs: ::core::option::Option<ExternalDocs>,
    #[prost(message, repeated, tag = "4")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
/// A metadata object that allows for more fine-tuned XML model definitions.  When using arrays, XML element names are *not* inferred (for singular/plural forms) and the `name` property SHOULD be used to add that information. See examples for expected behavior.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Xml {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub attribute: bool,
    #[prost(bool, tag = "5")]
    pub wrapped: bool,
    #[prost(message, repeated, tag = "6")]
    pub specification_extension: ::prost::alloc::vec::Vec<NamedAny>,
}
