// This file is generated by rust-protobuf 2.25.2. Do not edit
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
//! Generated file from `event_map.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GridEvent {
    GetGridData = 0,
    GetGridBlocks = 1,
    GetFields = 10,
    UpdateField = 11,
    InsertField = 12,
    DeleteField = 13,
    SwitchToField = 14,
    DuplicateField = 15,
    GetEditFieldContext = 16,
    NewSelectOption = 30,
    GetSelectOptionContext = 31,
    UpdateSelectOption = 32,
    CreateRow = 50,
    GetRow = 51,
    DeleteRow = 52,
    DuplicateRow = 53,
    GetCell = 70,
    UpdateCell = 71,
    UpdateCellSelectOption = 72,
}

impl ::protobuf::ProtobufEnum for GridEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GridEvent> {
        match value {
            0 => ::std::option::Option::Some(GridEvent::GetGridData),
            1 => ::std::option::Option::Some(GridEvent::GetGridBlocks),
            10 => ::std::option::Option::Some(GridEvent::GetFields),
            11 => ::std::option::Option::Some(GridEvent::UpdateField),
            12 => ::std::option::Option::Some(GridEvent::InsertField),
            13 => ::std::option::Option::Some(GridEvent::DeleteField),
            14 => ::std::option::Option::Some(GridEvent::SwitchToField),
            15 => ::std::option::Option::Some(GridEvent::DuplicateField),
            16 => ::std::option::Option::Some(GridEvent::GetEditFieldContext),
            30 => ::std::option::Option::Some(GridEvent::NewSelectOption),
            31 => ::std::option::Option::Some(GridEvent::GetSelectOptionContext),
            32 => ::std::option::Option::Some(GridEvent::UpdateSelectOption),
            50 => ::std::option::Option::Some(GridEvent::CreateRow),
            51 => ::std::option::Option::Some(GridEvent::GetRow),
            52 => ::std::option::Option::Some(GridEvent::DeleteRow),
            53 => ::std::option::Option::Some(GridEvent::DuplicateRow),
            70 => ::std::option::Option::Some(GridEvent::GetCell),
            71 => ::std::option::Option::Some(GridEvent::UpdateCell),
            72 => ::std::option::Option::Some(GridEvent::UpdateCellSelectOption),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GridEvent] = &[
            GridEvent::GetGridData,
            GridEvent::GetGridBlocks,
            GridEvent::GetFields,
            GridEvent::UpdateField,
            GridEvent::InsertField,
            GridEvent::DeleteField,
            GridEvent::SwitchToField,
            GridEvent::DuplicateField,
            GridEvent::GetEditFieldContext,
            GridEvent::NewSelectOption,
            GridEvent::GetSelectOptionContext,
            GridEvent::UpdateSelectOption,
            GridEvent::CreateRow,
            GridEvent::GetRow,
            GridEvent::DeleteRow,
            GridEvent::DuplicateRow,
            GridEvent::GetCell,
            GridEvent::UpdateCell,
            GridEvent::UpdateCellSelectOption,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<GridEvent>("GridEvent", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for GridEvent {
}

impl ::std::default::Default for GridEvent {
    fn default() -> Self {
        GridEvent::GetGridData
    }
}

impl ::protobuf::reflect::ProtobufValue for GridEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fevent_map.proto*\xef\x02\n\tGridEvent\x12\x0f\n\x0bGetGridData\x10\
    \0\x12\x11\n\rGetGridBlocks\x10\x01\x12\r\n\tGetFields\x10\n\x12\x0f\n\
    \x0bUpdateField\x10\x0b\x12\x0f\n\x0bInsertField\x10\x0c\x12\x0f\n\x0bDe\
    leteField\x10\r\x12\x11\n\rSwitchToField\x10\x0e\x12\x12\n\x0eDuplicateF\
    ield\x10\x0f\x12\x17\n\x13GetEditFieldContext\x10\x10\x12\x13\n\x0fNewSe\
    lectOption\x10\x1e\x12\x1a\n\x16GetSelectOptionContext\x10\x1f\x12\x16\n\
    \x12UpdateSelectOption\x10\x20\x12\r\n\tCreateRow\x102\x12\n\n\x06GetRow\
    \x103\x12\r\n\tDeleteRow\x104\x12\x10\n\x0cDuplicateRow\x105\x12\x0b\n\
    \x07GetCell\x10F\x12\x0e\n\nUpdateCell\x10G\x12\x1a\n\x16UpdateCellSelec\
    tOption\x10Hb\x06proto3\
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
