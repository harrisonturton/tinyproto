use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileDescriptor {
    pub name: String,
    pub syntax: Option<SyntaxDescriptor>,
    pub messages: Vec<MessageDescriptor>,
    pub services: Vec<ServiceDescriptor>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SyntaxDescriptor {
    Proto2,
    Proto3,
    Unknown(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageDescriptor {
    pub name: String,
    pub fields: Vec<FieldDescriptor>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FieldDescriptor {
    pub label: Option<FieldDescriptorLabel>,
    pub typ: FieldDescriptorType,
    pub name: String,
    pub number: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldDescriptorLabel {
    Optional,
    Required,
    Repeated,
    Unknown(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FieldDescriptorType {
    String,
    Message(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceDescriptor {
    pub name: String,
    pub methods: Vec<MethodDescriptor>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MethodDescriptor {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
    pub client_streaming: bool,
    pub server_streaming: bool,
}