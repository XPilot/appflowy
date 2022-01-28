// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `dart_notification.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UserNotification {
    Unknown = 0,
    UserAuthChanged = 1,
    UserProfileUpdated = 2,
    UserUnauthorized = 3,
    UserWsConnectStateChanged = 4,
}

impl ::protobuf::ProtobufEnum for UserNotification {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UserNotification> {
        match value {
            0 => ::std::option::Option::Some(UserNotification::Unknown),
            1 => ::std::option::Option::Some(UserNotification::UserAuthChanged),
            2 => ::std::option::Option::Some(UserNotification::UserProfileUpdated),
            3 => ::std::option::Option::Some(UserNotification::UserUnauthorized),
            4 => ::std::option::Option::Some(UserNotification::UserWsConnectStateChanged),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UserNotification] = &[
            UserNotification::Unknown,
            UserNotification::UserAuthChanged,
            UserNotification::UserProfileUpdated,
            UserNotification::UserUnauthorized,
            UserNotification::UserWsConnectStateChanged,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<UserNotification>("UserNotification", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for UserNotification {
}

impl ::std::default::Default for UserNotification {
    fn default() -> Self {
        UserNotification::Unknown
    }
}

impl ::protobuf::reflect::ProtobufValue for UserNotification {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17dart_notification.proto*\x81\x01\n\x10UserNotification\x12\x0b\n\
    \x07Unknown\x10\0\x12\x13\n\x0fUserAuthChanged\x10\x01\x12\x16\n\x12User\
    ProfileUpdated\x10\x02\x12\x14\n\x10UserUnauthorized\x10\x03\x12\x1d\n\
    \x19UserWsConnectStateChanged\x10\x04J\xf7\x01\n\x06\x12\x04\0\0\x08\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\x12\x04\x02\0\x08\x01\n\
    \n\n\x03\x05\0\x01\x12\x03\x02\x05\x15\n\x0b\n\x04\x05\0\x02\0\x12\x03\
    \x03\x04\x10\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x03\x04\x0b\n\x0c\n\x05\
    \x05\0\x02\0\x02\x12\x03\x03\x0e\x0f\n\x0b\n\x04\x05\0\x02\x01\x12\x03\
    \x04\x04\x18\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x04\x04\x13\n\x0c\n\
    \x05\x05\0\x02\x01\x02\x12\x03\x04\x16\x17\n\x0b\n\x04\x05\0\x02\x02\x12\
    \x03\x05\x04\x1b\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x05\x04\x16\n\x0c\
    \n\x05\x05\0\x02\x02\x02\x12\x03\x05\x19\x1a\n\x0b\n\x04\x05\0\x02\x03\
    \x12\x03\x06\x04\x19\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x06\x04\x14\n\
    \x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x06\x17\x18\n\x0b\n\x04\x05\0\x02\
    \x04\x12\x03\x07\x04\"\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x07\x04\x1d\
    \n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x07\x20!b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}