// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthOtpRequest {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<User>,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomCheckRequest {
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Otp {
    #[prost(float, tag = "1")]
    pub otp: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequest {
    #[prost(string, tag = "1")]
    pub phone: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub teach: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequestOtp {
    #[prost(string, tag = "1")]
    pub phone: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub code: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// @gotags: cql:"phone"
    #[prost(string, tag = "1")]
    pub phone: ::prost::alloc::string::String,
    /// @gotags: cql:"teach"
    #[prost(bool, tag = "2")]
    pub teach: bool,
    /// @gotags: cql:"code"
    #[prost(int64, tag = "3")]
    pub code: i64,
    /// @gotags: cql:"username"
    #[prost(string, tag = "4")]
    pub username: ::prost::alloc::string::String,
    /// @gotags: cql:"retry_otp"
    #[prost(int32, tag = "5")]
    pub retry_otp: i32,
    /// @gotags: cql:"verified"
    #[prost(bool, tag = "6")]
    pub verified: bool,
    /// @gotags: cql:"expire_otp"
    #[prost(message, optional, tag = "7")]
    pub expire_otp: ::core::option::Option<::prost_types::Timestamp>,
    /// @gotags: cql:"create_at"
    #[prost(message, optional, tag = "8")]
    pub create_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {
    /// @gotags: cql:"room_id"
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// @gotags: cql:"chat_room"
    #[prost(string, tag = "2")]
    pub chat_room: ::prost::alloc::string::String,
    /// @gotags: cql:"users"
    #[prost(string, repeated, tag = "3")]
    pub users: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaintEventInit {
    #[prost(string, tag = "3")]
    pub chat_room: ::prost::alloc::string::String,
    #[prost(bool, tag = "1")]
    pub init: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamVertex {
    #[prost(float, repeated, tag = "1")]
    pub position: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, tag = "2")]
    pub color: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct StreamRect {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
    #[prost(float, tag = "3")]
    pub width: f32,
    #[prost(float, tag = "4")]
    pub height: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assignment {
    /// @gotags: cql:"id"
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// @gotags: cql:"question"
    #[prost(string, tag = "2")]
    pub question: ::prost::alloc::string::String,
    /// @gotags: cql:"chat_room"
    #[prost(string, tag = "3")]
    pub chat_room: ::prost::alloc::string::String,
    /// @gotags: cql:"deleted"
    #[prost(bool, tag = "4")]
    pub deleted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTextEntries {
    #[prost(float, repeated, tag = "1")]
    pub position: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, tag = "2")]
    pub color: ::prost::alloc::vec::Vec<f32>,
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub pending: bool,
    #[prost(message, optional, tag = "5")]
    pub bounds: ::core::option::Option<StreamRect>,
    #[prost(int32, tag = "6")]
    pub font_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamRectangle {
    #[prost(float, repeated, tag = "1")]
    pub first: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, tag = "2")]
    pub last: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, tag = "3")]
    pub color: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAction {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub vertices: ::prost::alloc::vec::Vec<StreamVertex>,
    #[prost(message, optional, tag = "3")]
    pub text: ::core::option::Option<StreamTextEntries>,
    #[prost(message, optional, tag = "4")]
    pub rectangle: ::core::option::Option<StreamRectangle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamActions {
    #[prost(message, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<StreamAction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowPaintHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub paints: ::prost::alloc::vec::Vec<PaintEvent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuestionsHistoryRequest {
    #[prost(string, tag = "1")]
    pub chat_room: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuestionsHistoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub questions: ::prost::alloc::vec::Vec<Question>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, optional, tag = "2")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub room_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub stats_code: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    /// @gotags: cql:"id"
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// @gotags: cql:"chat_room"
    #[prost(string, tag = "2")]
    pub chat_room: ::prost::alloc::string::String,
    /// @gotags: cql:"username"
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// @gotags: cql:"message"
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    /// @gotags: cql:"deleted"
    #[prost(bool, tag = "5")]
    pub deleted: bool,
    /// @gotags: cql:"timestamp"
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaintEvent {
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<StreamActions>,
    #[prost(string, tag = "4")]
    pub chat_room: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "paint_event::ActionType", tags = "1, 2")]
    pub action_type: ::core::option::Option<paint_event::ActionType>,
}
/// Nested message and enum types in `PaintEvent`.
pub mod paint_event {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum ActionType {
        #[prost(bool, tag = "1")]
        IsDeleted(bool),
        #[prost(bool, tag = "2")]
        ActionRequest(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Question {
    /// @gotags: cql:"id"
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// @gotags: cql:"chat_room"
    #[prost(string, tag = "2")]
    pub chat_room: ::prost::alloc::string::String,
    /// @gotags: cql:"text"
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// @gotags: cql:"answers"
    #[prost(string, repeated, tag = "4")]
    pub answers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// @gotags: cql:"deleted"
    #[prost(bool, tag = "5")]
    pub deleted: bool,
    /// @gotags: cql:"timestamp"
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscribe {
    #[prost(string, tag = "6")]
    pub chat_room: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub username: ::prost::alloc::string::String,
    #[prost(oneof = "subscribe::Data", tags = "1, 2, 3, 4, 5")]
    pub data: ::core::option::Option<subscribe::Data>,
}
/// Nested message and enum types in `Subscribe`.
pub mod subscribe {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        ChatMessage(super::ChatMessage),
        #[prost(message, tag = "2")]
        PaintEvent(super::PaintEvent),
        #[prost(message, tag = "3")]
        Question(super::Question),
        #[prost(message, tag = "4")]
        PaintEventInit(super::PaintEventInit),
        #[prost(message, tag = "5")]
        Assignment(super::Assignment),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    #[prost(string, tag = "1")]
    pub chat_room: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
}
