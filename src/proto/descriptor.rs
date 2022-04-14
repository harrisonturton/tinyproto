use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct FileDescriptor<'a> {
    pub name: &'a str,
    pub syntax: Option<SyntaxDescriptor<'a>>,
    pub messages: Vec<MessageDescriptor<'a>>,
    pub services: Vec<ServiceDescriptor<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SyntaxDescriptor<'a> {
    #[serde(rename(serialize = "proto2"))]
    Proto2,
    #[serde(rename(serialize = "proto3"))]
    Proto3,
    Unknown(&'a str),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct MessageDescriptor<'a> {
    pub name: &'a str,
    pub fields: Vec<FieldDescriptor<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FieldDescriptor<'a> {
    pub label: Option<FieldDescriptorLabel<'a>>,
    #[serde(rename(serialize = "type"))]
    pub typ: FieldDescriptorType<'a>,
    pub name: &'a str,
    pub number: &'a str,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldDescriptorLabel<'a> {
    Optional,
    Required,
    Repeated,
    Unknown(&'a str),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldDescriptorType<'a> {
    String,
    Message(&'a str),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceDescriptor<'a> {
    pub name: &'a str,
    pub methods: Vec<MethodDescriptor<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MethodDescriptor<'a> {
    pub name: &'a str,
    pub input_type: &'a str,
    pub output_type: &'a str,
    pub client_streaming: bool,
    pub server_streaming: bool,
}