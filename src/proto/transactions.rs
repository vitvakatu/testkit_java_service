// This file is generated by rust-protobuf 2.4.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CreateCounterTxBody {
    // message fields
    pub name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl CreateCounterTxBody {
    pub fn new() -> CreateCounterTxBody {
        ::std::default::Default::default()
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl ::protobuf::Message for CreateCounterTxBody {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CreateCounterTxBody {
        CreateCounterTxBody::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    |m: &CreateCounterTxBody| { &m.name },
                    |m: &mut CreateCounterTxBody| { &mut m.name },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateCounterTxBody>(
                    "CreateCounterTxBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CreateCounterTxBody {
        static mut instance: ::protobuf::lazy::Lazy<CreateCounterTxBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateCounterTxBody,
        };
        unsafe {
            instance.get(CreateCounterTxBody::new)
        }
    }
}

impl ::protobuf::Clear for CreateCounterTxBody {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateCounterTxBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateCounterTxBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IncrementCounterTxBody {
    // message fields
    pub seed: u64,
    pub counterId: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl IncrementCounterTxBody {
    pub fn new() -> IncrementCounterTxBody {
        ::std::default::Default::default()
    }

    // uint64 seed = 1;

    pub fn clear_seed(&mut self) {
        self.seed = 0;
    }

    // Param is passed by value, moved
    pub fn set_seed(&mut self, v: u64) {
        self.seed = v;
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    // bytes counterId = 2;

    pub fn clear_counterId(&mut self) {
        self.counterId.clear();
    }

    // Param is passed by value, moved
    pub fn set_counterId(&mut self, v: ::std::vec::Vec<u8>) {
        self.counterId = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_counterId(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.counterId
    }

    // Take field
    pub fn take_counterId(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.counterId, ::std::vec::Vec::new())
    }

    pub fn get_counterId(&self) -> &[u8] {
        &self.counterId
    }
}

impl ::protobuf::Message for IncrementCounterTxBody {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.seed = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.counterId)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.seed != 0 {
            my_size += ::protobuf::rt::value_size(1, self.seed, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.counterId.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.counterId);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.seed != 0 {
            os.write_uint64(1, self.seed)?;
        }
        if !self.counterId.is_empty() {
            os.write_bytes(2, &self.counterId)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> IncrementCounterTxBody {
        IncrementCounterTxBody::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "seed",
                    |m: &IncrementCounterTxBody| { &m.seed },
                    |m: &mut IncrementCounterTxBody| { &mut m.seed },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "counterId",
                    |m: &IncrementCounterTxBody| { &m.counterId },
                    |m: &mut IncrementCounterTxBody| { &mut m.counterId },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IncrementCounterTxBody>(
                    "IncrementCounterTxBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static IncrementCounterTxBody {
        static mut instance: ::protobuf::lazy::Lazy<IncrementCounterTxBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IncrementCounterTxBody,
        };
        unsafe {
            instance.get(IncrementCounterTxBody::new)
        }
    }
}

impl ::protobuf::Clear for IncrementCounterTxBody {
    fn clear(&mut self) {
        self.clear_seed();
        self.clear_counterId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IncrementCounterTxBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IncrementCounterTxBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ThrowingTxBody {
    // message fields
    pub seed: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl ThrowingTxBody {
    pub fn new() -> ThrowingTxBody {
        ::std::default::Default::default()
    }

    // uint64 seed = 1;

    pub fn clear_seed(&mut self) {
        self.seed = 0;
    }

    // Param is passed by value, moved
    pub fn set_seed(&mut self, v: u64) {
        self.seed = v;
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }
}

impl ::protobuf::Message for ThrowingTxBody {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.seed = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.seed != 0 {
            my_size += ::protobuf::rt::value_size(1, self.seed, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.seed != 0 {
            os.write_uint64(1, self.seed)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ThrowingTxBody {
        ThrowingTxBody::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "seed",
                    |m: &ThrowingTxBody| { &m.seed },
                    |m: &mut ThrowingTxBody| { &mut m.seed },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ThrowingTxBody>(
                    "ThrowingTxBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ThrowingTxBody {
        static mut instance: ::protobuf::lazy::Lazy<ThrowingTxBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ThrowingTxBody,
        };
        unsafe {
            instance.get(ThrowingTxBody::new)
        }
    }
}

impl ::protobuf::Clear for ThrowingTxBody {
    fn clear(&mut self) {
        self.clear_seed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ThrowingTxBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThrowingTxBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ErrorTxBody {
    // message fields
    pub seed: u64,
    pub errorCode: i32,
    pub errorDescription: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl ErrorTxBody {
    pub fn new() -> ErrorTxBody {
        ::std::default::Default::default()
    }

    // uint64 seed = 1;

    pub fn clear_seed(&mut self) {
        self.seed = 0;
    }

    // Param is passed by value, moved
    pub fn set_seed(&mut self, v: u64) {
        self.seed = v;
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    // int32 errorCode = 2;

    pub fn clear_errorCode(&mut self) {
        self.errorCode = 0;
    }

    // Param is passed by value, moved
    pub fn set_errorCode(&mut self, v: i32) {
        self.errorCode = v;
    }

    pub fn get_errorCode(&self) -> i32 {
        self.errorCode
    }

    // string errorDescription = 3;

    pub fn clear_errorDescription(&mut self) {
        self.errorDescription.clear();
    }

    // Param is passed by value, moved
    pub fn set_errorDescription(&mut self, v: ::std::string::String) {
        self.errorDescription = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errorDescription(&mut self) -> &mut ::std::string::String {
        &mut self.errorDescription
    }

    // Take field
    pub fn take_errorDescription(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.errorDescription, ::std::string::String::new())
    }

    pub fn get_errorDescription(&self) -> &str {
        &self.errorDescription
    }
}

impl ::protobuf::Message for ErrorTxBody {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.seed = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.errorCode = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.errorDescription)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.seed != 0 {
            my_size += ::protobuf::rt::value_size(1, self.seed, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.errorCode != 0 {
            my_size += ::protobuf::rt::value_size(2, self.errorCode, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.errorDescription.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.errorDescription);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.seed != 0 {
            os.write_uint64(1, self.seed)?;
        }
        if self.errorCode != 0 {
            os.write_int32(2, self.errorCode)?;
        }
        if !self.errorDescription.is_empty() {
            os.write_string(3, &self.errorDescription)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ErrorTxBody {
        ErrorTxBody::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "seed",
                    |m: &ErrorTxBody| { &m.seed },
                    |m: &mut ErrorTxBody| { &mut m.seed },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "errorCode",
                    |m: &ErrorTxBody| { &m.errorCode },
                    |m: &mut ErrorTxBody| { &mut m.errorCode },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "errorDescription",
                    |m: &ErrorTxBody| { &m.errorDescription },
                    |m: &mut ErrorTxBody| { &mut m.errorDescription },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ErrorTxBody>(
                    "ErrorTxBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ErrorTxBody {
        static mut instance: ::protobuf::lazy::Lazy<ErrorTxBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErrorTxBody,
        };
        unsafe {
            instance.get(ErrorTxBody::new)
        }
    }
}

impl ::protobuf::Clear for ErrorTxBody {
    fn clear(&mut self) {
        self.clear_seed();
        self.clear_errorCode();
        self.clear_errorDescription();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ErrorTxBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorTxBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12transactions.proto\")\n\x13CreateCounterTxBody\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\"J\n\x16IncrementCounterTxBody\x12\x12\n\
    \x04seed\x18\x01\x20\x01(\x04R\x04seed\x12\x1c\n\tcounterId\x18\x02\x20\
    \x01(\x0cR\tcounterId\"$\n\x0eThrowingTxBody\x12\x12\n\x04seed\x18\x01\
    \x20\x01(\x04R\x04seed\"k\n\x0bErrorTxBody\x12\x12\n\x04seed\x18\x01\x20\
    \x01(\x04R\x04seed\x12\x1c\n\terrorCode\x18\x02\x20\x01(\x05R\terrorCode\
    \x12*\n\x10errorDescription\x18\x03\x20\x01(\tR\x10errorDescriptionB<\n)\
    com.exonum.binding.qaservice.transactionsB\x0fTxMessageProtosb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
