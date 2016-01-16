// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct KillProcessRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl KillProcessRequest {
    pub fn new() -> KillProcessRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KillProcessRequest {
        static mut instance: ::protobuf::lazy::Lazy<KillProcessRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KillProcessRequest,
        };
        unsafe {
            instance.get(|| {
                KillProcessRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for KillProcessRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<KillProcessRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KillProcessRequest {
    fn new() -> KillProcessRequest {
        KillProcessRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<KillProcessRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<KillProcessRequest>(
                    "KillProcessRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KillProcessRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for KillProcessRequest {
    fn eq(&self, other: &KillProcessRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for KillProcessRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecutionStatusResponse {
    // message fields
    executionResult: ::protobuf::SingularPtrField<super::spec::ProtoExecutionResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecutionStatusResponse {
    pub fn new() -> ExecutionStatusResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutionStatusResponse {
        static mut instance: ::protobuf::lazy::Lazy<ExecutionStatusResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutionStatusResponse,
        };
        unsafe {
            instance.get(|| {
                ExecutionStatusResponse {
                    executionResult: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.ProtoExecutionResult executionResult = 1;

    pub fn clear_executionResult(&mut self) {
        self.executionResult.clear();
    }

    pub fn has_executionResult(&self) -> bool {
        self.executionResult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executionResult(&mut self, v: super::spec::ProtoExecutionResult) {
        self.executionResult = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executionResult<'a>(&'a mut self) -> &'a mut super::spec::ProtoExecutionResult {
        if self.executionResult.is_none() {
            self.executionResult.set_default();
        };
        self.executionResult.as_mut().unwrap()
    }

    // Take field
    pub fn take_executionResult(&mut self) -> super::spec::ProtoExecutionResult {
        self.executionResult.take().unwrap_or_else(|| super::spec::ProtoExecutionResult::new())
    }

    pub fn get_executionResult<'a>(&'a self) -> &'a super::spec::ProtoExecutionResult {
        self.executionResult.as_ref().unwrap_or_else(|| super::spec::ProtoExecutionResult::default_instance())
    }
}

impl ::protobuf::Message for ExecutionStatusResponse {
    fn is_initialized(&self) -> bool {
        if self.executionResult.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executionResult));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.executionResult.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.executionResult.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecutionStatusResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecutionStatusResponse {
    fn new() -> ExecutionStatusResponse {
        ExecutionStatusResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutionStatusResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executionResult",
                    ExecutionStatusResponse::has_executionResult,
                    ExecutionStatusResponse::get_executionResult,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutionStatusResponse>(
                    "ExecutionStatusResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutionStatusResponse {
    fn clear(&mut self) {
        self.clear_executionResult();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecutionStatusResponse {
    fn eq(&self, other: &ExecutionStatusResponse) -> bool {
        self.executionResult == other.executionResult &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecutionStatusResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecutionStartingRequest {
    // message fields
    currentExecutionInfo: ::protobuf::SingularPtrField<ExecutionInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecutionStartingRequest {
    pub fn new() -> ExecutionStartingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutionStartingRequest {
        static mut instance: ::protobuf::lazy::Lazy<ExecutionStartingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutionStartingRequest,
        };
        unsafe {
            instance.get(|| {
                ExecutionStartingRequest {
                    currentExecutionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.ExecutionInfo currentExecutionInfo = 1;

    pub fn clear_currentExecutionInfo(&mut self) {
        self.currentExecutionInfo.clear();
    }

    pub fn has_currentExecutionInfo(&self) -> bool {
        self.currentExecutionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentExecutionInfo(&mut self, v: ExecutionInfo) {
        self.currentExecutionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentExecutionInfo<'a>(&'a mut self) -> &'a mut ExecutionInfo {
        if self.currentExecutionInfo.is_none() {
            self.currentExecutionInfo.set_default();
        };
        self.currentExecutionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentExecutionInfo(&mut self) -> ExecutionInfo {
        self.currentExecutionInfo.take().unwrap_or_else(|| ExecutionInfo::new())
    }

    pub fn get_currentExecutionInfo<'a>(&'a self) -> &'a ExecutionInfo {
        self.currentExecutionInfo.as_ref().unwrap_or_else(|| ExecutionInfo::default_instance())
    }
}

impl ::protobuf::Message for ExecutionStartingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentExecutionInfo));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentExecutionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentExecutionInfo.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecutionStartingRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecutionStartingRequest {
    fn new() -> ExecutionStartingRequest {
        ExecutionStartingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutionStartingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentExecutionInfo",
                    ExecutionStartingRequest::has_currentExecutionInfo,
                    ExecutionStartingRequest::get_currentExecutionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutionStartingRequest>(
                    "ExecutionStartingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutionStartingRequest {
    fn clear(&mut self) {
        self.clear_currentExecutionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecutionStartingRequest {
    fn eq(&self, other: &ExecutionStartingRequest) -> bool {
        self.currentExecutionInfo == other.currentExecutionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecutionStartingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecutionEndingRequest {
    // message fields
    currentExecutionInfo: ::protobuf::SingularPtrField<ExecutionInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecutionEndingRequest {
    pub fn new() -> ExecutionEndingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutionEndingRequest {
        static mut instance: ::protobuf::lazy::Lazy<ExecutionEndingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutionEndingRequest,
        };
        unsafe {
            instance.get(|| {
                ExecutionEndingRequest {
                    currentExecutionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.ExecutionInfo currentExecutionInfo = 1;

    pub fn clear_currentExecutionInfo(&mut self) {
        self.currentExecutionInfo.clear();
    }

    pub fn has_currentExecutionInfo(&self) -> bool {
        self.currentExecutionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentExecutionInfo(&mut self, v: ExecutionInfo) {
        self.currentExecutionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentExecutionInfo<'a>(&'a mut self) -> &'a mut ExecutionInfo {
        if self.currentExecutionInfo.is_none() {
            self.currentExecutionInfo.set_default();
        };
        self.currentExecutionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentExecutionInfo(&mut self) -> ExecutionInfo {
        self.currentExecutionInfo.take().unwrap_or_else(|| ExecutionInfo::new())
    }

    pub fn get_currentExecutionInfo<'a>(&'a self) -> &'a ExecutionInfo {
        self.currentExecutionInfo.as_ref().unwrap_or_else(|| ExecutionInfo::default_instance())
    }
}

impl ::protobuf::Message for ExecutionEndingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentExecutionInfo));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentExecutionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentExecutionInfo.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecutionEndingRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecutionEndingRequest {
    fn new() -> ExecutionEndingRequest {
        ExecutionEndingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutionEndingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentExecutionInfo",
                    ExecutionEndingRequest::has_currentExecutionInfo,
                    ExecutionEndingRequest::get_currentExecutionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutionEndingRequest>(
                    "ExecutionEndingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutionEndingRequest {
    fn clear(&mut self) {
        self.clear_currentExecutionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecutionEndingRequest {
    fn eq(&self, other: &ExecutionEndingRequest) -> bool {
        self.currentExecutionInfo == other.currentExecutionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecutionEndingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SpecExecutionStartingRequest {
    // message fields
    currentExecutionInfo: ::protobuf::SingularPtrField<ExecutionInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SpecExecutionStartingRequest {
    pub fn new() -> SpecExecutionStartingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpecExecutionStartingRequest {
        static mut instance: ::protobuf::lazy::Lazy<SpecExecutionStartingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpecExecutionStartingRequest,
        };
        unsafe {
            instance.get(|| {
                SpecExecutionStartingRequest {
                    currentExecutionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.ExecutionInfo currentExecutionInfo = 1;

    pub fn clear_currentExecutionInfo(&mut self) {
        self.currentExecutionInfo.clear();
    }

    pub fn has_currentExecutionInfo(&self) -> bool {
        self.currentExecutionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentExecutionInfo(&mut self, v: ExecutionInfo) {
        self.currentExecutionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentExecutionInfo<'a>(&'a mut self) -> &'a mut ExecutionInfo {
        if self.currentExecutionInfo.is_none() {
            self.currentExecutionInfo.set_default();
        };
        self.currentExecutionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentExecutionInfo(&mut self) -> ExecutionInfo {
        self.currentExecutionInfo.take().unwrap_or_else(|| ExecutionInfo::new())
    }

    pub fn get_currentExecutionInfo<'a>(&'a self) -> &'a ExecutionInfo {
        self.currentExecutionInfo.as_ref().unwrap_or_else(|| ExecutionInfo::default_instance())
    }
}

impl ::protobuf::Message for SpecExecutionStartingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentExecutionInfo));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentExecutionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentExecutionInfo.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SpecExecutionStartingRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SpecExecutionStartingRequest {
    fn new() -> SpecExecutionStartingRequest {
        SpecExecutionStartingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpecExecutionStartingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentExecutionInfo",
                    SpecExecutionStartingRequest::has_currentExecutionInfo,
                    SpecExecutionStartingRequest::get_currentExecutionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpecExecutionStartingRequest>(
                    "SpecExecutionStartingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpecExecutionStartingRequest {
    fn clear(&mut self) {
        self.clear_currentExecutionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SpecExecutionStartingRequest {
    fn eq(&self, other: &SpecExecutionStartingRequest) -> bool {
        self.currentExecutionInfo == other.currentExecutionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SpecExecutionStartingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SpecExecutionEndingRequest {
    // message fields
    currentExecutionInfo: ::protobuf::SingularPtrField<ExecutionInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SpecExecutionEndingRequest {
    pub fn new() -> SpecExecutionEndingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpecExecutionEndingRequest {
        static mut instance: ::protobuf::lazy::Lazy<SpecExecutionEndingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpecExecutionEndingRequest,
        };
        unsafe {
            instance.get(|| {
                SpecExecutionEndingRequest {
                    currentExecutionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.ExecutionInfo currentExecutionInfo = 1;

    pub fn clear_currentExecutionInfo(&mut self) {
        self.currentExecutionInfo.clear();
    }

    pub fn has_currentExecutionInfo(&self) -> bool {
        self.currentExecutionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentExecutionInfo(&mut self, v: ExecutionInfo) {
        self.currentExecutionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentExecutionInfo<'a>(&'a mut self) -> &'a mut ExecutionInfo {
        if self.currentExecutionInfo.is_none() {
            self.currentExecutionInfo.set_default();
        };
        self.currentExecutionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentExecutionInfo(&mut self) -> ExecutionInfo {
        self.currentExecutionInfo.take().unwrap_or_else(|| ExecutionInfo::new())
    }

    pub fn get_currentExecutionInfo<'a>(&'a self) -> &'a ExecutionInfo {
        self.currentExecutionInfo.as_ref().unwrap_or_else(|| ExecutionInfo::default_instance())
    }
}

impl ::protobuf::Message for SpecExecutionEndingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentExecutionInfo));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentExecutionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentExecutionInfo.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SpecExecutionEndingRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SpecExecutionEndingRequest {
    fn new() -> SpecExecutionEndingRequest {
        SpecExecutionEndingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpecExecutionEndingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentExecutionInfo",
                    SpecExecutionEndingRequest::has_currentExecutionInfo,
                    SpecExecutionEndingRequest::get_currentExecutionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpecExecutionEndingRequest>(
                    "SpecExecutionEndingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpecExecutionEndingRequest {
    fn clear(&mut self) {
        self.clear_currentExecutionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SpecExecutionEndingRequest {
    fn eq(&self, other: &SpecExecutionEndingRequest) -> bool {
        self.currentExecutionInfo == other.currentExecutionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SpecExecutionEndingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ScenarioExecutionStartingRequest {
    // message fields
    currentExecutionInfo: ::protobuf::SingularPtrField<ExecutionInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ScenarioExecutionStartingRequest {
    pub fn new() -> ScenarioExecutionStartingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScenarioExecutionStartingRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScenarioExecutionStartingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScenarioExecutionStartingRequest,
        };
        unsafe {
            instance.get(|| {
                ScenarioExecutionStartingRequest {
                    currentExecutionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.ExecutionInfo currentExecutionInfo = 1;

    pub fn clear_currentExecutionInfo(&mut self) {
        self.currentExecutionInfo.clear();
    }

    pub fn has_currentExecutionInfo(&self) -> bool {
        self.currentExecutionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentExecutionInfo(&mut self, v: ExecutionInfo) {
        self.currentExecutionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentExecutionInfo<'a>(&'a mut self) -> &'a mut ExecutionInfo {
        if self.currentExecutionInfo.is_none() {
            self.currentExecutionInfo.set_default();
        };
        self.currentExecutionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentExecutionInfo(&mut self) -> ExecutionInfo {
        self.currentExecutionInfo.take().unwrap_or_else(|| ExecutionInfo::new())
    }

    pub fn get_currentExecutionInfo<'a>(&'a self) -> &'a ExecutionInfo {
        self.currentExecutionInfo.as_ref().unwrap_or_else(|| ExecutionInfo::default_instance())
    }
}

impl ::protobuf::Message for ScenarioExecutionStartingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentExecutionInfo));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentExecutionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentExecutionInfo.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ScenarioExecutionStartingRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScenarioExecutionStartingRequest {
    fn new() -> ScenarioExecutionStartingRequest {
        ScenarioExecutionStartingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScenarioExecutionStartingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentExecutionInfo",
                    ScenarioExecutionStartingRequest::has_currentExecutionInfo,
                    ScenarioExecutionStartingRequest::get_currentExecutionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScenarioExecutionStartingRequest>(
                    "ScenarioExecutionStartingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScenarioExecutionStartingRequest {
    fn clear(&mut self) {
        self.clear_currentExecutionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScenarioExecutionStartingRequest {
    fn eq(&self, other: &ScenarioExecutionStartingRequest) -> bool {
        self.currentExecutionInfo == other.currentExecutionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScenarioExecutionStartingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ScenarioExecutionEndingRequest {
    // message fields
    currentExecutionInfo: ::protobuf::SingularPtrField<ExecutionInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ScenarioExecutionEndingRequest {
    pub fn new() -> ScenarioExecutionEndingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScenarioExecutionEndingRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScenarioExecutionEndingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScenarioExecutionEndingRequest,
        };
        unsafe {
            instance.get(|| {
                ScenarioExecutionEndingRequest {
                    currentExecutionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.ExecutionInfo currentExecutionInfo = 1;

    pub fn clear_currentExecutionInfo(&mut self) {
        self.currentExecutionInfo.clear();
    }

    pub fn has_currentExecutionInfo(&self) -> bool {
        self.currentExecutionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentExecutionInfo(&mut self, v: ExecutionInfo) {
        self.currentExecutionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentExecutionInfo<'a>(&'a mut self) -> &'a mut ExecutionInfo {
        if self.currentExecutionInfo.is_none() {
            self.currentExecutionInfo.set_default();
        };
        self.currentExecutionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentExecutionInfo(&mut self) -> ExecutionInfo {
        self.currentExecutionInfo.take().unwrap_or_else(|| ExecutionInfo::new())
    }

    pub fn get_currentExecutionInfo<'a>(&'a self) -> &'a ExecutionInfo {
        self.currentExecutionInfo.as_ref().unwrap_or_else(|| ExecutionInfo::default_instance())
    }
}

impl ::protobuf::Message for ScenarioExecutionEndingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentExecutionInfo));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentExecutionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentExecutionInfo.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ScenarioExecutionEndingRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScenarioExecutionEndingRequest {
    fn new() -> ScenarioExecutionEndingRequest {
        ScenarioExecutionEndingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScenarioExecutionEndingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentExecutionInfo",
                    ScenarioExecutionEndingRequest::has_currentExecutionInfo,
                    ScenarioExecutionEndingRequest::get_currentExecutionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScenarioExecutionEndingRequest>(
                    "ScenarioExecutionEndingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScenarioExecutionEndingRequest {
    fn clear(&mut self) {
        self.clear_currentExecutionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScenarioExecutionEndingRequest {
    fn eq(&self, other: &ScenarioExecutionEndingRequest) -> bool {
        self.currentExecutionInfo == other.currentExecutionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScenarioExecutionEndingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepExecutionStartingRequest {
    // message fields
    currentExecutionInfo: ::protobuf::SingularPtrField<ExecutionInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepExecutionStartingRequest {
    pub fn new() -> StepExecutionStartingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepExecutionStartingRequest {
        static mut instance: ::protobuf::lazy::Lazy<StepExecutionStartingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepExecutionStartingRequest,
        };
        unsafe {
            instance.get(|| {
                StepExecutionStartingRequest {
                    currentExecutionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.ExecutionInfo currentExecutionInfo = 1;

    pub fn clear_currentExecutionInfo(&mut self) {
        self.currentExecutionInfo.clear();
    }

    pub fn has_currentExecutionInfo(&self) -> bool {
        self.currentExecutionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentExecutionInfo(&mut self, v: ExecutionInfo) {
        self.currentExecutionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentExecutionInfo<'a>(&'a mut self) -> &'a mut ExecutionInfo {
        if self.currentExecutionInfo.is_none() {
            self.currentExecutionInfo.set_default();
        };
        self.currentExecutionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentExecutionInfo(&mut self) -> ExecutionInfo {
        self.currentExecutionInfo.take().unwrap_or_else(|| ExecutionInfo::new())
    }

    pub fn get_currentExecutionInfo<'a>(&'a self) -> &'a ExecutionInfo {
        self.currentExecutionInfo.as_ref().unwrap_or_else(|| ExecutionInfo::default_instance())
    }
}

impl ::protobuf::Message for StepExecutionStartingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentExecutionInfo));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentExecutionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentExecutionInfo.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepExecutionStartingRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepExecutionStartingRequest {
    fn new() -> StepExecutionStartingRequest {
        StepExecutionStartingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepExecutionStartingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentExecutionInfo",
                    StepExecutionStartingRequest::has_currentExecutionInfo,
                    StepExecutionStartingRequest::get_currentExecutionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepExecutionStartingRequest>(
                    "StepExecutionStartingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepExecutionStartingRequest {
    fn clear(&mut self) {
        self.clear_currentExecutionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepExecutionStartingRequest {
    fn eq(&self, other: &StepExecutionStartingRequest) -> bool {
        self.currentExecutionInfo == other.currentExecutionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepExecutionStartingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepExecutionEndingRequest {
    // message fields
    currentExecutionInfo: ::protobuf::SingularPtrField<ExecutionInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepExecutionEndingRequest {
    pub fn new() -> StepExecutionEndingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepExecutionEndingRequest {
        static mut instance: ::protobuf::lazy::Lazy<StepExecutionEndingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepExecutionEndingRequest,
        };
        unsafe {
            instance.get(|| {
                StepExecutionEndingRequest {
                    currentExecutionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.ExecutionInfo currentExecutionInfo = 1;

    pub fn clear_currentExecutionInfo(&mut self) {
        self.currentExecutionInfo.clear();
    }

    pub fn has_currentExecutionInfo(&self) -> bool {
        self.currentExecutionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentExecutionInfo(&mut self, v: ExecutionInfo) {
        self.currentExecutionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentExecutionInfo<'a>(&'a mut self) -> &'a mut ExecutionInfo {
        if self.currentExecutionInfo.is_none() {
            self.currentExecutionInfo.set_default();
        };
        self.currentExecutionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentExecutionInfo(&mut self) -> ExecutionInfo {
        self.currentExecutionInfo.take().unwrap_or_else(|| ExecutionInfo::new())
    }

    pub fn get_currentExecutionInfo<'a>(&'a self) -> &'a ExecutionInfo {
        self.currentExecutionInfo.as_ref().unwrap_or_else(|| ExecutionInfo::default_instance())
    }
}

impl ::protobuf::Message for StepExecutionEndingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentExecutionInfo));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentExecutionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentExecutionInfo.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepExecutionEndingRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepExecutionEndingRequest {
    fn new() -> StepExecutionEndingRequest {
        StepExecutionEndingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepExecutionEndingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentExecutionInfo",
                    StepExecutionEndingRequest::has_currentExecutionInfo,
                    StepExecutionEndingRequest::get_currentExecutionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepExecutionEndingRequest>(
                    "StepExecutionEndingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepExecutionEndingRequest {
    fn clear(&mut self) {
        self.clear_currentExecutionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepExecutionEndingRequest {
    fn eq(&self, other: &StepExecutionEndingRequest) -> bool {
        self.currentExecutionInfo == other.currentExecutionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepExecutionEndingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecutionInfo {
    // message fields
    currentSpec: ::protobuf::SingularPtrField<SpecInfo>,
    currentScenario: ::protobuf::SingularPtrField<ScenarioInfo>,
    currentStep: ::protobuf::SingularPtrField<StepInfo>,
    stacktrace: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecutionInfo {
    pub fn new() -> ExecutionInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutionInfo {
        static mut instance: ::protobuf::lazy::Lazy<ExecutionInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutionInfo,
        };
        unsafe {
            instance.get(|| {
                ExecutionInfo {
                    currentSpec: ::protobuf::SingularPtrField::none(),
                    currentScenario: ::protobuf::SingularPtrField::none(),
                    currentStep: ::protobuf::SingularPtrField::none(),
                    stacktrace: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .gauge.messages.SpecInfo currentSpec = 1;

    pub fn clear_currentSpec(&mut self) {
        self.currentSpec.clear();
    }

    pub fn has_currentSpec(&self) -> bool {
        self.currentSpec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentSpec(&mut self, v: SpecInfo) {
        self.currentSpec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentSpec<'a>(&'a mut self) -> &'a mut SpecInfo {
        if self.currentSpec.is_none() {
            self.currentSpec.set_default();
        };
        self.currentSpec.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentSpec(&mut self) -> SpecInfo {
        self.currentSpec.take().unwrap_or_else(|| SpecInfo::new())
    }

    pub fn get_currentSpec<'a>(&'a self) -> &'a SpecInfo {
        self.currentSpec.as_ref().unwrap_or_else(|| SpecInfo::default_instance())
    }

    // optional .gauge.messages.ScenarioInfo currentScenario = 2;

    pub fn clear_currentScenario(&mut self) {
        self.currentScenario.clear();
    }

    pub fn has_currentScenario(&self) -> bool {
        self.currentScenario.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentScenario(&mut self, v: ScenarioInfo) {
        self.currentScenario = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentScenario<'a>(&'a mut self) -> &'a mut ScenarioInfo {
        if self.currentScenario.is_none() {
            self.currentScenario.set_default();
        };
        self.currentScenario.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentScenario(&mut self) -> ScenarioInfo {
        self.currentScenario.take().unwrap_or_else(|| ScenarioInfo::new())
    }

    pub fn get_currentScenario<'a>(&'a self) -> &'a ScenarioInfo {
        self.currentScenario.as_ref().unwrap_or_else(|| ScenarioInfo::default_instance())
    }

    // optional .gauge.messages.StepInfo currentStep = 3;

    pub fn clear_currentStep(&mut self) {
        self.currentStep.clear();
    }

    pub fn has_currentStep(&self) -> bool {
        self.currentStep.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentStep(&mut self, v: StepInfo) {
        self.currentStep = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentStep<'a>(&'a mut self) -> &'a mut StepInfo {
        if self.currentStep.is_none() {
            self.currentStep.set_default();
        };
        self.currentStep.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentStep(&mut self) -> StepInfo {
        self.currentStep.take().unwrap_or_else(|| StepInfo::new())
    }

    pub fn get_currentStep<'a>(&'a self) -> &'a StepInfo {
        self.currentStep.as_ref().unwrap_or_else(|| StepInfo::default_instance())
    }

    // optional string stacktrace = 4;

    pub fn clear_stacktrace(&mut self) {
        self.stacktrace.clear();
    }

    pub fn has_stacktrace(&self) -> bool {
        self.stacktrace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stacktrace(&mut self, v: ::std::string::String) {
        self.stacktrace = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stacktrace<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.stacktrace.is_none() {
            self.stacktrace.set_default();
        };
        self.stacktrace.as_mut().unwrap()
    }

    // Take field
    pub fn take_stacktrace(&mut self) -> ::std::string::String {
        self.stacktrace.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_stacktrace<'a>(&'a self) -> &'a str {
        match self.stacktrace.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ExecutionInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentSpec));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentScenario));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentStep));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.stacktrace));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.currentSpec.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.currentScenario.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.currentStep.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stacktrace.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentSpec.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.currentScenario.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.currentStep.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stacktrace.as_ref() {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecutionInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecutionInfo {
    fn new() -> ExecutionInfo {
        ExecutionInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutionInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentSpec",
                    ExecutionInfo::has_currentSpec,
                    ExecutionInfo::get_currentSpec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentScenario",
                    ExecutionInfo::has_currentScenario,
                    ExecutionInfo::get_currentScenario,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentStep",
                    ExecutionInfo::has_currentStep,
                    ExecutionInfo::get_currentStep,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "stacktrace",
                    ExecutionInfo::has_stacktrace,
                    ExecutionInfo::get_stacktrace,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutionInfo>(
                    "ExecutionInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutionInfo {
    fn clear(&mut self) {
        self.clear_currentSpec();
        self.clear_currentScenario();
        self.clear_currentStep();
        self.clear_stacktrace();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecutionInfo {
    fn eq(&self, other: &ExecutionInfo) -> bool {
        self.currentSpec == other.currentSpec &&
        self.currentScenario == other.currentScenario &&
        self.currentStep == other.currentStep &&
        self.stacktrace == other.stacktrace &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecutionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SpecInfo {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    fileName: ::protobuf::SingularField<::std::string::String>,
    isFailed: ::std::option::Option<bool>,
    tags: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SpecInfo {
    pub fn new() -> SpecInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpecInfo {
        static mut instance: ::protobuf::lazy::Lazy<SpecInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpecInfo,
        };
        unsafe {
            instance.get(|| {
                SpecInfo {
                    name: ::protobuf::SingularField::none(),
                    fileName: ::protobuf::SingularField::none(),
                    isFailed: ::std::option::Option::None,
                    tags: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string fileName = 2;

    pub fn clear_fileName(&mut self) {
        self.fileName.clear();
    }

    pub fn has_fileName(&self) -> bool {
        self.fileName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileName(&mut self, v: ::std::string::String) {
        self.fileName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.fileName.is_none() {
            self.fileName.set_default();
        };
        self.fileName.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileName(&mut self) -> ::std::string::String {
        self.fileName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fileName<'a>(&'a self) -> &'a str {
        match self.fileName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required bool isFailed = 3;

    pub fn clear_isFailed(&mut self) {
        self.isFailed = ::std::option::Option::None;
    }

    pub fn has_isFailed(&self) -> bool {
        self.isFailed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isFailed(&mut self, v: bool) {
        self.isFailed = ::std::option::Option::Some(v);
    }

    pub fn get_isFailed<'a>(&self) -> bool {
        self.isFailed.unwrap_or(false)
    }

    // repeated string tags = 4;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags<'a>(&'a self) -> &'a [::std::string::String] {
        &self.tags
    }
}

impl ::protobuf::Message for SpecInfo {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.fileName.is_none() {
            return false;
        };
        if self.isFailed.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fileName));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.isFailed = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.fileName.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.isFailed.is_some() {
            my_size += 2;
        };
        for value in self.tags.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.fileName.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.isFailed {
            try!(os.write_bool(3, v));
        };
        for v in self.tags.iter() {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SpecInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SpecInfo {
    fn new() -> SpecInfo {
        SpecInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpecInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    SpecInfo::has_name,
                    SpecInfo::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fileName",
                    SpecInfo::has_fileName,
                    SpecInfo::get_fileName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isFailed",
                    SpecInfo::has_isFailed,
                    SpecInfo::get_isFailed,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "tags",
                    SpecInfo::get_tags,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpecInfo>(
                    "SpecInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpecInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_fileName();
        self.clear_isFailed();
        self.clear_tags();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SpecInfo {
    fn eq(&self, other: &SpecInfo) -> bool {
        self.name == other.name &&
        self.fileName == other.fileName &&
        self.isFailed == other.isFailed &&
        self.tags == other.tags &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SpecInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ScenarioInfo {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    isFailed: ::std::option::Option<bool>,
    tags: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ScenarioInfo {
    pub fn new() -> ScenarioInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScenarioInfo {
        static mut instance: ::protobuf::lazy::Lazy<ScenarioInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScenarioInfo,
        };
        unsafe {
            instance.get(|| {
                ScenarioInfo {
                    name: ::protobuf::SingularField::none(),
                    isFailed: ::std::option::Option::None,
                    tags: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required bool isFailed = 2;

    pub fn clear_isFailed(&mut self) {
        self.isFailed = ::std::option::Option::None;
    }

    pub fn has_isFailed(&self) -> bool {
        self.isFailed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isFailed(&mut self, v: bool) {
        self.isFailed = ::std::option::Option::Some(v);
    }

    pub fn get_isFailed<'a>(&self) -> bool {
        self.isFailed.unwrap_or(false)
    }

    // repeated string tags = 3;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags<'a>(&'a self) -> &'a [::std::string::String] {
        &self.tags
    }
}

impl ::protobuf::Message for ScenarioInfo {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.isFailed.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.isFailed = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.isFailed.is_some() {
            my_size += 2;
        };
        for value in self.tags.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.isFailed {
            try!(os.write_bool(2, v));
        };
        for v in self.tags.iter() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ScenarioInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScenarioInfo {
    fn new() -> ScenarioInfo {
        ScenarioInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScenarioInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    ScenarioInfo::has_name,
                    ScenarioInfo::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isFailed",
                    ScenarioInfo::has_isFailed,
                    ScenarioInfo::get_isFailed,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "tags",
                    ScenarioInfo::get_tags,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScenarioInfo>(
                    "ScenarioInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScenarioInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_isFailed();
        self.clear_tags();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScenarioInfo {
    fn eq(&self, other: &ScenarioInfo) -> bool {
        self.name == other.name &&
        self.isFailed == other.isFailed &&
        self.tags == other.tags &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScenarioInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepInfo {
    // message fields
    step: ::protobuf::SingularPtrField<ExecuteStepRequest>,
    isFailed: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepInfo {
    pub fn new() -> StepInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepInfo {
        static mut instance: ::protobuf::lazy::Lazy<StepInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepInfo,
        };
        unsafe {
            instance.get(|| {
                StepInfo {
                    step: ::protobuf::SingularPtrField::none(),
                    isFailed: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.ExecuteStepRequest step = 1;

    pub fn clear_step(&mut self) {
        self.step.clear();
    }

    pub fn has_step(&self) -> bool {
        self.step.is_some()
    }

    // Param is passed by value, moved
    pub fn set_step(&mut self, v: ExecuteStepRequest) {
        self.step = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_step<'a>(&'a mut self) -> &'a mut ExecuteStepRequest {
        if self.step.is_none() {
            self.step.set_default();
        };
        self.step.as_mut().unwrap()
    }

    // Take field
    pub fn take_step(&mut self) -> ExecuteStepRequest {
        self.step.take().unwrap_or_else(|| ExecuteStepRequest::new())
    }

    pub fn get_step<'a>(&'a self) -> &'a ExecuteStepRequest {
        self.step.as_ref().unwrap_or_else(|| ExecuteStepRequest::default_instance())
    }

    // required bool isFailed = 2;

    pub fn clear_isFailed(&mut self) {
        self.isFailed = ::std::option::Option::None;
    }

    pub fn has_isFailed(&self) -> bool {
        self.isFailed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isFailed(&mut self, v: bool) {
        self.isFailed = ::std::option::Option::Some(v);
    }

    pub fn get_isFailed<'a>(&self) -> bool {
        self.isFailed.unwrap_or(false)
    }
}

impl ::protobuf::Message for StepInfo {
    fn is_initialized(&self) -> bool {
        if self.step.is_none() {
            return false;
        };
        if self.isFailed.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.step));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.isFailed = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.step.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.isFailed.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.step.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.isFailed {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepInfo {
    fn new() -> StepInfo {
        StepInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "step",
                    StepInfo::has_step,
                    StepInfo::get_step,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isFailed",
                    StepInfo::has_isFailed,
                    StepInfo::get_isFailed,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepInfo>(
                    "StepInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepInfo {
    fn clear(&mut self) {
        self.clear_step();
        self.clear_isFailed();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepInfo {
    fn eq(&self, other: &StepInfo) -> bool {
        self.step == other.step &&
        self.isFailed == other.isFailed &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecuteStepRequest {
    // message fields
    actualStepText: ::protobuf::SingularField<::std::string::String>,
    parsedStepText: ::protobuf::SingularField<::std::string::String>,
    scenarioFailing: ::std::option::Option<bool>,
    parameters: ::protobuf::RepeatedField<super::spec::Parameter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecuteStepRequest {
    pub fn new() -> ExecuteStepRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecuteStepRequest {
        static mut instance: ::protobuf::lazy::Lazy<ExecuteStepRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecuteStepRequest,
        };
        unsafe {
            instance.get(|| {
                ExecuteStepRequest {
                    actualStepText: ::protobuf::SingularField::none(),
                    parsedStepText: ::protobuf::SingularField::none(),
                    scenarioFailing: ::std::option::Option::None,
                    parameters: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string actualStepText = 1;

    pub fn clear_actualStepText(&mut self) {
        self.actualStepText.clear();
    }

    pub fn has_actualStepText(&self) -> bool {
        self.actualStepText.is_some()
    }

    // Param is passed by value, moved
    pub fn set_actualStepText(&mut self, v: ::std::string::String) {
        self.actualStepText = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_actualStepText<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.actualStepText.is_none() {
            self.actualStepText.set_default();
        };
        self.actualStepText.as_mut().unwrap()
    }

    // Take field
    pub fn take_actualStepText(&mut self) -> ::std::string::String {
        self.actualStepText.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_actualStepText<'a>(&'a self) -> &'a str {
        match self.actualStepText.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string parsedStepText = 2;

    pub fn clear_parsedStepText(&mut self) {
        self.parsedStepText.clear();
    }

    pub fn has_parsedStepText(&self) -> bool {
        self.parsedStepText.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parsedStepText(&mut self, v: ::std::string::String) {
        self.parsedStepText = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parsedStepText<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.parsedStepText.is_none() {
            self.parsedStepText.set_default();
        };
        self.parsedStepText.as_mut().unwrap()
    }

    // Take field
    pub fn take_parsedStepText(&mut self) -> ::std::string::String {
        self.parsedStepText.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_parsedStepText<'a>(&'a self) -> &'a str {
        match self.parsedStepText.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool scenarioFailing = 3;

    pub fn clear_scenarioFailing(&mut self) {
        self.scenarioFailing = ::std::option::Option::None;
    }

    pub fn has_scenarioFailing(&self) -> bool {
        self.scenarioFailing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scenarioFailing(&mut self, v: bool) {
        self.scenarioFailing = ::std::option::Option::Some(v);
    }

    pub fn get_scenarioFailing<'a>(&self) -> bool {
        self.scenarioFailing.unwrap_or(false)
    }

    // repeated .gauge.messages.Parameter parameters = 4;

    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_parameters(&mut self, v: ::protobuf::RepeatedField<super::spec::Parameter>) {
        self.parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parameters<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<super::spec::Parameter> {
        &mut self.parameters
    }

    // Take field
    pub fn take_parameters(&mut self) -> ::protobuf::RepeatedField<super::spec::Parameter> {
        ::std::mem::replace(&mut self.parameters, ::protobuf::RepeatedField::new())
    }

    pub fn get_parameters<'a>(&'a self) -> &'a [super::spec::Parameter] {
        &self.parameters
    }
}

impl ::protobuf::Message for ExecuteStepRequest {
    fn is_initialized(&self) -> bool {
        if self.actualStepText.is_none() {
            return false;
        };
        if self.parsedStepText.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.actualStepText));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.parsedStepText));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.scenarioFailing = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.parameters));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.actualStepText.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.parsedStepText.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.scenarioFailing.is_some() {
            my_size += 2;
        };
        for value in self.parameters.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.actualStepText.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.parsedStepText.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.scenarioFailing {
            try!(os.write_bool(3, v));
        };
        for v in self.parameters.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecuteStepRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecuteStepRequest {
    fn new() -> ExecuteStepRequest {
        ExecuteStepRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecuteStepRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "actualStepText",
                    ExecuteStepRequest::has_actualStepText,
                    ExecuteStepRequest::get_actualStepText,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "parsedStepText",
                    ExecuteStepRequest::has_parsedStepText,
                    ExecuteStepRequest::get_parsedStepText,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "scenarioFailing",
                    ExecuteStepRequest::has_scenarioFailing,
                    ExecuteStepRequest::get_scenarioFailing,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "parameters",
                    ExecuteStepRequest::get_parameters,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecuteStepRequest>(
                    "ExecuteStepRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecuteStepRequest {
    fn clear(&mut self) {
        self.clear_actualStepText();
        self.clear_parsedStepText();
        self.clear_scenarioFailing();
        self.clear_parameters();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecuteStepRequest {
    fn eq(&self, other: &ExecuteStepRequest) -> bool {
        self.actualStepText == other.actualStepText &&
        self.parsedStepText == other.parsedStepText &&
        self.scenarioFailing == other.scenarioFailing &&
        self.parameters == other.parameters &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecuteStepRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepValidateRequest {
    // message fields
    stepText: ::protobuf::SingularField<::std::string::String>,
    numberOfParameters: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepValidateRequest {
    pub fn new() -> StepValidateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepValidateRequest {
        static mut instance: ::protobuf::lazy::Lazy<StepValidateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepValidateRequest,
        };
        unsafe {
            instance.get(|| {
                StepValidateRequest {
                    stepText: ::protobuf::SingularField::none(),
                    numberOfParameters: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string stepText = 1;

    pub fn clear_stepText(&mut self) {
        self.stepText.clear();
    }

    pub fn has_stepText(&self) -> bool {
        self.stepText.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepText(&mut self, v: ::std::string::String) {
        self.stepText = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepText<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.stepText.is_none() {
            self.stepText.set_default();
        };
        self.stepText.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepText(&mut self) -> ::std::string::String {
        self.stepText.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_stepText<'a>(&'a self) -> &'a str {
        match self.stepText.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required int32 numberOfParameters = 2;

    pub fn clear_numberOfParameters(&mut self) {
        self.numberOfParameters = ::std::option::Option::None;
    }

    pub fn has_numberOfParameters(&self) -> bool {
        self.numberOfParameters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numberOfParameters(&mut self, v: i32) {
        self.numberOfParameters = ::std::option::Option::Some(v);
    }

    pub fn get_numberOfParameters<'a>(&self) -> i32 {
        self.numberOfParameters.unwrap_or(0)
    }
}

impl ::protobuf::Message for StepValidateRequest {
    fn is_initialized(&self) -> bool {
        if self.stepText.is_none() {
            return false;
        };
        if self.numberOfParameters.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.stepText));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.numberOfParameters = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.stepText.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.numberOfParameters.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stepText.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.numberOfParameters {
            try!(os.write_int32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepValidateRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepValidateRequest {
    fn new() -> StepValidateRequest {
        StepValidateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepValidateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "stepText",
                    StepValidateRequest::has_stepText,
                    StepValidateRequest::get_stepText,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "numberOfParameters",
                    StepValidateRequest::has_numberOfParameters,
                    StepValidateRequest::get_numberOfParameters,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepValidateRequest>(
                    "StepValidateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepValidateRequest {
    fn clear(&mut self) {
        self.clear_stepText();
        self.clear_numberOfParameters();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepValidateRequest {
    fn eq(&self, other: &StepValidateRequest) -> bool {
        self.stepText == other.stepText &&
        self.numberOfParameters == other.numberOfParameters &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepValidateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepValidateResponse {
    // message fields
    isValid: ::std::option::Option<bool>,
    errorMessage: ::protobuf::SingularField<::std::string::String>,
    errorType: ::std::option::Option<StepValidateResponse_ErrorType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepValidateResponse {
    pub fn new() -> StepValidateResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepValidateResponse {
        static mut instance: ::protobuf::lazy::Lazy<StepValidateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepValidateResponse,
        };
        unsafe {
            instance.get(|| {
                StepValidateResponse {
                    isValid: ::std::option::Option::None,
                    errorMessage: ::protobuf::SingularField::none(),
                    errorType: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool isValid = 1;

    pub fn clear_isValid(&mut self) {
        self.isValid = ::std::option::Option::None;
    }

    pub fn has_isValid(&self) -> bool {
        self.isValid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isValid(&mut self, v: bool) {
        self.isValid = ::std::option::Option::Some(v);
    }

    pub fn get_isValid<'a>(&self) -> bool {
        self.isValid.unwrap_or(false)
    }

    // optional string errorMessage = 2;

    pub fn clear_errorMessage(&mut self) {
        self.errorMessage.clear();
    }

    pub fn has_errorMessage(&self) -> bool {
        self.errorMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorMessage(&mut self, v: ::std::string::String) {
        self.errorMessage = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errorMessage<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.errorMessage.is_none() {
            self.errorMessage.set_default();
        };
        self.errorMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_errorMessage(&mut self) -> ::std::string::String {
        self.errorMessage.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_errorMessage<'a>(&'a self) -> &'a str {
        match self.errorMessage.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gauge.messages.StepValidateResponse.ErrorType errorType = 3;

    pub fn clear_errorType(&mut self) {
        self.errorType = ::std::option::Option::None;
    }

    pub fn has_errorType(&self) -> bool {
        self.errorType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorType(&mut self, v: StepValidateResponse_ErrorType) {
        self.errorType = ::std::option::Option::Some(v);
    }

    pub fn get_errorType<'a>(&self) -> StepValidateResponse_ErrorType {
        self.errorType.unwrap_or(StepValidateResponse_ErrorType::STEP_IMPLEMENTATION_NOT_FOUND)
    }
}

impl ::protobuf::Message for StepValidateResponse {
    fn is_initialized(&self) -> bool {
        if self.isValid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.isValid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.errorMessage));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.errorType = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.isValid.is_some() {
            my_size += 2;
        };
        for value in self.errorMessage.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.errorType.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.isValid {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.errorMessage.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.errorType {
            try!(os.write_enum(3, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepValidateResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepValidateResponse {
    fn new() -> StepValidateResponse {
        StepValidateResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepValidateResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isValid",
                    StepValidateResponse::has_isValid,
                    StepValidateResponse::get_isValid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "errorMessage",
                    StepValidateResponse::has_errorMessage,
                    StepValidateResponse::get_errorMessage,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "errorType",
                    StepValidateResponse::has_errorType,
                    StepValidateResponse::get_errorType,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepValidateResponse>(
                    "StepValidateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepValidateResponse {
    fn clear(&mut self) {
        self.clear_isValid();
        self.clear_errorMessage();
        self.clear_errorType();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepValidateResponse {
    fn eq(&self, other: &StepValidateResponse) -> bool {
        self.isValid == other.isValid &&
        self.errorMessage == other.errorMessage &&
        self.errorType == other.errorType &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepValidateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StepValidateResponse_ErrorType {
    STEP_IMPLEMENTATION_NOT_FOUND = 0,
}

impl ::protobuf::ProtobufEnum for StepValidateResponse_ErrorType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StepValidateResponse_ErrorType> {
        match value {
            0 => ::std::option::Option::Some(StepValidateResponse_ErrorType::STEP_IMPLEMENTATION_NOT_FOUND),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StepValidateResponse_ErrorType] = &[
            StepValidateResponse_ErrorType::STEP_IMPLEMENTATION_NOT_FOUND,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<StepValidateResponse_ErrorType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StepValidateResponse_ErrorType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StepValidateResponse_ErrorType {
}

#[derive(Clone,Default)]
pub struct SuiteExecutionResult {
    // message fields
    suiteResult: ::protobuf::SingularPtrField<super::spec::ProtoSuiteResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SuiteExecutionResult {
    pub fn new() -> SuiteExecutionResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SuiteExecutionResult {
        static mut instance: ::protobuf::lazy::Lazy<SuiteExecutionResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SuiteExecutionResult,
        };
        unsafe {
            instance.get(|| {
                SuiteExecutionResult {
                    suiteResult: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.ProtoSuiteResult suiteResult = 1;

    pub fn clear_suiteResult(&mut self) {
        self.suiteResult.clear();
    }

    pub fn has_suiteResult(&self) -> bool {
        self.suiteResult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suiteResult(&mut self, v: super::spec::ProtoSuiteResult) {
        self.suiteResult = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_suiteResult<'a>(&'a mut self) -> &'a mut super::spec::ProtoSuiteResult {
        if self.suiteResult.is_none() {
            self.suiteResult.set_default();
        };
        self.suiteResult.as_mut().unwrap()
    }

    // Take field
    pub fn take_suiteResult(&mut self) -> super::spec::ProtoSuiteResult {
        self.suiteResult.take().unwrap_or_else(|| super::spec::ProtoSuiteResult::new())
    }

    pub fn get_suiteResult<'a>(&'a self) -> &'a super::spec::ProtoSuiteResult {
        self.suiteResult.as_ref().unwrap_or_else(|| super::spec::ProtoSuiteResult::default_instance())
    }
}

impl ::protobuf::Message for SuiteExecutionResult {
    fn is_initialized(&self) -> bool {
        if self.suiteResult.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.suiteResult));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.suiteResult.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suiteResult.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SuiteExecutionResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SuiteExecutionResult {
    fn new() -> SuiteExecutionResult {
        SuiteExecutionResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<SuiteExecutionResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "suiteResult",
                    SuiteExecutionResult::has_suiteResult,
                    SuiteExecutionResult::get_suiteResult,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SuiteExecutionResult>(
                    "SuiteExecutionResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SuiteExecutionResult {
    fn clear(&mut self) {
        self.clear_suiteResult();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SuiteExecutionResult {
    fn eq(&self, other: &SuiteExecutionResult) -> bool {
        self.suiteResult == other.suiteResult &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SuiteExecutionResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepNamesRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepNamesRequest {
    pub fn new() -> StepNamesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepNamesRequest {
        static mut instance: ::protobuf::lazy::Lazy<StepNamesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepNamesRequest,
        };
        unsafe {
            instance.get(|| {
                StepNamesRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for StepNamesRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepNamesRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepNamesRequest {
    fn new() -> StepNamesRequest {
        StepNamesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepNamesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StepNamesRequest>(
                    "StepNamesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepNamesRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepNamesRequest {
    fn eq(&self, other: &StepNamesRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepNamesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepNamesResponse {
    // message fields
    steps: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepNamesResponse {
    pub fn new() -> StepNamesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepNamesResponse {
        static mut instance: ::protobuf::lazy::Lazy<StepNamesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepNamesResponse,
        };
        unsafe {
            instance.get(|| {
                StepNamesResponse {
                    steps: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string steps = 1;

    pub fn clear_steps(&mut self) {
        self.steps.clear();
    }

    // Param is passed by value, moved
    pub fn set_steps(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.steps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_steps<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.steps
    }

    // Take field
    pub fn take_steps(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.steps, ::protobuf::RepeatedField::new())
    }

    pub fn get_steps<'a>(&'a self) -> &'a [::std::string::String] {
        &self.steps
    }
}

impl ::protobuf::Message for StepNamesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.steps));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.steps.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.steps.iter() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepNamesResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepNamesResponse {
    fn new() -> StepNamesResponse {
        StepNamesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepNamesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "steps",
                    StepNamesResponse::get_steps,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepNamesResponse>(
                    "StepNamesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepNamesResponse {
    fn clear(&mut self) {
        self.clear_steps();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepNamesResponse {
    fn eq(&self, other: &StepNamesResponse) -> bool {
        self.steps == other.steps &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepNamesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ScenarioDataStoreInitRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ScenarioDataStoreInitRequest {
    pub fn new() -> ScenarioDataStoreInitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScenarioDataStoreInitRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScenarioDataStoreInitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScenarioDataStoreInitRequest,
        };
        unsafe {
            instance.get(|| {
                ScenarioDataStoreInitRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ScenarioDataStoreInitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ScenarioDataStoreInitRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScenarioDataStoreInitRequest {
    fn new() -> ScenarioDataStoreInitRequest {
        ScenarioDataStoreInitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScenarioDataStoreInitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ScenarioDataStoreInitRequest>(
                    "ScenarioDataStoreInitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScenarioDataStoreInitRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScenarioDataStoreInitRequest {
    fn eq(&self, other: &ScenarioDataStoreInitRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScenarioDataStoreInitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SpecDataStoreInitRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SpecDataStoreInitRequest {
    pub fn new() -> SpecDataStoreInitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpecDataStoreInitRequest {
        static mut instance: ::protobuf::lazy::Lazy<SpecDataStoreInitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpecDataStoreInitRequest,
        };
        unsafe {
            instance.get(|| {
                SpecDataStoreInitRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for SpecDataStoreInitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SpecDataStoreInitRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SpecDataStoreInitRequest {
    fn new() -> SpecDataStoreInitRequest {
        SpecDataStoreInitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpecDataStoreInitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SpecDataStoreInitRequest>(
                    "SpecDataStoreInitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpecDataStoreInitRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SpecDataStoreInitRequest {
    fn eq(&self, other: &SpecDataStoreInitRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SpecDataStoreInitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SuiteDataStoreInitRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SuiteDataStoreInitRequest {
    pub fn new() -> SuiteDataStoreInitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SuiteDataStoreInitRequest {
        static mut instance: ::protobuf::lazy::Lazy<SuiteDataStoreInitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SuiteDataStoreInitRequest,
        };
        unsafe {
            instance.get(|| {
                SuiteDataStoreInitRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for SuiteDataStoreInitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SuiteDataStoreInitRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SuiteDataStoreInitRequest {
    fn new() -> SuiteDataStoreInitRequest {
        SuiteDataStoreInitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SuiteDataStoreInitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SuiteDataStoreInitRequest>(
                    "SuiteDataStoreInitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SuiteDataStoreInitRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SuiteDataStoreInitRequest {
    fn eq(&self, other: &SuiteDataStoreInitRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SuiteDataStoreInitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ParameterPosition {
    // message fields
    oldPosition: ::std::option::Option<i32>,
    newPosition: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ParameterPosition {
    pub fn new() -> ParameterPosition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ParameterPosition {
        static mut instance: ::protobuf::lazy::Lazy<ParameterPosition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ParameterPosition,
        };
        unsafe {
            instance.get(|| {
                ParameterPosition {
                    oldPosition: ::std::option::Option::None,
                    newPosition: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 oldPosition = 1;

    pub fn clear_oldPosition(&mut self) {
        self.oldPosition = ::std::option::Option::None;
    }

    pub fn has_oldPosition(&self) -> bool {
        self.oldPosition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oldPosition(&mut self, v: i32) {
        self.oldPosition = ::std::option::Option::Some(v);
    }

    pub fn get_oldPosition<'a>(&self) -> i32 {
        self.oldPosition.unwrap_or(0)
    }

    // required int32 newPosition = 2;

    pub fn clear_newPosition(&mut self) {
        self.newPosition = ::std::option::Option::None;
    }

    pub fn has_newPosition(&self) -> bool {
        self.newPosition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newPosition(&mut self, v: i32) {
        self.newPosition = ::std::option::Option::Some(v);
    }

    pub fn get_newPosition<'a>(&self) -> i32 {
        self.newPosition.unwrap_or(0)
    }
}

impl ::protobuf::Message for ParameterPosition {
    fn is_initialized(&self) -> bool {
        if self.oldPosition.is_none() {
            return false;
        };
        if self.newPosition.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.oldPosition = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.newPosition = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.oldPosition.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.newPosition.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.oldPosition {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.newPosition {
            try!(os.write_int32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ParameterPosition>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ParameterPosition {
    fn new() -> ParameterPosition {
        ParameterPosition::new()
    }

    fn descriptor_static(_: ::std::option::Option<ParameterPosition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "oldPosition",
                    ParameterPosition::has_oldPosition,
                    ParameterPosition::get_oldPosition,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "newPosition",
                    ParameterPosition::has_newPosition,
                    ParameterPosition::get_newPosition,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ParameterPosition>(
                    "ParameterPosition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ParameterPosition {
    fn clear(&mut self) {
        self.clear_oldPosition();
        self.clear_newPosition();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ParameterPosition {
    fn eq(&self, other: &ParameterPosition) -> bool {
        self.oldPosition == other.oldPosition &&
        self.newPosition == other.newPosition &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ParameterPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RefactorRequest {
    // message fields
    oldStepValue: ::protobuf::SingularPtrField<super::spec::ProtoStepValue>,
    newStepValue: ::protobuf::SingularPtrField<super::spec::ProtoStepValue>,
    paramPositions: ::protobuf::RepeatedField<ParameterPosition>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RefactorRequest {
    pub fn new() -> RefactorRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefactorRequest {
        static mut instance: ::protobuf::lazy::Lazy<RefactorRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefactorRequest,
        };
        unsafe {
            instance.get(|| {
                RefactorRequest {
                    oldStepValue: ::protobuf::SingularPtrField::none(),
                    newStepValue: ::protobuf::SingularPtrField::none(),
                    paramPositions: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.ProtoStepValue oldStepValue = 1;

    pub fn clear_oldStepValue(&mut self) {
        self.oldStepValue.clear();
    }

    pub fn has_oldStepValue(&self) -> bool {
        self.oldStepValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oldStepValue(&mut self, v: super::spec::ProtoStepValue) {
        self.oldStepValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oldStepValue<'a>(&'a mut self) -> &'a mut super::spec::ProtoStepValue {
        if self.oldStepValue.is_none() {
            self.oldStepValue.set_default();
        };
        self.oldStepValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_oldStepValue(&mut self) -> super::spec::ProtoStepValue {
        self.oldStepValue.take().unwrap_or_else(|| super::spec::ProtoStepValue::new())
    }

    pub fn get_oldStepValue<'a>(&'a self) -> &'a super::spec::ProtoStepValue {
        self.oldStepValue.as_ref().unwrap_or_else(|| super::spec::ProtoStepValue::default_instance())
    }

    // required .gauge.messages.ProtoStepValue newStepValue = 2;

    pub fn clear_newStepValue(&mut self) {
        self.newStepValue.clear();
    }

    pub fn has_newStepValue(&self) -> bool {
        self.newStepValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newStepValue(&mut self, v: super::spec::ProtoStepValue) {
        self.newStepValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_newStepValue<'a>(&'a mut self) -> &'a mut super::spec::ProtoStepValue {
        if self.newStepValue.is_none() {
            self.newStepValue.set_default();
        };
        self.newStepValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_newStepValue(&mut self) -> super::spec::ProtoStepValue {
        self.newStepValue.take().unwrap_or_else(|| super::spec::ProtoStepValue::new())
    }

    pub fn get_newStepValue<'a>(&'a self) -> &'a super::spec::ProtoStepValue {
        self.newStepValue.as_ref().unwrap_or_else(|| super::spec::ProtoStepValue::default_instance())
    }

    // repeated .gauge.messages.ParameterPosition paramPositions = 3;

    pub fn clear_paramPositions(&mut self) {
        self.paramPositions.clear();
    }

    // Param is passed by value, moved
    pub fn set_paramPositions(&mut self, v: ::protobuf::RepeatedField<ParameterPosition>) {
        self.paramPositions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_paramPositions<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ParameterPosition> {
        &mut self.paramPositions
    }

    // Take field
    pub fn take_paramPositions(&mut self) -> ::protobuf::RepeatedField<ParameterPosition> {
        ::std::mem::replace(&mut self.paramPositions, ::protobuf::RepeatedField::new())
    }

    pub fn get_paramPositions<'a>(&'a self) -> &'a [ParameterPosition] {
        &self.paramPositions
    }
}

impl ::protobuf::Message for RefactorRequest {
    fn is_initialized(&self) -> bool {
        if self.oldStepValue.is_none() {
            return false;
        };
        if self.newStepValue.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.oldStepValue));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.newStepValue));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.paramPositions));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.oldStepValue.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.newStepValue.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.paramPositions.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.oldStepValue.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.newStepValue.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.paramPositions.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RefactorRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RefactorRequest {
    fn new() -> RefactorRequest {
        RefactorRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefactorRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "oldStepValue",
                    RefactorRequest::has_oldStepValue,
                    RefactorRequest::get_oldStepValue,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "newStepValue",
                    RefactorRequest::has_newStepValue,
                    RefactorRequest::get_newStepValue,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "paramPositions",
                    RefactorRequest::get_paramPositions,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RefactorRequest>(
                    "RefactorRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefactorRequest {
    fn clear(&mut self) {
        self.clear_oldStepValue();
        self.clear_newStepValue();
        self.clear_paramPositions();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RefactorRequest {
    fn eq(&self, other: &RefactorRequest) -> bool {
        self.oldStepValue == other.oldStepValue &&
        self.newStepValue == other.newStepValue &&
        self.paramPositions == other.paramPositions &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RefactorRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RefactorResponse {
    // message fields
    success: ::std::option::Option<bool>,
    error: ::protobuf::SingularField<::std::string::String>,
    filesChanged: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RefactorResponse {
    pub fn new() -> RefactorResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefactorResponse {
        static mut instance: ::protobuf::lazy::Lazy<RefactorResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefactorResponse,
        };
        unsafe {
            instance.get(|| {
                RefactorResponse {
                    success: ::std::option::Option::None,
                    error: ::protobuf::SingularField::none(),
                    filesChanged: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success<'a>(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // optional string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated string filesChanged = 3;

    pub fn clear_filesChanged(&mut self) {
        self.filesChanged.clear();
    }

    // Param is passed by value, moved
    pub fn set_filesChanged(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.filesChanged = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filesChanged<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.filesChanged
    }

    // Take field
    pub fn take_filesChanged(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.filesChanged, ::protobuf::RepeatedField::new())
    }

    pub fn get_filesChanged<'a>(&'a self) -> &'a [::std::string::String] {
        &self.filesChanged
    }
}

impl ::protobuf::Message for RefactorResponse {
    fn is_initialized(&self) -> bool {
        if self.success.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.filesChanged));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.success.is_some() {
            my_size += 2;
        };
        for value in self.error.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.filesChanged.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in self.filesChanged.iter() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RefactorResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RefactorResponse {
    fn new() -> RefactorResponse {
        RefactorResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefactorResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    RefactorResponse::has_success,
                    RefactorResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    RefactorResponse::has_error,
                    RefactorResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "filesChanged",
                    RefactorResponse::get_filesChanged,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RefactorResponse>(
                    "RefactorResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefactorResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_error();
        self.clear_filesChanged();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RefactorResponse {
    fn eq(&self, other: &RefactorResponse) -> bool {
        self.success == other.success &&
        self.error == other.error &&
        self.filesChanged == other.filesChanged &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RefactorResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepNameRequest {
    // message fields
    stepValue: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepNameRequest {
    pub fn new() -> StepNameRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepNameRequest {
        static mut instance: ::protobuf::lazy::Lazy<StepNameRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepNameRequest,
        };
        unsafe {
            instance.get(|| {
                StepNameRequest {
                    stepValue: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string stepValue = 1;

    pub fn clear_stepValue(&mut self) {
        self.stepValue.clear();
    }

    pub fn has_stepValue(&self) -> bool {
        self.stepValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepValue(&mut self, v: ::std::string::String) {
        self.stepValue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepValue<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.stepValue.is_none() {
            self.stepValue.set_default();
        };
        self.stepValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepValue(&mut self) -> ::std::string::String {
        self.stepValue.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_stepValue<'a>(&'a self) -> &'a str {
        match self.stepValue.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for StepNameRequest {
    fn is_initialized(&self) -> bool {
        if self.stepValue.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.stepValue));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.stepValue.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stepValue.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepNameRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepNameRequest {
    fn new() -> StepNameRequest {
        StepNameRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepNameRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "stepValue",
                    StepNameRequest::has_stepValue,
                    StepNameRequest::get_stepValue,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepNameRequest>(
                    "StepNameRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepNameRequest {
    fn clear(&mut self) {
        self.clear_stepValue();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepNameRequest {
    fn eq(&self, other: &StepNameRequest) -> bool {
        self.stepValue == other.stepValue &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepNameRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StepNameResponse {
    // message fields
    isStepPresent: ::std::option::Option<bool>,
    stepName: ::protobuf::RepeatedField<::std::string::String>,
    hasAlias: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StepNameResponse {
    pub fn new() -> StepNameResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepNameResponse {
        static mut instance: ::protobuf::lazy::Lazy<StepNameResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepNameResponse,
        };
        unsafe {
            instance.get(|| {
                StepNameResponse {
                    isStepPresent: ::std::option::Option::None,
                    stepName: ::protobuf::RepeatedField::new(),
                    hasAlias: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool isStepPresent = 1;

    pub fn clear_isStepPresent(&mut self) {
        self.isStepPresent = ::std::option::Option::None;
    }

    pub fn has_isStepPresent(&self) -> bool {
        self.isStepPresent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isStepPresent(&mut self, v: bool) {
        self.isStepPresent = ::std::option::Option::Some(v);
    }

    pub fn get_isStepPresent<'a>(&self) -> bool {
        self.isStepPresent.unwrap_or(false)
    }

    // repeated string stepName = 2;

    pub fn clear_stepName(&mut self) {
        self.stepName.clear();
    }

    // Param is passed by value, moved
    pub fn set_stepName(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.stepName = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stepName<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.stepName
    }

    // Take field
    pub fn take_stepName(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.stepName, ::protobuf::RepeatedField::new())
    }

    pub fn get_stepName<'a>(&'a self) -> &'a [::std::string::String] {
        &self.stepName
    }

    // required bool hasAlias = 3;

    pub fn clear_hasAlias(&mut self) {
        self.hasAlias = ::std::option::Option::None;
    }

    pub fn has_hasAlias(&self) -> bool {
        self.hasAlias.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hasAlias(&mut self, v: bool) {
        self.hasAlias = ::std::option::Option::Some(v);
    }

    pub fn get_hasAlias<'a>(&self) -> bool {
        self.hasAlias.unwrap_or(false)
    }
}

impl ::protobuf::Message for StepNameResponse {
    fn is_initialized(&self) -> bool {
        if self.isStepPresent.is_none() {
            return false;
        };
        if self.hasAlias.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.isStepPresent = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.stepName));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.hasAlias = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.isStepPresent.is_some() {
            my_size += 2;
        };
        for value in self.stepName.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.hasAlias.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.isStepPresent {
            try!(os.write_bool(1, v));
        };
        for v in self.stepName.iter() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.hasAlias {
            try!(os.write_bool(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StepNameResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StepNameResponse {
    fn new() -> StepNameResponse {
        StepNameResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepNameResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isStepPresent",
                    StepNameResponse::has_isStepPresent,
                    StepNameResponse::get_isStepPresent,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "stepName",
                    StepNameResponse::get_stepName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "hasAlias",
                    StepNameResponse::has_hasAlias,
                    StepNameResponse::get_hasAlias,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepNameResponse>(
                    "StepNameResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepNameResponse {
    fn clear(&mut self) {
        self.clear_isStepPresent();
        self.clear_stepName();
        self.clear_hasAlias();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StepNameResponse {
    fn eq(&self, other: &StepNameResponse) -> bool {
        self.isStepPresent == other.isStepPresent &&
        self.stepName == other.stepName &&
        self.hasAlias == other.hasAlias &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StepNameResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UnsupportedMessageResponse {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl UnsupportedMessageResponse {
    pub fn new() -> UnsupportedMessageResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnsupportedMessageResponse {
        static mut instance: ::protobuf::lazy::Lazy<UnsupportedMessageResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnsupportedMessageResponse,
        };
        unsafe {
            instance.get(|| {
                UnsupportedMessageResponse {
                    message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for UnsupportedMessageResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.message.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UnsupportedMessageResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UnsupportedMessageResponse {
    fn new() -> UnsupportedMessageResponse {
        UnsupportedMessageResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnsupportedMessageResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    UnsupportedMessageResponse::has_message,
                    UnsupportedMessageResponse::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnsupportedMessageResponse>(
                    "UnsupportedMessageResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnsupportedMessageResponse {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UnsupportedMessageResponse {
    fn eq(&self, other: &UnsupportedMessageResponse) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UnsupportedMessageResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Message {
    // message fields
    messageType: ::std::option::Option<Message_MessageType>,
    messageId: ::std::option::Option<i64>,
    executionStartingRequest: ::protobuf::SingularPtrField<ExecutionStartingRequest>,
    specExecutionStartingRequest: ::protobuf::SingularPtrField<SpecExecutionStartingRequest>,
    specExecutionEndingRequest: ::protobuf::SingularPtrField<SpecExecutionEndingRequest>,
    scenarioExecutionStartingRequest: ::protobuf::SingularPtrField<ScenarioExecutionStartingRequest>,
    scenarioExecutionEndingRequest: ::protobuf::SingularPtrField<ScenarioExecutionEndingRequest>,
    stepExecutionStartingRequest: ::protobuf::SingularPtrField<StepExecutionStartingRequest>,
    stepExecutionEndingRequest: ::protobuf::SingularPtrField<StepExecutionEndingRequest>,
    executeStepRequest: ::protobuf::SingularPtrField<ExecuteStepRequest>,
    executionEndingRequest: ::protobuf::SingularPtrField<ExecutionEndingRequest>,
    stepValidateRequest: ::protobuf::SingularPtrField<StepValidateRequest>,
    stepValidateResponse: ::protobuf::SingularPtrField<StepValidateResponse>,
    executionStatusResponse: ::protobuf::SingularPtrField<ExecutionStatusResponse>,
    stepNamesRequest: ::protobuf::SingularPtrField<StepNamesRequest>,
    stepNamesResponse: ::protobuf::SingularPtrField<StepNamesResponse>,
    suiteExecutionResult: ::protobuf::SingularPtrField<SuiteExecutionResult>,
    killProcessRequest: ::protobuf::SingularPtrField<KillProcessRequest>,
    scenarioDataStoreInitRequest: ::protobuf::SingularPtrField<ScenarioDataStoreInitRequest>,
    specDataStoreInitRequest: ::protobuf::SingularPtrField<SpecDataStoreInitRequest>,
    suiteDataStoreInitRequest: ::protobuf::SingularPtrField<SuiteDataStoreInitRequest>,
    stepNameRequest: ::protobuf::SingularPtrField<StepNameRequest>,
    stepNameResponse: ::protobuf::SingularPtrField<StepNameResponse>,
    refactorRequest: ::protobuf::SingularPtrField<RefactorRequest>,
    refactorResponse: ::protobuf::SingularPtrField<RefactorResponse>,
    unsupportedMessageResponse: ::protobuf::SingularPtrField<UnsupportedMessageResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(|| {
                Message {
                    messageType: ::std::option::Option::None,
                    messageId: ::std::option::Option::None,
                    executionStartingRequest: ::protobuf::SingularPtrField::none(),
                    specExecutionStartingRequest: ::protobuf::SingularPtrField::none(),
                    specExecutionEndingRequest: ::protobuf::SingularPtrField::none(),
                    scenarioExecutionStartingRequest: ::protobuf::SingularPtrField::none(),
                    scenarioExecutionEndingRequest: ::protobuf::SingularPtrField::none(),
                    stepExecutionStartingRequest: ::protobuf::SingularPtrField::none(),
                    stepExecutionEndingRequest: ::protobuf::SingularPtrField::none(),
                    executeStepRequest: ::protobuf::SingularPtrField::none(),
                    executionEndingRequest: ::protobuf::SingularPtrField::none(),
                    stepValidateRequest: ::protobuf::SingularPtrField::none(),
                    stepValidateResponse: ::protobuf::SingularPtrField::none(),
                    executionStatusResponse: ::protobuf::SingularPtrField::none(),
                    stepNamesRequest: ::protobuf::SingularPtrField::none(),
                    stepNamesResponse: ::protobuf::SingularPtrField::none(),
                    suiteExecutionResult: ::protobuf::SingularPtrField::none(),
                    killProcessRequest: ::protobuf::SingularPtrField::none(),
                    scenarioDataStoreInitRequest: ::protobuf::SingularPtrField::none(),
                    specDataStoreInitRequest: ::protobuf::SingularPtrField::none(),
                    suiteDataStoreInitRequest: ::protobuf::SingularPtrField::none(),
                    stepNameRequest: ::protobuf::SingularPtrField::none(),
                    stepNameResponse: ::protobuf::SingularPtrField::none(),
                    refactorRequest: ::protobuf::SingularPtrField::none(),
                    refactorResponse: ::protobuf::SingularPtrField::none(),
                    unsupportedMessageResponse: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.Message.MessageType messageType = 1;

    pub fn clear_messageType(&mut self) {
        self.messageType = ::std::option::Option::None;
    }

    pub fn has_messageType(&self) -> bool {
        self.messageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_messageType(&mut self, v: Message_MessageType) {
        self.messageType = ::std::option::Option::Some(v);
    }

    pub fn get_messageType<'a>(&self) -> Message_MessageType {
        self.messageType.unwrap_or(Message_MessageType::ExecutionStarting)
    }

    // required int64 messageId = 2;

    pub fn clear_messageId(&mut self) {
        self.messageId = ::std::option::Option::None;
    }

    pub fn has_messageId(&self) -> bool {
        self.messageId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_messageId(&mut self, v: i64) {
        self.messageId = ::std::option::Option::Some(v);
    }

    pub fn get_messageId<'a>(&self) -> i64 {
        self.messageId.unwrap_or(0)
    }

    // optional .gauge.messages.ExecutionStartingRequest executionStartingRequest = 3;

    pub fn clear_executionStartingRequest(&mut self) {
        self.executionStartingRequest.clear();
    }

    pub fn has_executionStartingRequest(&self) -> bool {
        self.executionStartingRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executionStartingRequest(&mut self, v: ExecutionStartingRequest) {
        self.executionStartingRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executionStartingRequest<'a>(&'a mut self) -> &'a mut ExecutionStartingRequest {
        if self.executionStartingRequest.is_none() {
            self.executionStartingRequest.set_default();
        };
        self.executionStartingRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_executionStartingRequest(&mut self) -> ExecutionStartingRequest {
        self.executionStartingRequest.take().unwrap_or_else(|| ExecutionStartingRequest::new())
    }

    pub fn get_executionStartingRequest<'a>(&'a self) -> &'a ExecutionStartingRequest {
        self.executionStartingRequest.as_ref().unwrap_or_else(|| ExecutionStartingRequest::default_instance())
    }

    // optional .gauge.messages.SpecExecutionStartingRequest specExecutionStartingRequest = 4;

    pub fn clear_specExecutionStartingRequest(&mut self) {
        self.specExecutionStartingRequest.clear();
    }

    pub fn has_specExecutionStartingRequest(&self) -> bool {
        self.specExecutionStartingRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_specExecutionStartingRequest(&mut self, v: SpecExecutionStartingRequest) {
        self.specExecutionStartingRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_specExecutionStartingRequest<'a>(&'a mut self) -> &'a mut SpecExecutionStartingRequest {
        if self.specExecutionStartingRequest.is_none() {
            self.specExecutionStartingRequest.set_default();
        };
        self.specExecutionStartingRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_specExecutionStartingRequest(&mut self) -> SpecExecutionStartingRequest {
        self.specExecutionStartingRequest.take().unwrap_or_else(|| SpecExecutionStartingRequest::new())
    }

    pub fn get_specExecutionStartingRequest<'a>(&'a self) -> &'a SpecExecutionStartingRequest {
        self.specExecutionStartingRequest.as_ref().unwrap_or_else(|| SpecExecutionStartingRequest::default_instance())
    }

    // optional .gauge.messages.SpecExecutionEndingRequest specExecutionEndingRequest = 5;

    pub fn clear_specExecutionEndingRequest(&mut self) {
        self.specExecutionEndingRequest.clear();
    }

    pub fn has_specExecutionEndingRequest(&self) -> bool {
        self.specExecutionEndingRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_specExecutionEndingRequest(&mut self, v: SpecExecutionEndingRequest) {
        self.specExecutionEndingRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_specExecutionEndingRequest<'a>(&'a mut self) -> &'a mut SpecExecutionEndingRequest {
        if self.specExecutionEndingRequest.is_none() {
            self.specExecutionEndingRequest.set_default();
        };
        self.specExecutionEndingRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_specExecutionEndingRequest(&mut self) -> SpecExecutionEndingRequest {
        self.specExecutionEndingRequest.take().unwrap_or_else(|| SpecExecutionEndingRequest::new())
    }

    pub fn get_specExecutionEndingRequest<'a>(&'a self) -> &'a SpecExecutionEndingRequest {
        self.specExecutionEndingRequest.as_ref().unwrap_or_else(|| SpecExecutionEndingRequest::default_instance())
    }

    // optional .gauge.messages.ScenarioExecutionStartingRequest scenarioExecutionStartingRequest = 6;

    pub fn clear_scenarioExecutionStartingRequest(&mut self) {
        self.scenarioExecutionStartingRequest.clear();
    }

    pub fn has_scenarioExecutionStartingRequest(&self) -> bool {
        self.scenarioExecutionStartingRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scenarioExecutionStartingRequest(&mut self, v: ScenarioExecutionStartingRequest) {
        self.scenarioExecutionStartingRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scenarioExecutionStartingRequest<'a>(&'a mut self) -> &'a mut ScenarioExecutionStartingRequest {
        if self.scenarioExecutionStartingRequest.is_none() {
            self.scenarioExecutionStartingRequest.set_default();
        };
        self.scenarioExecutionStartingRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_scenarioExecutionStartingRequest(&mut self) -> ScenarioExecutionStartingRequest {
        self.scenarioExecutionStartingRequest.take().unwrap_or_else(|| ScenarioExecutionStartingRequest::new())
    }

    pub fn get_scenarioExecutionStartingRequest<'a>(&'a self) -> &'a ScenarioExecutionStartingRequest {
        self.scenarioExecutionStartingRequest.as_ref().unwrap_or_else(|| ScenarioExecutionStartingRequest::default_instance())
    }

    // optional .gauge.messages.ScenarioExecutionEndingRequest scenarioExecutionEndingRequest = 7;

    pub fn clear_scenarioExecutionEndingRequest(&mut self) {
        self.scenarioExecutionEndingRequest.clear();
    }

    pub fn has_scenarioExecutionEndingRequest(&self) -> bool {
        self.scenarioExecutionEndingRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scenarioExecutionEndingRequest(&mut self, v: ScenarioExecutionEndingRequest) {
        self.scenarioExecutionEndingRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scenarioExecutionEndingRequest<'a>(&'a mut self) -> &'a mut ScenarioExecutionEndingRequest {
        if self.scenarioExecutionEndingRequest.is_none() {
            self.scenarioExecutionEndingRequest.set_default();
        };
        self.scenarioExecutionEndingRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_scenarioExecutionEndingRequest(&mut self) -> ScenarioExecutionEndingRequest {
        self.scenarioExecutionEndingRequest.take().unwrap_or_else(|| ScenarioExecutionEndingRequest::new())
    }

    pub fn get_scenarioExecutionEndingRequest<'a>(&'a self) -> &'a ScenarioExecutionEndingRequest {
        self.scenarioExecutionEndingRequest.as_ref().unwrap_or_else(|| ScenarioExecutionEndingRequest::default_instance())
    }

    // optional .gauge.messages.StepExecutionStartingRequest stepExecutionStartingRequest = 8;

    pub fn clear_stepExecutionStartingRequest(&mut self) {
        self.stepExecutionStartingRequest.clear();
    }

    pub fn has_stepExecutionStartingRequest(&self) -> bool {
        self.stepExecutionStartingRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepExecutionStartingRequest(&mut self, v: StepExecutionStartingRequest) {
        self.stepExecutionStartingRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepExecutionStartingRequest<'a>(&'a mut self) -> &'a mut StepExecutionStartingRequest {
        if self.stepExecutionStartingRequest.is_none() {
            self.stepExecutionStartingRequest.set_default();
        };
        self.stepExecutionStartingRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepExecutionStartingRequest(&mut self) -> StepExecutionStartingRequest {
        self.stepExecutionStartingRequest.take().unwrap_or_else(|| StepExecutionStartingRequest::new())
    }

    pub fn get_stepExecutionStartingRequest<'a>(&'a self) -> &'a StepExecutionStartingRequest {
        self.stepExecutionStartingRequest.as_ref().unwrap_or_else(|| StepExecutionStartingRequest::default_instance())
    }

    // optional .gauge.messages.StepExecutionEndingRequest stepExecutionEndingRequest = 9;

    pub fn clear_stepExecutionEndingRequest(&mut self) {
        self.stepExecutionEndingRequest.clear();
    }

    pub fn has_stepExecutionEndingRequest(&self) -> bool {
        self.stepExecutionEndingRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepExecutionEndingRequest(&mut self, v: StepExecutionEndingRequest) {
        self.stepExecutionEndingRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepExecutionEndingRequest<'a>(&'a mut self) -> &'a mut StepExecutionEndingRequest {
        if self.stepExecutionEndingRequest.is_none() {
            self.stepExecutionEndingRequest.set_default();
        };
        self.stepExecutionEndingRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepExecutionEndingRequest(&mut self) -> StepExecutionEndingRequest {
        self.stepExecutionEndingRequest.take().unwrap_or_else(|| StepExecutionEndingRequest::new())
    }

    pub fn get_stepExecutionEndingRequest<'a>(&'a self) -> &'a StepExecutionEndingRequest {
        self.stepExecutionEndingRequest.as_ref().unwrap_or_else(|| StepExecutionEndingRequest::default_instance())
    }

    // optional .gauge.messages.ExecuteStepRequest executeStepRequest = 10;

    pub fn clear_executeStepRequest(&mut self) {
        self.executeStepRequest.clear();
    }

    pub fn has_executeStepRequest(&self) -> bool {
        self.executeStepRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executeStepRequest(&mut self, v: ExecuteStepRequest) {
        self.executeStepRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executeStepRequest<'a>(&'a mut self) -> &'a mut ExecuteStepRequest {
        if self.executeStepRequest.is_none() {
            self.executeStepRequest.set_default();
        };
        self.executeStepRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_executeStepRequest(&mut self) -> ExecuteStepRequest {
        self.executeStepRequest.take().unwrap_or_else(|| ExecuteStepRequest::new())
    }

    pub fn get_executeStepRequest<'a>(&'a self) -> &'a ExecuteStepRequest {
        self.executeStepRequest.as_ref().unwrap_or_else(|| ExecuteStepRequest::default_instance())
    }

    // optional .gauge.messages.ExecutionEndingRequest executionEndingRequest = 11;

    pub fn clear_executionEndingRequest(&mut self) {
        self.executionEndingRequest.clear();
    }

    pub fn has_executionEndingRequest(&self) -> bool {
        self.executionEndingRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executionEndingRequest(&mut self, v: ExecutionEndingRequest) {
        self.executionEndingRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executionEndingRequest<'a>(&'a mut self) -> &'a mut ExecutionEndingRequest {
        if self.executionEndingRequest.is_none() {
            self.executionEndingRequest.set_default();
        };
        self.executionEndingRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_executionEndingRequest(&mut self) -> ExecutionEndingRequest {
        self.executionEndingRequest.take().unwrap_or_else(|| ExecutionEndingRequest::new())
    }

    pub fn get_executionEndingRequest<'a>(&'a self) -> &'a ExecutionEndingRequest {
        self.executionEndingRequest.as_ref().unwrap_or_else(|| ExecutionEndingRequest::default_instance())
    }

    // optional .gauge.messages.StepValidateRequest stepValidateRequest = 12;

    pub fn clear_stepValidateRequest(&mut self) {
        self.stepValidateRequest.clear();
    }

    pub fn has_stepValidateRequest(&self) -> bool {
        self.stepValidateRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepValidateRequest(&mut self, v: StepValidateRequest) {
        self.stepValidateRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepValidateRequest<'a>(&'a mut self) -> &'a mut StepValidateRequest {
        if self.stepValidateRequest.is_none() {
            self.stepValidateRequest.set_default();
        };
        self.stepValidateRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepValidateRequest(&mut self) -> StepValidateRequest {
        self.stepValidateRequest.take().unwrap_or_else(|| StepValidateRequest::new())
    }

    pub fn get_stepValidateRequest<'a>(&'a self) -> &'a StepValidateRequest {
        self.stepValidateRequest.as_ref().unwrap_or_else(|| StepValidateRequest::default_instance())
    }

    // optional .gauge.messages.StepValidateResponse stepValidateResponse = 13;

    pub fn clear_stepValidateResponse(&mut self) {
        self.stepValidateResponse.clear();
    }

    pub fn has_stepValidateResponse(&self) -> bool {
        self.stepValidateResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepValidateResponse(&mut self, v: StepValidateResponse) {
        self.stepValidateResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepValidateResponse<'a>(&'a mut self) -> &'a mut StepValidateResponse {
        if self.stepValidateResponse.is_none() {
            self.stepValidateResponse.set_default();
        };
        self.stepValidateResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepValidateResponse(&mut self) -> StepValidateResponse {
        self.stepValidateResponse.take().unwrap_or_else(|| StepValidateResponse::new())
    }

    pub fn get_stepValidateResponse<'a>(&'a self) -> &'a StepValidateResponse {
        self.stepValidateResponse.as_ref().unwrap_or_else(|| StepValidateResponse::default_instance())
    }

    // optional .gauge.messages.ExecutionStatusResponse executionStatusResponse = 14;

    pub fn clear_executionStatusResponse(&mut self) {
        self.executionStatusResponse.clear();
    }

    pub fn has_executionStatusResponse(&self) -> bool {
        self.executionStatusResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executionStatusResponse(&mut self, v: ExecutionStatusResponse) {
        self.executionStatusResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executionStatusResponse<'a>(&'a mut self) -> &'a mut ExecutionStatusResponse {
        if self.executionStatusResponse.is_none() {
            self.executionStatusResponse.set_default();
        };
        self.executionStatusResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_executionStatusResponse(&mut self) -> ExecutionStatusResponse {
        self.executionStatusResponse.take().unwrap_or_else(|| ExecutionStatusResponse::new())
    }

    pub fn get_executionStatusResponse<'a>(&'a self) -> &'a ExecutionStatusResponse {
        self.executionStatusResponse.as_ref().unwrap_or_else(|| ExecutionStatusResponse::default_instance())
    }

    // optional .gauge.messages.StepNamesRequest stepNamesRequest = 15;

    pub fn clear_stepNamesRequest(&mut self) {
        self.stepNamesRequest.clear();
    }

    pub fn has_stepNamesRequest(&self) -> bool {
        self.stepNamesRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepNamesRequest(&mut self, v: StepNamesRequest) {
        self.stepNamesRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepNamesRequest<'a>(&'a mut self) -> &'a mut StepNamesRequest {
        if self.stepNamesRequest.is_none() {
            self.stepNamesRequest.set_default();
        };
        self.stepNamesRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepNamesRequest(&mut self) -> StepNamesRequest {
        self.stepNamesRequest.take().unwrap_or_else(|| StepNamesRequest::new())
    }

    pub fn get_stepNamesRequest<'a>(&'a self) -> &'a StepNamesRequest {
        self.stepNamesRequest.as_ref().unwrap_or_else(|| StepNamesRequest::default_instance())
    }

    // optional .gauge.messages.StepNamesResponse stepNamesResponse = 16;

    pub fn clear_stepNamesResponse(&mut self) {
        self.stepNamesResponse.clear();
    }

    pub fn has_stepNamesResponse(&self) -> bool {
        self.stepNamesResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepNamesResponse(&mut self, v: StepNamesResponse) {
        self.stepNamesResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepNamesResponse<'a>(&'a mut self) -> &'a mut StepNamesResponse {
        if self.stepNamesResponse.is_none() {
            self.stepNamesResponse.set_default();
        };
        self.stepNamesResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepNamesResponse(&mut self) -> StepNamesResponse {
        self.stepNamesResponse.take().unwrap_or_else(|| StepNamesResponse::new())
    }

    pub fn get_stepNamesResponse<'a>(&'a self) -> &'a StepNamesResponse {
        self.stepNamesResponse.as_ref().unwrap_or_else(|| StepNamesResponse::default_instance())
    }

    // optional .gauge.messages.SuiteExecutionResult suiteExecutionResult = 17;

    pub fn clear_suiteExecutionResult(&mut self) {
        self.suiteExecutionResult.clear();
    }

    pub fn has_suiteExecutionResult(&self) -> bool {
        self.suiteExecutionResult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suiteExecutionResult(&mut self, v: SuiteExecutionResult) {
        self.suiteExecutionResult = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_suiteExecutionResult<'a>(&'a mut self) -> &'a mut SuiteExecutionResult {
        if self.suiteExecutionResult.is_none() {
            self.suiteExecutionResult.set_default();
        };
        self.suiteExecutionResult.as_mut().unwrap()
    }

    // Take field
    pub fn take_suiteExecutionResult(&mut self) -> SuiteExecutionResult {
        self.suiteExecutionResult.take().unwrap_or_else(|| SuiteExecutionResult::new())
    }

    pub fn get_suiteExecutionResult<'a>(&'a self) -> &'a SuiteExecutionResult {
        self.suiteExecutionResult.as_ref().unwrap_or_else(|| SuiteExecutionResult::default_instance())
    }

    // optional .gauge.messages.KillProcessRequest killProcessRequest = 18;

    pub fn clear_killProcessRequest(&mut self) {
        self.killProcessRequest.clear();
    }

    pub fn has_killProcessRequest(&self) -> bool {
        self.killProcessRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killProcessRequest(&mut self, v: KillProcessRequest) {
        self.killProcessRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_killProcessRequest<'a>(&'a mut self) -> &'a mut KillProcessRequest {
        if self.killProcessRequest.is_none() {
            self.killProcessRequest.set_default();
        };
        self.killProcessRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_killProcessRequest(&mut self) -> KillProcessRequest {
        self.killProcessRequest.take().unwrap_or_else(|| KillProcessRequest::new())
    }

    pub fn get_killProcessRequest<'a>(&'a self) -> &'a KillProcessRequest {
        self.killProcessRequest.as_ref().unwrap_or_else(|| KillProcessRequest::default_instance())
    }

    // optional .gauge.messages.ScenarioDataStoreInitRequest scenarioDataStoreInitRequest = 19;

    pub fn clear_scenarioDataStoreInitRequest(&mut self) {
        self.scenarioDataStoreInitRequest.clear();
    }

    pub fn has_scenarioDataStoreInitRequest(&self) -> bool {
        self.scenarioDataStoreInitRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scenarioDataStoreInitRequest(&mut self, v: ScenarioDataStoreInitRequest) {
        self.scenarioDataStoreInitRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scenarioDataStoreInitRequest<'a>(&'a mut self) -> &'a mut ScenarioDataStoreInitRequest {
        if self.scenarioDataStoreInitRequest.is_none() {
            self.scenarioDataStoreInitRequest.set_default();
        };
        self.scenarioDataStoreInitRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_scenarioDataStoreInitRequest(&mut self) -> ScenarioDataStoreInitRequest {
        self.scenarioDataStoreInitRequest.take().unwrap_or_else(|| ScenarioDataStoreInitRequest::new())
    }

    pub fn get_scenarioDataStoreInitRequest<'a>(&'a self) -> &'a ScenarioDataStoreInitRequest {
        self.scenarioDataStoreInitRequest.as_ref().unwrap_or_else(|| ScenarioDataStoreInitRequest::default_instance())
    }

    // optional .gauge.messages.SpecDataStoreInitRequest specDataStoreInitRequest = 20;

    pub fn clear_specDataStoreInitRequest(&mut self) {
        self.specDataStoreInitRequest.clear();
    }

    pub fn has_specDataStoreInitRequest(&self) -> bool {
        self.specDataStoreInitRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_specDataStoreInitRequest(&mut self, v: SpecDataStoreInitRequest) {
        self.specDataStoreInitRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_specDataStoreInitRequest<'a>(&'a mut self) -> &'a mut SpecDataStoreInitRequest {
        if self.specDataStoreInitRequest.is_none() {
            self.specDataStoreInitRequest.set_default();
        };
        self.specDataStoreInitRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_specDataStoreInitRequest(&mut self) -> SpecDataStoreInitRequest {
        self.specDataStoreInitRequest.take().unwrap_or_else(|| SpecDataStoreInitRequest::new())
    }

    pub fn get_specDataStoreInitRequest<'a>(&'a self) -> &'a SpecDataStoreInitRequest {
        self.specDataStoreInitRequest.as_ref().unwrap_or_else(|| SpecDataStoreInitRequest::default_instance())
    }

    // optional .gauge.messages.SuiteDataStoreInitRequest suiteDataStoreInitRequest = 21;

    pub fn clear_suiteDataStoreInitRequest(&mut self) {
        self.suiteDataStoreInitRequest.clear();
    }

    pub fn has_suiteDataStoreInitRequest(&self) -> bool {
        self.suiteDataStoreInitRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suiteDataStoreInitRequest(&mut self, v: SuiteDataStoreInitRequest) {
        self.suiteDataStoreInitRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_suiteDataStoreInitRequest<'a>(&'a mut self) -> &'a mut SuiteDataStoreInitRequest {
        if self.suiteDataStoreInitRequest.is_none() {
            self.suiteDataStoreInitRequest.set_default();
        };
        self.suiteDataStoreInitRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_suiteDataStoreInitRequest(&mut self) -> SuiteDataStoreInitRequest {
        self.suiteDataStoreInitRequest.take().unwrap_or_else(|| SuiteDataStoreInitRequest::new())
    }

    pub fn get_suiteDataStoreInitRequest<'a>(&'a self) -> &'a SuiteDataStoreInitRequest {
        self.suiteDataStoreInitRequest.as_ref().unwrap_or_else(|| SuiteDataStoreInitRequest::default_instance())
    }

    // optional .gauge.messages.StepNameRequest stepNameRequest = 22;

    pub fn clear_stepNameRequest(&mut self) {
        self.stepNameRequest.clear();
    }

    pub fn has_stepNameRequest(&self) -> bool {
        self.stepNameRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepNameRequest(&mut self, v: StepNameRequest) {
        self.stepNameRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepNameRequest<'a>(&'a mut self) -> &'a mut StepNameRequest {
        if self.stepNameRequest.is_none() {
            self.stepNameRequest.set_default();
        };
        self.stepNameRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepNameRequest(&mut self) -> StepNameRequest {
        self.stepNameRequest.take().unwrap_or_else(|| StepNameRequest::new())
    }

    pub fn get_stepNameRequest<'a>(&'a self) -> &'a StepNameRequest {
        self.stepNameRequest.as_ref().unwrap_or_else(|| StepNameRequest::default_instance())
    }

    // optional .gauge.messages.StepNameResponse stepNameResponse = 23;

    pub fn clear_stepNameResponse(&mut self) {
        self.stepNameResponse.clear();
    }

    pub fn has_stepNameResponse(&self) -> bool {
        self.stepNameResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepNameResponse(&mut self, v: StepNameResponse) {
        self.stepNameResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepNameResponse<'a>(&'a mut self) -> &'a mut StepNameResponse {
        if self.stepNameResponse.is_none() {
            self.stepNameResponse.set_default();
        };
        self.stepNameResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepNameResponse(&mut self) -> StepNameResponse {
        self.stepNameResponse.take().unwrap_or_else(|| StepNameResponse::new())
    }

    pub fn get_stepNameResponse<'a>(&'a self) -> &'a StepNameResponse {
        self.stepNameResponse.as_ref().unwrap_or_else(|| StepNameResponse::default_instance())
    }

    // optional .gauge.messages.RefactorRequest refactorRequest = 24;

    pub fn clear_refactorRequest(&mut self) {
        self.refactorRequest.clear();
    }

    pub fn has_refactorRequest(&self) -> bool {
        self.refactorRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refactorRequest(&mut self, v: RefactorRequest) {
        self.refactorRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_refactorRequest<'a>(&'a mut self) -> &'a mut RefactorRequest {
        if self.refactorRequest.is_none() {
            self.refactorRequest.set_default();
        };
        self.refactorRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_refactorRequest(&mut self) -> RefactorRequest {
        self.refactorRequest.take().unwrap_or_else(|| RefactorRequest::new())
    }

    pub fn get_refactorRequest<'a>(&'a self) -> &'a RefactorRequest {
        self.refactorRequest.as_ref().unwrap_or_else(|| RefactorRequest::default_instance())
    }

    // optional .gauge.messages.RefactorResponse refactorResponse = 25;

    pub fn clear_refactorResponse(&mut self) {
        self.refactorResponse.clear();
    }

    pub fn has_refactorResponse(&self) -> bool {
        self.refactorResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refactorResponse(&mut self, v: RefactorResponse) {
        self.refactorResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_refactorResponse<'a>(&'a mut self) -> &'a mut RefactorResponse {
        if self.refactorResponse.is_none() {
            self.refactorResponse.set_default();
        };
        self.refactorResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_refactorResponse(&mut self) -> RefactorResponse {
        self.refactorResponse.take().unwrap_or_else(|| RefactorResponse::new())
    }

    pub fn get_refactorResponse<'a>(&'a self) -> &'a RefactorResponse {
        self.refactorResponse.as_ref().unwrap_or_else(|| RefactorResponse::default_instance())
    }

    // optional .gauge.messages.UnsupportedMessageResponse unsupportedMessageResponse = 26;

    pub fn clear_unsupportedMessageResponse(&mut self) {
        self.unsupportedMessageResponse.clear();
    }

    pub fn has_unsupportedMessageResponse(&self) -> bool {
        self.unsupportedMessageResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unsupportedMessageResponse(&mut self, v: UnsupportedMessageResponse) {
        self.unsupportedMessageResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unsupportedMessageResponse<'a>(&'a mut self) -> &'a mut UnsupportedMessageResponse {
        if self.unsupportedMessageResponse.is_none() {
            self.unsupportedMessageResponse.set_default();
        };
        self.unsupportedMessageResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_unsupportedMessageResponse(&mut self) -> UnsupportedMessageResponse {
        self.unsupportedMessageResponse.take().unwrap_or_else(|| UnsupportedMessageResponse::new())
    }

    pub fn get_unsupportedMessageResponse<'a>(&'a self) -> &'a UnsupportedMessageResponse {
        self.unsupportedMessageResponse.as_ref().unwrap_or_else(|| UnsupportedMessageResponse::default_instance())
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        if self.messageType.is_none() {
            return false;
        };
        if self.messageId.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.messageType = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.messageId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executionStartingRequest));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.specExecutionStartingRequest));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.specExecutionEndingRequest));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scenarioExecutionStartingRequest));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scenarioExecutionEndingRequest));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepExecutionStartingRequest));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepExecutionEndingRequest));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executeStepRequest));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executionEndingRequest));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepValidateRequest));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepValidateResponse));
                },
                14 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.executionStatusResponse));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepNamesRequest));
                },
                16 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepNamesResponse));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.suiteExecutionResult));
                },
                18 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.killProcessRequest));
                },
                19 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scenarioDataStoreInitRequest));
                },
                20 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.specDataStoreInitRequest));
                },
                21 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.suiteDataStoreInitRequest));
                },
                22 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepNameRequest));
                },
                23 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepNameResponse));
                },
                24 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.refactorRequest));
                },
                25 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.refactorResponse));
                },
                26 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unsupportedMessageResponse));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.messageType.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.messageId.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.executionStartingRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.specExecutionStartingRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.specExecutionEndingRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.scenarioExecutionStartingRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.scenarioExecutionEndingRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stepExecutionStartingRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stepExecutionEndingRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executeStepRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executionEndingRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stepValidateRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stepValidateResponse.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executionStatusResponse.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stepNamesRequest.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stepNamesResponse.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.suiteExecutionResult.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.killProcessRequest.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.scenarioDataStoreInitRequest.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.specDataStoreInitRequest.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.suiteDataStoreInitRequest.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stepNameRequest.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stepNameResponse.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.refactorRequest.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.refactorResponse.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.unsupportedMessageResponse.iter() {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.messageType {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.messageId {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.executionStartingRequest.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.specExecutionStartingRequest.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.specExecutionEndingRequest.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.scenarioExecutionStartingRequest.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.scenarioExecutionEndingRequest.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepExecutionStartingRequest.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepExecutionEndingRequest.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executeStepRequest.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executionEndingRequest.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepValidateRequest.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepValidateResponse.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executionStatusResponse.as_ref() {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepNamesRequest.as_ref() {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepNamesResponse.as_ref() {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.suiteExecutionResult.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.killProcessRequest.as_ref() {
            try!(os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.scenarioDataStoreInitRequest.as_ref() {
            try!(os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.specDataStoreInitRequest.as_ref() {
            try!(os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.suiteDataStoreInitRequest.as_ref() {
            try!(os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepNameRequest.as_ref() {
            try!(os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepNameResponse.as_ref() {
            try!(os.write_tag(23, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.refactorRequest.as_ref() {
            try!(os.write_tag(24, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.refactorResponse.as_ref() {
            try!(os.write_tag(25, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.unsupportedMessageResponse.as_ref() {
            try!(os.write_tag(26, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Message>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "messageType",
                    Message::has_messageType,
                    Message::get_messageType,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "messageId",
                    Message::has_messageId,
                    Message::get_messageId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executionStartingRequest",
                    Message::has_executionStartingRequest,
                    Message::get_executionStartingRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "specExecutionStartingRequest",
                    Message::has_specExecutionStartingRequest,
                    Message::get_specExecutionStartingRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "specExecutionEndingRequest",
                    Message::has_specExecutionEndingRequest,
                    Message::get_specExecutionEndingRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scenarioExecutionStartingRequest",
                    Message::has_scenarioExecutionStartingRequest,
                    Message::get_scenarioExecutionStartingRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scenarioExecutionEndingRequest",
                    Message::has_scenarioExecutionEndingRequest,
                    Message::get_scenarioExecutionEndingRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepExecutionStartingRequest",
                    Message::has_stepExecutionStartingRequest,
                    Message::get_stepExecutionStartingRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepExecutionEndingRequest",
                    Message::has_stepExecutionEndingRequest,
                    Message::get_stepExecutionEndingRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executeStepRequest",
                    Message::has_executeStepRequest,
                    Message::get_executeStepRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executionEndingRequest",
                    Message::has_executionEndingRequest,
                    Message::get_executionEndingRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepValidateRequest",
                    Message::has_stepValidateRequest,
                    Message::get_stepValidateRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepValidateResponse",
                    Message::has_stepValidateResponse,
                    Message::get_stepValidateResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executionStatusResponse",
                    Message::has_executionStatusResponse,
                    Message::get_executionStatusResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepNamesRequest",
                    Message::has_stepNamesRequest,
                    Message::get_stepNamesRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepNamesResponse",
                    Message::has_stepNamesResponse,
                    Message::get_stepNamesResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "suiteExecutionResult",
                    Message::has_suiteExecutionResult,
                    Message::get_suiteExecutionResult,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "killProcessRequest",
                    Message::has_killProcessRequest,
                    Message::get_killProcessRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "scenarioDataStoreInitRequest",
                    Message::has_scenarioDataStoreInitRequest,
                    Message::get_scenarioDataStoreInitRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "specDataStoreInitRequest",
                    Message::has_specDataStoreInitRequest,
                    Message::get_specDataStoreInitRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "suiteDataStoreInitRequest",
                    Message::has_suiteDataStoreInitRequest,
                    Message::get_suiteDataStoreInitRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepNameRequest",
                    Message::has_stepNameRequest,
                    Message::get_stepNameRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepNameResponse",
                    Message::has_stepNameResponse,
                    Message::get_stepNameResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "refactorRequest",
                    Message::has_refactorRequest,
                    Message::get_refactorRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "refactorResponse",
                    Message::has_refactorResponse,
                    Message::get_refactorResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "unsupportedMessageResponse",
                    Message::has_unsupportedMessageResponse,
                    Message::get_unsupportedMessageResponse,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_messageType();
        self.clear_messageId();
        self.clear_executionStartingRequest();
        self.clear_specExecutionStartingRequest();
        self.clear_specExecutionEndingRequest();
        self.clear_scenarioExecutionStartingRequest();
        self.clear_scenarioExecutionEndingRequest();
        self.clear_stepExecutionStartingRequest();
        self.clear_stepExecutionEndingRequest();
        self.clear_executeStepRequest();
        self.clear_executionEndingRequest();
        self.clear_stepValidateRequest();
        self.clear_stepValidateResponse();
        self.clear_executionStatusResponse();
        self.clear_stepNamesRequest();
        self.clear_stepNamesResponse();
        self.clear_suiteExecutionResult();
        self.clear_killProcessRequest();
        self.clear_scenarioDataStoreInitRequest();
        self.clear_specDataStoreInitRequest();
        self.clear_suiteDataStoreInitRequest();
        self.clear_stepNameRequest();
        self.clear_stepNameResponse();
        self.clear_refactorRequest();
        self.clear_refactorResponse();
        self.clear_unsupportedMessageResponse();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        self.messageType == other.messageType &&
        self.messageId == other.messageId &&
        self.executionStartingRequest == other.executionStartingRequest &&
        self.specExecutionStartingRequest == other.specExecutionStartingRequest &&
        self.specExecutionEndingRequest == other.specExecutionEndingRequest &&
        self.scenarioExecutionStartingRequest == other.scenarioExecutionStartingRequest &&
        self.scenarioExecutionEndingRequest == other.scenarioExecutionEndingRequest &&
        self.stepExecutionStartingRequest == other.stepExecutionStartingRequest &&
        self.stepExecutionEndingRequest == other.stepExecutionEndingRequest &&
        self.executeStepRequest == other.executeStepRequest &&
        self.executionEndingRequest == other.executionEndingRequest &&
        self.stepValidateRequest == other.stepValidateRequest &&
        self.stepValidateResponse == other.stepValidateResponse &&
        self.executionStatusResponse == other.executionStatusResponse &&
        self.stepNamesRequest == other.stepNamesRequest &&
        self.stepNamesResponse == other.stepNamesResponse &&
        self.suiteExecutionResult == other.suiteExecutionResult &&
        self.killProcessRequest == other.killProcessRequest &&
        self.scenarioDataStoreInitRequest == other.scenarioDataStoreInitRequest &&
        self.specDataStoreInitRequest == other.specDataStoreInitRequest &&
        self.suiteDataStoreInitRequest == other.suiteDataStoreInitRequest &&
        self.stepNameRequest == other.stepNameRequest &&
        self.stepNameResponse == other.stepNameResponse &&
        self.refactorRequest == other.refactorRequest &&
        self.refactorResponse == other.refactorResponse &&
        self.unsupportedMessageResponse == other.unsupportedMessageResponse &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_MessageType {
    ExecutionStarting = 0,
    SpecExecutionStarting = 1,
    SpecExecutionEnding = 2,
    ScenarioExecutionStarting = 3,
    ScenarioExecutionEnding = 4,
    StepExecutionStarting = 5,
    StepExecutionEnding = 6,
    ExecuteStep = 7,
    ExecutionEnding = 8,
    StepValidateRequest = 9,
    StepValidateResponse = 10,
    ExecutionStatusResponse = 11,
    StepNamesRequest = 12,
    StepNamesResponse = 13,
    KillProcessRequest = 14,
    SuiteExecutionResult = 15,
    ScenarioDataStoreInit = 16,
    SpecDataStoreInit = 17,
    SuiteDataStoreInit = 18,
    StepNameRequest = 19,
    StepNameResponse = 20,
    RefactorRequest = 21,
    RefactorResponse = 22,
    UnsupportedMessageResponse = 23,
}

impl ::protobuf::ProtobufEnum for Message_MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_MessageType> {
        match value {
            0 => ::std::option::Option::Some(Message_MessageType::ExecutionStarting),
            1 => ::std::option::Option::Some(Message_MessageType::SpecExecutionStarting),
            2 => ::std::option::Option::Some(Message_MessageType::SpecExecutionEnding),
            3 => ::std::option::Option::Some(Message_MessageType::ScenarioExecutionStarting),
            4 => ::std::option::Option::Some(Message_MessageType::ScenarioExecutionEnding),
            5 => ::std::option::Option::Some(Message_MessageType::StepExecutionStarting),
            6 => ::std::option::Option::Some(Message_MessageType::StepExecutionEnding),
            7 => ::std::option::Option::Some(Message_MessageType::ExecuteStep),
            8 => ::std::option::Option::Some(Message_MessageType::ExecutionEnding),
            9 => ::std::option::Option::Some(Message_MessageType::StepValidateRequest),
            10 => ::std::option::Option::Some(Message_MessageType::StepValidateResponse),
            11 => ::std::option::Option::Some(Message_MessageType::ExecutionStatusResponse),
            12 => ::std::option::Option::Some(Message_MessageType::StepNamesRequest),
            13 => ::std::option::Option::Some(Message_MessageType::StepNamesResponse),
            14 => ::std::option::Option::Some(Message_MessageType::KillProcessRequest),
            15 => ::std::option::Option::Some(Message_MessageType::SuiteExecutionResult),
            16 => ::std::option::Option::Some(Message_MessageType::ScenarioDataStoreInit),
            17 => ::std::option::Option::Some(Message_MessageType::SpecDataStoreInit),
            18 => ::std::option::Option::Some(Message_MessageType::SuiteDataStoreInit),
            19 => ::std::option::Option::Some(Message_MessageType::StepNameRequest),
            20 => ::std::option::Option::Some(Message_MessageType::StepNameResponse),
            21 => ::std::option::Option::Some(Message_MessageType::RefactorRequest),
            22 => ::std::option::Option::Some(Message_MessageType::RefactorResponse),
            23 => ::std::option::Option::Some(Message_MessageType::UnsupportedMessageResponse),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Message_MessageType] = &[
            Message_MessageType::ExecutionStarting,
            Message_MessageType::SpecExecutionStarting,
            Message_MessageType::SpecExecutionEnding,
            Message_MessageType::ScenarioExecutionStarting,
            Message_MessageType::ScenarioExecutionEnding,
            Message_MessageType::StepExecutionStarting,
            Message_MessageType::StepExecutionEnding,
            Message_MessageType::ExecuteStep,
            Message_MessageType::ExecutionEnding,
            Message_MessageType::StepValidateRequest,
            Message_MessageType::StepValidateResponse,
            Message_MessageType::ExecutionStatusResponse,
            Message_MessageType::StepNamesRequest,
            Message_MessageType::StepNamesResponse,
            Message_MessageType::KillProcessRequest,
            Message_MessageType::SuiteExecutionResult,
            Message_MessageType::ScenarioDataStoreInit,
            Message_MessageType::SpecDataStoreInit,
            Message_MessageType::SuiteDataStoreInit,
            Message_MessageType::StepNameRequest,
            Message_MessageType::StepNameResponse,
            Message_MessageType::RefactorRequest,
            Message_MessageType::RefactorResponse,
            Message_MessageType::UnsupportedMessageResponse,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Message_MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Message_MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_MessageType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x1a, 0x0a, 0x73, 0x70, 0x65, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x14, 0x0a, 0x12,
    0x4b, 0x69, 0x6c, 0x6c, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x22, 0x58, 0x0a, 0x17, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3d, 0x0a,
    0x0f, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x57, 0x0a, 0x18,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e,
    0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3b, 0x0a, 0x14, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f,
    0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x55, 0x0a, 0x16, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x3b, 0x0a, 0x14, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x5b, 0x0a, 0x1c,
    0x53, 0x70, 0x65, 0x63, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61,
    0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3b, 0x0a, 0x14,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x49, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x67, 0x61, 0x75,
    0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x59, 0x0a, 0x1a, 0x53, 0x70, 0x65,
    0x63, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3b, 0x0a, 0x14, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x74, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x49, 0x6e, 0x66, 0x6f, 0x22, 0x5f, 0x0a, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e,
    0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3b, 0x0a, 0x14, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f,
    0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x5d, 0x0a, 0x1e, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69,
    0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3b, 0x0a, 0x14, 0x63, 0x75, 0x72, 0x72, 0x65,
    0x6e, 0x74, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x49, 0x6e, 0x66, 0x6f, 0x22, 0x5b, 0x0a, 0x1c, 0x53, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x3b, 0x0a, 0x14, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66,
    0x6f, 0x22, 0x59, 0x0a, 0x1a, 0x53, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x3b, 0x0a, 0x14, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0xb8, 0x01, 0x0a,
    0x0d, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2d,
    0x0a, 0x0b, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x53, 0x70, 0x65, 0x63, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x70, 0x65, 0x63, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x35, 0x0a,
    0x0f, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2d, 0x0a, 0x0b, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x53,
    0x74, 0x65, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x67, 0x61, 0x75, 0x67,
    0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x49,
    0x6e, 0x66, 0x6f, 0x12, 0x12, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x74, 0x72, 0x61, 0x63,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0x4a, 0x0a, 0x08, 0x53, 0x70, 0x65, 0x63, 0x49,
    0x6e, 0x66, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x69, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x09, 0x22, 0x3c, 0x0a, 0x0c, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x49,
    0x6e, 0x66, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x10, 0x0a, 0x08, 0x69, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x09, 0x22, 0x4e, 0x0a, 0x08, 0x53, 0x74, 0x65, 0x70, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x30, 0x0a,
    0x04, 0x73, 0x74, 0x65, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x65, 0x53, 0x74, 0x65, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x10, 0x0a, 0x08, 0x69, 0x73, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x08, 0x22, 0x8c, 0x01, 0x0a, 0x12, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x53, 0x74, 0x65,
    0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x0e, 0x61, 0x63, 0x74, 0x75,
    0x61, 0x6c, 0x53, 0x74, 0x65, 0x70, 0x54, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x16, 0x0a, 0x0e, 0x70, 0x61, 0x72, 0x73, 0x65, 0x64, 0x53, 0x74, 0x65, 0x70, 0x54, 0x65,
    0x78, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x63, 0x65, 0x6e,
    0x61, 0x72, 0x69, 0x6f, 0x46, 0x61, 0x69, 0x6c, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x2d, 0x0a, 0x0a, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72,
    0x22, 0x43, 0x0a, 0x13, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x74, 0x65, 0x70, 0x54,
    0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x1a, 0x0a, 0x12, 0x6e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x4f, 0x66, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x05, 0x22, 0xb0, 0x01, 0x0a, 0x14, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0f,
    0x0a, 0x07, 0x69, 0x73, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12,
    0x14, 0x0a, 0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x41, 0x0a, 0x09, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x54, 0x79,
    0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2e, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65,
    0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x54, 0x79, 0x70, 0x65, 0x22, 0x2e, 0x0a, 0x09, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x21, 0x0a, 0x1d, 0x53, 0x54, 0x45, 0x50, 0x5f, 0x49, 0x4d,
    0x50, 0x4c, 0x45, 0x4d, 0x45, 0x4e, 0x54, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4e, 0x4f, 0x54,
    0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x00, 0x22, 0x4d, 0x0a, 0x14, 0x53, 0x75, 0x69, 0x74,
    0x65, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x12, 0x35, 0x0a, 0x0b, 0x73, 0x75, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x53, 0x75, 0x69, 0x74,
    0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x12, 0x0a, 0x10, 0x53, 0x74, 0x65, 0x70, 0x4e,
    0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x22, 0x0a, 0x11, 0x53,
    0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x0d, 0x0a, 0x05, 0x73, 0x74, 0x65, 0x70, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x22,
    0x1e, 0x0a, 0x1c, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22,
    0x1a, 0x0a, 0x18, 0x53, 0x70, 0x65, 0x63, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x49, 0x6e, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x1b, 0x0a, 0x19, 0x53,
    0x75, 0x69, 0x74, 0x65, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x3d, 0x0a, 0x11, 0x50, 0x61, 0x72, 0x61,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x13, 0x0a,
    0x0b, 0x6f, 0x6c, 0x64, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x65, 0x77, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x22, 0xb8, 0x01, 0x0a, 0x0f, 0x52, 0x65, 0x66, 0x61,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x34, 0x0a, 0x0c, 0x6f,
    0x6c, 0x64, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x1e, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x12, 0x34, 0x0a, 0x0c, 0x6e, 0x65, 0x77, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x53, 0x74,
    0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x39, 0x0a, 0x0e, 0x70, 0x61, 0x72, 0x61, 0x6d,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x21, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x2e, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x22, 0x48, 0x0a, 0x10, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0d, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x43,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x22, 0x24, 0x0a, 0x0f,
    0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x11, 0x0a, 0x09, 0x73, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x22, 0x4d, 0x0a, 0x10, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x15, 0x0a, 0x0d, 0x69, 0x73, 0x53, 0x74, 0x65, 0x70,
    0x50, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x10, 0x0a,
    0x08, 0x73, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x12,
    0x10, 0x0a, 0x08, 0x68, 0x61, 0x73, 0x41, 0x6c, 0x69, 0x61, 0x73, 0x18, 0x03, 0x20, 0x02, 0x28,
    0x08, 0x22, 0x2d, 0x0a, 0x1a, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x22, 0xfe, 0x12, 0x0a, 0x07, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x38, 0x0a, 0x0b,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0e, 0x32, 0x23, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x49, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x4a, 0x0a, 0x18, 0x65, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x52, 0x0a, 0x1c, 0x73, 0x70, 0x65, 0x63, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x70, 0x65,
    0x63, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x4e, 0x0a, 0x1a, 0x73, 0x70, 0x65,
    0x63, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53,
    0x70, 0x65, 0x63, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x5a, 0x0a, 0x20, 0x73, 0x63, 0x65,
    0x6e, 0x61, 0x72, 0x69, 0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x56, 0x0a, 0x1e, 0x73, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69,
    0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53,
    0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x52, 0x0a,
    0x1c, 0x73, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x4e, 0x0a, 0x1a, 0x73, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x3e, 0x0a, 0x12, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x53, 0x74, 0x65, 0x70,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x53, 0x74, 0x65, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x46, 0x0a, 0x16, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x26, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x40, 0x0a, 0x13, 0x73, 0x74, 0x65,
    0x70, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x42, 0x0a, 0x14, 0x73,
    0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x67, 0x61, 0x75, 0x67,
    0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x56,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x48, 0x0a, 0x17, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x27, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3a, 0x0a, 0x10, 0x73, 0x74, 0x65,
    0x70, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x0f, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3c, 0x0a, 0x11, 0x73, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d,
    0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x21, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x42, 0x0a, 0x14, 0x73, 0x75, 0x69, 0x74, 0x65, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x11, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x24, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x53, 0x75, 0x69, 0x74, 0x65, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x3e, 0x0a, 0x12, 0x6b, 0x69, 0x6c, 0x6c, 0x50,
    0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x12, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x4b, 0x69, 0x6c, 0x6c, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x52, 0x0a, 0x1c, 0x73, 0x63, 0x65, 0x6e, 0x61,
    0x72, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53,
    0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x49, 0x6e, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x4a, 0x0a, 0x18, 0x73,
    0x70, 0x65, 0x63, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53,
    0x70, 0x65, 0x63, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x4c, 0x0a, 0x19, 0x73, 0x75, 0x69, 0x74, 0x65,
    0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x67, 0x61, 0x75,
    0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x75, 0x69, 0x74,
    0x65, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x38, 0x0a, 0x0f, 0x73, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x3a, 0x0a, 0x10, 0x73, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x18, 0x17, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x67, 0x61, 0x75, 0x67,
    0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x4e,
    0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x38, 0x0a, 0x0f, 0x72,
    0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x18,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3a, 0x0a, 0x10, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f,
    0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x19, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x20, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x2e, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x4e, 0x0a, 0x1a, 0x75, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18,
    0x1a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74,
    0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0xdd, 0x04, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x15, 0x0a, 0x11, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x15, 0x53, 0x70, 0x65, 0x63,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e,
    0x67, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x53, 0x70, 0x65, 0x63, 0x45, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x10, 0x02, 0x12, 0x1d, 0x0a, 0x19,
    0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f,
    0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x10, 0x03, 0x12, 0x1b, 0x0a, 0x17, 0x53,
    0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x10, 0x04, 0x12, 0x19, 0x0a, 0x15, 0x53, 0x74, 0x65, 0x70,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e,
    0x67, 0x10, 0x05, 0x12, 0x17, 0x0a, 0x13, 0x53, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x10, 0x06, 0x12, 0x0f, 0x0a, 0x0b,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x53, 0x74, 0x65, 0x70, 0x10, 0x07, 0x12, 0x13, 0x0a,
    0x0f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x10, 0x08, 0x12, 0x17, 0x0a, 0x13, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x09, 0x12, 0x18, 0x0a, 0x14, 0x53,
    0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x10, 0x0a, 0x12, 0x1b, 0x0a, 0x17, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x10, 0x0b, 0x12, 0x14, 0x0a, 0x10, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x0c, 0x12, 0x15, 0x0a, 0x11, 0x53, 0x74, 0x65, 0x70,
    0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x0d, 0x12,
    0x16, 0x0a, 0x12, 0x4b, 0x69, 0x6c, 0x6c, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x0e, 0x12, 0x18, 0x0a, 0x14, 0x53, 0x75, 0x69, 0x74, 0x65,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x10,
    0x0f, 0x12, 0x19, 0x0a, 0x15, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x44, 0x61, 0x74,
    0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x10, 0x10, 0x12, 0x15, 0x0a, 0x11,
    0x53, 0x70, 0x65, 0x63, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69,
    0x74, 0x10, 0x11, 0x12, 0x16, 0x0a, 0x12, 0x53, 0x75, 0x69, 0x74, 0x65, 0x44, 0x61, 0x74, 0x61,
    0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x10, 0x12, 0x12, 0x13, 0x0a, 0x0f, 0x53,
    0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x13,
    0x12, 0x14, 0x0a, 0x10, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x10, 0x14, 0x12, 0x13, 0x0a, 0x0f, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x15, 0x12, 0x14, 0x0a, 0x10, 0x52,
    0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10,
    0x16, 0x12, 0x1e, 0x0a, 0x1a, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10,
    0x17, 0x4a, 0xf1, 0x69, 0x0a, 0x07, 0x12, 0x05, 0x13, 0x00, 0xbe, 0x02, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x14, 0x08, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x16,
    0x07, 0x13, 0x0a, 0x3d, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x19, 0x00, 0x1a, 0x01, 0x1a, 0x31,
    0x2f, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x2e, 0x20, 0x54, 0x65, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e,
    0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x1a, 0x0a, 0x83, 0x01,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1e, 0x00, 0x20, 0x01, 0x1a, 0x77, 0x2f, 0x20, 0x53, 0x65,
    0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20, 0x61,
    0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x20, 0x61, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x2f, 0x20,
    0x75, 0x73, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x74, 0x65, 0x70, 0x20, 0x65, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x73, 0x20, 0x65, 0x74,
    0x63, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x02, 0x43, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x1f, 0x0b, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1f, 0x2f, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1f, 0x41, 0x42, 0x0a, 0x61, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01,
    0x1a, 0x55, 0x2f, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x74, 0x20, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x75, 0x69, 0x74, 0x65, 0x20, 0x45, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x65, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x65, 0x20, 0x60, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x5f, 0x73, 0x75, 0x69, 0x74, 0x65, 0x60,
    0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x23, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x04, 0x34,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x24, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x32, 0x33, 0x0a, 0x5e, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x28, 0x00, 0x2a, 0x01, 0x1a, 0x52, 0x2f, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x74, 0x20,
    0x65, 0x6e, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x75, 0x69, 0x74, 0x65, 0x20, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x65, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x65, 0x20, 0x60, 0x61, 0x66, 0x74, 0x65, 0x72, 0x5f, 0x73, 0x75, 0x69, 0x74, 0x65,
    0x60, 0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x28, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x29, 0x04,
    0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x32, 0x33, 0x0a, 0x5f, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x2d, 0x00, 0x2f, 0x01, 0x1a, 0x53, 0x2f, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x74,
    0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x65, 0x6c, 0x6c, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x65, 0x20, 0x60, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x5f, 0x73, 0x70,
    0x65, 0x63, 0x60, 0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x2d, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x2e, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2e, 0x0d, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x32, 0x33, 0x0a, 0x5c, 0x0a, 0x02, 0x04,
    0x05, 0x12, 0x04, 0x32, 0x00, 0x34, 0x01, 0x1a, 0x50, 0x2f, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20,
    0x61, 0x74, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x65, 0x6c, 0x6c, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x65, 0x20, 0x60, 0x61, 0x66, 0x74, 0x65, 0x72, 0x5f, 0x73, 0x70, 0x65,
    0x63, 0x60, 0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x32, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x33,
    0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x33, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x33, 0x0d, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33, 0x32, 0x33, 0x0a, 0x67, 0x0a, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x37, 0x00, 0x39, 0x01, 0x1a, 0x5b, 0x2f, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x61,
    0x74, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61,
    0x72, 0x69, 0x6f, 0x20, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54,
    0x65, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20,
    0x74, 0x6f, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x20, 0x60, 0x62, 0x65, 0x66, 0x6f,
    0x72, 0x65, 0x5f, 0x73, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x60, 0x20, 0x68, 0x6f, 0x6f,
    0x6b, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x37, 0x08, 0x28, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x38, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x38, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x38, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x38, 0x32, 0x33, 0x0a, 0x64, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3c, 0x00, 0x3e, 0x01,
    0x1a, 0x58, 0x2f, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x74, 0x20, 0x65, 0x6e, 0x64, 0x20,
    0x6f, 0x66, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x20, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x65, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x65, 0x20, 0x60, 0x61, 0x66, 0x74, 0x65, 0x72, 0x5f, 0x73, 0x63, 0x65, 0x6e, 0x61, 0x72,
    0x69, 0x6f, 0x60, 0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07,
    0x01, 0x12, 0x03, 0x3c, 0x08, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03,
    0x3d, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3d, 0x0d, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x1b, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x32, 0x33, 0x0a, 0x5f, 0x0a, 0x02, 0x04,
    0x08, 0x12, 0x04, 0x41, 0x00, 0x43, 0x01, 0x1a, 0x53, 0x2f, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20,
    0x61, 0x74, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x74, 0x65, 0x70,
    0x20, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x65, 0x6c, 0x6c,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20,
    0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x20, 0x60, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x5f,
    0x73, 0x74, 0x65, 0x70, 0x60, 0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x08, 0x01, 0x12, 0x03, 0x41, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00,
    0x12, 0x03, 0x42, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x42, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x42, 0x0d,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x1b, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x32, 0x33, 0x0a, 0x5c, 0x0a,
    0x02, 0x04, 0x09, 0x12, 0x04, 0x46, 0x00, 0x48, 0x01, 0x1a, 0x50, 0x2f, 0x20, 0x53, 0x65, 0x6e,
    0x74, 0x20, 0x61, 0x74, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x74, 0x65, 0x70,
    0x20, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x65, 0x6c, 0x6c,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20,
    0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x20, 0x60, 0x61, 0x66, 0x74, 0x65, 0x72, 0x5f, 0x73,
    0x74, 0x65, 0x70, 0x60, 0x20, 0x68, 0x6f, 0x6f, 0x6b, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x03, 0x46, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12,
    0x03, 0x47, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x47, 0x0d, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x1b, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x32, 0x33, 0x0a, 0x8e, 0x01, 0x0a,
    0x02, 0x04, 0x0a, 0x12, 0x04, 0x4c, 0x00, 0x55, 0x01, 0x1a, 0x81, 0x01, 0x2f, 0x20, 0x43, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x20, 0x0a, 0x2f, 0x20, 0x44, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x20, 0x28, 0x53, 0x74,
    0x65, 0x70, 0x2c, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x2c, 0x20, 0x53, 0x70,
    0x65, 0x63, 0x20, 0x6f, 0x72, 0x20, 0x53, 0x75, 0x69, 0x74, 0x65, 0x29, 0x2c, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x15, 0x0a, 0x5e, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x00, 0x12, 0x03, 0x4e, 0x04, 0x26, 0x1a, 0x51, 0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53,
    0x70, 0x65, 0x63, 0x2e, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x78, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x4e, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x4e, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x24,
    0x25, 0x0a, 0x66, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x50, 0x04, 0x2e, 0x1a, 0x59,
    0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x2e,
    0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x20, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x50, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x50, 0x1a, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x50, 0x2c,
    0x2d, 0x0a, 0x5e, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x52, 0x04, 0x26, 0x1a, 0x51,
    0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x74, 0x65, 0x70, 0x2e, 0x20, 0x56, 0x61, 0x6c,
    0x69, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x06, 0x12, 0x03, 0x52, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03, 0x52, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x02, 0x03, 0x12, 0x03, 0x52, 0x24, 0x25, 0x0a, 0x5a, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x03, 0x12, 0x03, 0x54, 0x04, 0x23, 0x1a, 0x4d, 0x2f, 0x20, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x74,
    0x72, 0x61, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x20, 0x6f, 0x6e, 0x6c,
    0x79, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x54, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x54, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x54, 0x14, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x54, 0x21, 0x22, 0x0a, 0x36, 0x0a,
    0x02, 0x04, 0x0b, 0x12, 0x04, 0x58, 0x00, 0x61, 0x01, 0x1a, 0x2a, 0x2f, 0x20, 0x43, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x58, 0x08,
    0x10, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x04, 0x1d, 0x1a, 0x2b,
    0x2f, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67,
    0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x5a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x5a, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x5a, 0x1b, 0x1c, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x04, 0x21,
    0x1a, 0x3d, 0x2f, 0x20, 0x46, 0x75, 0x6c, 0x6c, 0x20, 0x46, 0x69, 0x6c, 0x65, 0x20, 0x70, 0x61,
    0x74, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x62,
    0x65, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5c, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x5c, 0x1f, 0x20, 0x0a, 0x46, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12,
    0x03, 0x5e, 0x04, 0x1f, 0x1a, 0x39, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x74, 0x6f, 0x20,
    0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x65, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5e, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x5e, 0x1d, 0x1e, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12,
    0x03, 0x60, 0x04, 0x1e, 0x1a, 0x2f, 0x2f, 0x20, 0x54, 0x61, 0x67, 0x73, 0x20, 0x72, 0x65, 0x6c,
    0x65, 0x76, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x60, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x03, 0x60, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x03, 0x60, 0x15, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x03, 0x60, 0x1c, 0x1d, 0x0a, 0x3a, 0x0a,
    0x02, 0x04, 0x0c, 0x12, 0x04, 0x64, 0x00, 0x6b, 0x01, 0x1a, 0x2e, 0x2f, 0x20, 0x43, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x20, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01,
    0x12, 0x03, 0x64, 0x08, 0x14, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x66,
    0x04, 0x1d, 0x1a, 0x2f, 0x2f, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72,
    0x69, 0x6f, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x66, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x66, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x14, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x66, 0x1b, 0x1c, 0x0a, 0x4a, 0x0a, 0x04, 0x04,
    0x0c, 0x02, 0x01, 0x12, 0x03, 0x68, 0x04, 0x1f, 0x1a, 0x3d, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67,
    0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x69, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x63, 0x65, 0x6e,
    0x61, 0x72, 0x69, 0x6f, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66,
    0x61, 0x69, 0x6c, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x68, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x68, 0x12,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x68, 0x1d, 0x1e, 0x0a,
    0x40, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x6a, 0x04, 0x1e, 0x1a, 0x33, 0x2f, 0x20,
    0x54, 0x61, 0x67, 0x73, 0x20, 0x72, 0x65, 0x6c, 0x65, 0x76, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x63, 0x65,
    0x6e, 0x61, 0x72, 0x69, 0x6f, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6a, 0x1c, 0x1d, 0x0a, 0x36, 0x0a, 0x02, 0x04, 0x0d, 0x12,
    0x04, 0x6e, 0x00, 0x73, 0x01, 0x1a, 0x2a, 0x2f, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x73, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x33, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x70, 0x04, 0x29, 0x1a, 0x26, 0x2f, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x20, 0x53, 0x74, 0x65,
    0x70, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x03, 0x70, 0x0d, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x20, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x27, 0x28, 0x0a, 0x46, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x01, 0x12, 0x03, 0x72, 0x04, 0x1f, 0x1a, 0x39, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20,
    0x74, 0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20,
    0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x03, 0x72, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x03, 0x72, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x72, 0x1d, 0x1e, 0x0a, 0x3c, 0x0a, 0x02, 0x04, 0x0e,
    0x12, 0x05, 0x76, 0x00, 0x81, 0x01, 0x01, 0x1a, 0x2f, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65,
    0x20, 0x61, 0x20, 0x53, 0x74, 0x65, 0x70, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12,
    0x03, 0x76, 0x08, 0x1a, 0x0a, 0x7c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x79, 0x02,
    0x25, 0x1a, 0x6f, 0x2f, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20,
    0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x64, 0x2e, 0x20, 0x0a, 0x2f, 0x20, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70,
    0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x61, 0x73, 0x20, 0x64, 0x65, 0x66,
    0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x70, 0x65, 0x63,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x79, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x79, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x79, 0x23, 0x24, 0x0a, 0x75, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x01, 0x12, 0x03, 0x7c, 0x02, 0x25, 0x1a, 0x68, 0x2f, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x65, 0x78, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20,
    0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x64, 0x2e, 0x20,
    0x0a, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x74, 0x65, 0x72, 0x73,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x68, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x73, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7c, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7c, 0x23, 0x24, 0x0a, 0x67, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x02, 0x12, 0x03, 0x7e, 0x02, 0x24, 0x1a, 0x5a, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x74,
    0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x2c, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x20, 0x53, 0x74, 0x65, 0x70, 0x2c, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7e, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7e, 0x10, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7e, 0x22, 0x23, 0x0a, 0x49, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x03, 0x12, 0x04, 0x80, 0x01, 0x02, 0x33, 0x1a, 0x3b, 0x2f, 0x20, 0x43, 0x6f, 0x6c, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65,
    0x74, 0x65, 0x72, 0x73, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x53,
    0x74, 0x65, 0x70, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x80, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x06, 0x12, 0x04, 0x80,
    0x01, 0x0b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x01, 0x12, 0x04, 0x80, 0x01,
    0x24, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03, 0x12, 0x04, 0x80, 0x01, 0x31,
    0x32, 0x0a, 0xa7, 0x01, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x85, 0x01, 0x00, 0x8a, 0x01, 0x01,
    0x1a, 0x98, 0x01, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x73, 0x65, 0x6e,
    0x74, 0x20, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20,
    0x74, 0x6f, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x69, 0x66, 0x20, 0x67, 0x69, 0x76, 0x65,
    0x6e, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x2e,
    0x20, 0x0a, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x69, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65,
    0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20,
    0x53, 0x74, 0x65, 0x70, 0x20, 0x54, 0x65, 0x78, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x0f, 0x01, 0x12, 0x04, 0x85, 0x01, 0x08, 0x1b, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00,
    0x12, 0x04, 0x87, 0x01, 0x02, 0x1f, 0x1a, 0x31, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x65,
    0x78, 0x74, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x6f,
    0x6f, 0x6b, 0x75, 0x70, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x87, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x87, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x87, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x87, 0x01, 0x1d, 0x1e, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0x89,
    0x01, 0x02, 0x28, 0x1a, 0x26, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65,
    0x72, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x74, 0x65, 0x72, 0x73, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x74, 0x65, 0x70, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0x89, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x89, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x89, 0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x89, 0x01, 0x26, 0x27, 0x0a, 0xd7, 0x01, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06,
    0x90, 0x01, 0x00, 0x97, 0x01, 0x01, 0x1a, 0xc8, 0x01, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x2f, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x65, 0x6c, 0x6c, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x65, 0x72, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x77, 0x61, 0x73, 0x20, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x2c, 0x20, 0x0a, 0x2f, 0x20, 0x69, 0x2e, 0x65, 0x2e, 0x20, 0x61, 0x6e, 0x20, 0x69,
    0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78,
    0x69, 0x73, 0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x53,
    0x74, 0x65, 0x70, 0x20, 0x74, 0x65, 0x78, 0x74, 0x2e, 0x0a, 0x2f, 0x20, 0x52, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x90, 0x01, 0x08, 0x1c, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x10, 0x04, 0x00, 0x12, 0x06, 0x91, 0x01, 0x04, 0x93, 0x01, 0x05, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x04, 0x00, 0x01, 0x12, 0x04, 0x91, 0x01, 0x09, 0x12, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x10, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x92, 0x01, 0x08, 0x2a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x10, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x92, 0x01, 0x08, 0x25, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x10, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x92, 0x01, 0x28, 0x29, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x94, 0x01, 0x04, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0x94, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02,
    0x01, 0x12, 0x04, 0x95, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x95, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x95, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x95, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0x95,
    0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0x96, 0x01, 0x04,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x06, 0x12, 0x04, 0x96, 0x01, 0x0d, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x17, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0x96, 0x01, 0x23, 0x24, 0x0a, 0x2f, 0x0a,
    0x02, 0x04, 0x11, 0x12, 0x06, 0x9a, 0x01, 0x00, 0x9c, 0x01, 0x01, 0x1a, 0x21, 0x2f, 0x20, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x75, 0x69,
    0x74, 0x65, 0x20, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x00, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x9b, 0x01, 0x0d, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x9b, 0x01, 0x2d, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x9b, 0x01, 0x3b, 0x3c, 0x0a, 0x37, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0x9f, 0x01, 0x00,
    0xa0, 0x01, 0x01, 0x1a, 0x29, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20,
    0x47, 0x61, 0x75, 0x67, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x69, 0x76, 0x65, 0x20, 0x61, 0x6c,
    0x6c, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x18, 0x0a, 0x2d, 0x0a, 0x02, 0x04,
    0x13, 0x12, 0x06, 0xa4, 0x01, 0x00, 0xa7, 0x01, 0x01, 0x1a, 0x1f, 0x2f, 0x20, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d,
    0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13,
    0x01, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x19, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12,
    0x04, 0xa6, 0x01, 0x04, 0x1e, 0x1a, 0x35, 0x2f, 0x20, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x73, 0x20, 0x63,
    0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20,
    0x53, 0x74, 0x65, 0x70, 0x20, 0x74, 0x65, 0x78, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x1c, 0x1d, 0x0a, 0x7e, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06,
    0xab, 0x01, 0x00, 0xac, 0x01, 0x01, 0x1a, 0x70, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x69, 0x74,
    0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x20,
    0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x0a, 0x2f, 0x20, 0x53, 0x63, 0x65, 0x6e,
    0x61, 0x72, 0x69, 0x6f, 0x20, 0x44, 0x61, 0x74, 0x61, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x72, 0x65, 0x73, 0x65, 0x74, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x65, 0x76,
    0x65, 0x72, 0x79, 0x20, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x20, 0x65, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12,
    0x04, 0xab, 0x01, 0x08, 0x24, 0x0a, 0x72, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xb0, 0x01, 0x00,
    0xb1, 0x01, 0x01, 0x1a, 0x64, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x72,
    0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x0a, 0x2f, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x44, 0x61, 0x74, 0x61, 0x73, 0x74,
    0x6f, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x65, 0x74, 0x20, 0x61, 0x66, 0x74,
    0x65, 0x72, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x53, 0x70, 0x65, 0x63, 0x20, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01,
    0x12, 0x04, 0xb0, 0x01, 0x08, 0x20, 0x0a, 0x75, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xb5, 0x01,
    0x00, 0xb6, 0x01, 0x01, 0x1a, 0x67, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20,
    0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x20, 0x53, 0x75, 0x69, 0x74, 0x65, 0x20, 0x44, 0x61, 0x74, 0x61, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x0a, 0x2f, 0x20, 0x53, 0x75, 0x69, 0x74, 0x65, 0x20, 0x44, 0x61, 0x74,
    0x61, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73, 0x65, 0x74, 0x20,
    0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x53, 0x75, 0x69, 0x74,
    0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x21, 0x0a, 0x60, 0x0a, 0x02, 0x04, 0x17,
    0x12, 0x06, 0xba, 0x01, 0x00, 0xbd, 0x01, 0x01, 0x1a, 0x52, 0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6f, 0x6c,
    0x64, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x2f, 0x20, 0x55, 0x73,
    0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72,
    0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x53, 0x74, 0x65, 0x70, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x17, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x00, 0x12, 0x04, 0xbb, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xbb, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xbb, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xbb, 0x01, 0x11, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb,
    0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x02,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x05, 0x12, 0x04, 0xbc, 0x01, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x11, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x1f, 0x20, 0x0a, 0x41, 0x0a,
    0x02, 0x04, 0x18, 0x12, 0x06, 0xc0, 0x01, 0x00, 0xc7, 0x01, 0x01, 0x1a, 0x33, 0x2f, 0x20, 0x54,
    0x65, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72, 0x20,
    0x74, 0x6f, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x53, 0x74, 0x65, 0x70, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x17, 0x0a, 0x3b, 0x0a,
    0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xc2, 0x01, 0x02, 0x3a, 0x1a, 0x2d, 0x2f, 0x20, 0x4f,
    0x6c, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2c, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x74, 0x6f,
    0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xc2, 0x01, 0x0b, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xc2, 0x01, 0x29, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xc2, 0x01, 0x38, 0x39, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04,
    0xc4, 0x01, 0x02, 0x3a, 0x1a, 0x37, 0x2f, 0x20, 0x4e, 0x65, 0x77, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x2d, 0x62, 0x65, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67,
    0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc4, 0x01, 0x0b, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x29, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x38, 0x39, 0x0a, 0x67, 0x0a, 0x04, 0x04, 0x18, 0x02,
    0x02, 0x12, 0x04, 0xc6, 0x01, 0x02, 0x30, 0x1a, 0x59, 0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73,
    0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x70, 0x61, 0x72, 0x61,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73,
    0x20, 0x6f, 0x6c, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x70, 0x61, 0x72,
    0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x06, 0x12, 0x04, 0xc6, 0x01, 0x0b, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x1d, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x2e, 0x2f, 0x0a, 0x2e,
    0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xca, 0x01, 0x00, 0xd1, 0x01, 0x01, 0x1a, 0x20, 0x2f, 0x20,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x52, 0x65,
    0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xca, 0x01, 0x08, 0x18, 0x0a, 0x43, 0x0a, 0x04, 0x04,
    0x19, 0x02, 0x00, 0x12, 0x04, 0xcc, 0x01, 0x02, 0x1c, 0x1a, 0x35, 0x2f, 0x20, 0x46, 0x6c, 0x61,
    0x67, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x52, 0x65, 0x66, 0x61,
    0x63, 0x74, 0x6f, 0x72, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcc, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x0b, 0x0f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x10, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x1a, 0x1b, 0x0a, 0x48, 0x0a, 0x04,
    0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xce, 0x01, 0x02, 0x1c, 0x1a, 0x3a, 0x2f, 0x20, 0x45, 0x72,
    0x72, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2c, 0x20, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x69, 0x66, 0x20, 0x52, 0x65, 0x66, 0x61, 0x63,
    0x74, 0x6f, 0x72, 0x20, 0x77, 0x61, 0x73, 0x6e, 0x27, 0x74, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65,
    0x73, 0x73, 0x66, 0x75, 0x6c, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xce, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xce, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xce,
    0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xce, 0x01,
    0x1a, 0x1b, 0x0a, 0x4d, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x23,
    0x1a, 0x3f, 0x2f, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x65, 0x72, 0x65, 0x20, 0x61, 0x66, 0x66, 0x65,
    0x63, 0x74, 0x65, 0x64, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd0, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x12, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x21, 0x22, 0x0a, 0x36, 0x0a,
    0x02, 0x04, 0x1a, 0x12, 0x06, 0xd4, 0x01, 0x00, 0xd8, 0x01, 0x01, 0x1a, 0x28, 0x2f, 0x20, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69,
    0x6c, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x53, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x53,
    0x74, 0x65, 0x70, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xd4, 0x01,
    0x08, 0x17, 0x0a, 0x77, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x20,
    0x1a, 0x69, 0x2f, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x74, 0x65, 0x70,
    0x2e, 0x20, 0x0a, 0x2f, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x64, 0x20, 0x73, 0x74, 0x65, 0x70, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x2c, 0x20, 0x69, 0x2e, 0x65, 0x2e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x70, 0x6c,
    0x61, 0x63, 0x65, 0x68, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x70,
    0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xd7, 0x01, 0x1e, 0x1f, 0x0a, 0x2d, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xdb,
    0x01, 0x00, 0xe2, 0x01, 0x01, 0x1a, 0x1f, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xdb,
    0x01, 0x08, 0x18, 0x0a, 0x4d, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xdd, 0x01, 0x02,
    0x22, 0x1a, 0x3f, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x54, 0x65, 0x78, 0x74,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdd, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdd, 0x01, 0x0b, 0x0f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x10, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x20, 0x21, 0x0a, 0x32,
    0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x02, 0x1f, 0x1a, 0x24, 0x2f, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x73, 0x74, 0x65, 0x70, 0x2e,
    0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xdf, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x05, 0x12, 0x04, 0xdf, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x12, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdf, 0x01, 0x1d, 0x1e, 0x0a, 0x3f,
    0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0xe1, 0x01, 0x02, 0x1d, 0x1a, 0x31, 0x2f, 0x20,
    0x46, 0x6c, 0x61, 0x67, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20,
    0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x53, 0x74, 0x65,
    0x70, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x6c, 0x69, 0x61, 0x73, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe1, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x10, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x1b, 0x1c, 0x0a, 0x45, 0x0a, 0x02, 0x04,
    0x1c, 0x12, 0x06, 0xe5, 0x01, 0x00, 0xe7, 0x01, 0x01, 0x1a, 0x37, 0x2f, 0x20, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x61, 0x20, 0x75, 0x6e, 0x73,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74,
    0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xe6, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x1e, 0x1f, 0x0a, 0xc0, 0x01, 0x0a, 0x02, 0x04, 0x1d,
    0x12, 0x06, 0xec, 0x01, 0x00, 0xbe, 0x02, 0x01, 0x1a, 0xb1, 0x01, 0x2f, 0x20, 0x54, 0x68, 0x69,
    0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x67, 0x65, 0x74, 0x73, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x0a, 0x2f, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x70, 0x72, 0x6f, 0x70,
    0x65, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20,
    0x73, 0x65, 0x74, 0x0a, 0x2f, 0x20, 0x4f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x68, 0x61,
    0x76, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2c, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x1d, 0x01, 0x12, 0x04, 0xec, 0x01, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1d, 0x04,
    0x00, 0x12, 0x06, 0xed, 0x01, 0x02, 0x86, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x04,
    0x00, 0x01, 0x12, 0x04, 0xed, 0x01, 0x07, 0x12, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xee, 0x01, 0x06, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xee, 0x01, 0x06, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xee, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xef, 0x01, 0x06, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xef, 0x01, 0x06, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1d, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xef, 0x01, 0x1e, 0x1f, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1d, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xf0, 0x01, 0x06, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1d, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x06, 0x19, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xf0, 0x01, 0x1c, 0x1d, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x06, 0x24, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x06, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xf1, 0x01, 0x22, 0x23,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xf2, 0x01, 0x06, 0x22,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x06,
    0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xf2, 0x01,
    0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xf3, 0x01,
    0x06, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf3,
    0x01, 0x06, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04,
    0xf3, 0x01, 0x1e, 0x1f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04,
    0xf4, 0x01, 0x06, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xf4, 0x01, 0x06, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x06, 0x02,
    0x12, 0x04, 0xf4, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x07,
    0x12, 0x04, 0xf5, 0x01, 0x06, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x04, 0xf5, 0x01, 0x06, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02,
    0x07, 0x02, 0x12, 0x04, 0xf5, 0x01, 0x14, 0x15, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00,
    0x02, 0x08, 0x12, 0x04, 0xf6, 0x01, 0x06, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x06, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04,
    0x00, 0x02, 0x08, 0x02, 0x12, 0x04, 0xf6, 0x01, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d,
    0x04, 0x00, 0x02, 0x09, 0x12, 0x04, 0xf7, 0x01, 0x06, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d,
    0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x06, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1d, 0x04, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0xf7, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1d, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x04, 0xf8, 0x01, 0x06, 0x20, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1d, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x06, 0x1a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x04, 0xf8, 0x01, 0x1d, 0x1f, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x04, 0xf9, 0x01, 0x06, 0x23, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x06, 0x1d, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x04, 0xf9, 0x01, 0x20, 0x22,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x04, 0xfa, 0x01, 0x06, 0x1c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xfa, 0x01, 0x06,
    0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x04, 0xfa, 0x01,
    0x19, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x04, 0xfb, 0x01,
    0x06, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xfb,
    0x01, 0x06, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x04,
    0xfb, 0x01, 0x1a, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0e, 0x12, 0x04,
    0xfc, 0x01, 0x06, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12,
    0x04, 0xfc, 0x01, 0x06, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0e, 0x02,
    0x12, 0x04, 0xfc, 0x01, 0x1b, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0f,
    0x12, 0x04, 0xfd, 0x01, 0x06, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x0f,
    0x01, 0x12, 0x04, 0xfd, 0x01, 0x06, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02,
    0x0f, 0x02, 0x12, 0x04, 0xfd, 0x01, 0x1d, 0x1f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00,
    0x02, 0x10, 0x12, 0x04, 0xfe, 0x01, 0x06, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00,
    0x02, 0x10, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x06, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04,
    0x00, 0x02, 0x10, 0x02, 0x12, 0x04, 0xfe, 0x01, 0x1e, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d,
    0x04, 0x00, 0x02, 0x11, 0x12, 0x04, 0xff, 0x01, 0x06, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d,
    0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x04, 0xff, 0x01, 0x06, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1d, 0x04, 0x00, 0x02, 0x11, 0x02, 0x12, 0x04, 0xff, 0x01, 0x1a, 0x1c, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1d, 0x04, 0x00, 0x02, 0x12, 0x12, 0x04, 0x80, 0x02, 0x06, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1d, 0x04, 0x00, 0x02, 0x12, 0x01, 0x12, 0x04, 0x80, 0x02, 0x06, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x12, 0x02, 0x12, 0x04, 0x80, 0x02, 0x1b, 0x1d, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x13, 0x12, 0x04, 0x81, 0x02, 0x06, 0x1b, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x13, 0x01, 0x12, 0x04, 0x81, 0x02, 0x06, 0x15, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x13, 0x02, 0x12, 0x04, 0x81, 0x02, 0x18, 0x1a,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x14, 0x12, 0x04, 0x82, 0x02, 0x06, 0x1c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x14, 0x01, 0x12, 0x04, 0x82, 0x02, 0x06,
    0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x14, 0x02, 0x12, 0x04, 0x82, 0x02,
    0x19, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x15, 0x12, 0x04, 0x83, 0x02,
    0x06, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x15, 0x01, 0x12, 0x04, 0x83,
    0x02, 0x06, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x15, 0x02, 0x12, 0x04,
    0x83, 0x02, 0x18, 0x1a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x16, 0x12, 0x04,
    0x84, 0x02, 0x06, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x16, 0x01, 0x12,
    0x04, 0x84, 0x02, 0x06, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x16, 0x02,
    0x12, 0x04, 0x84, 0x02, 0x19, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x17,
    0x12, 0x04, 0x85, 0x02, 0x06, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02, 0x17,
    0x01, 0x12, 0x04, 0x85, 0x02, 0x06, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1d, 0x04, 0x00, 0x02,
    0x17, 0x02, 0x12, 0x04, 0x85, 0x02, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00,
    0x12, 0x04, 0x88, 0x02, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x88, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x88, 0x02, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88,
    0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x02,
    0x25, 0x26, 0x0a, 0xa2, 0x01, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x02,
    0x1f, 0x1a, 0x93, 0x01, 0x2f, 0x20, 0x41, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x69,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x20, 0x41, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x6f,
    0x70, 0x79, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x2e, 0x0a, 0x2f, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a,
    0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x26, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x8c, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x8c, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x8c, 0x02, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c,
    0x02, 0x1d, 0x1e, 0x0a, 0x55, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x02, 0x12, 0x04, 0x8f, 0x02, 0x02,
    0x41, 0x1a, 0x47, 0x2f, 0x20, 0x5b, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53,
    0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28,
    0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e,
    0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x8f, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x02, 0x06, 0x12, 0x04, 0x8f, 0x02, 0x0b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x8f, 0x02, 0x24, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x8f, 0x02, 0x3f, 0x40, 0x0a, 0x5d, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x03, 0x12, 0x04,
    0x91, 0x02, 0x02, 0x49, 0x1a, 0x4f, 0x2f, 0x20, 0x5b, 0x53, 0x70, 0x65, 0x63, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x70, 0x65, 0x63, 0x45, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x91, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x06, 0x12, 0x04, 0x91,
    0x02, 0x0b, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x01, 0x12, 0x04, 0x91, 0x02,
    0x28, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x03, 0x12, 0x04, 0x91, 0x02, 0x47,
    0x48, 0x0a, 0x59, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x04, 0x12, 0x04, 0x93, 0x02, 0x02, 0x45, 0x1a,
    0x4b, 0x2f, 0x20, 0x5b, 0x53, 0x70, 0x65, 0x63, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f,
    0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28,
    0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x53, 0x70, 0x65, 0x63, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x04, 0x04, 0x12, 0x04, 0x93, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x04, 0x06, 0x12, 0x04, 0x93, 0x02, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x93, 0x02, 0x26, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x93, 0x02, 0x43, 0x44, 0x0a, 0x65, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x05,
    0x12, 0x04, 0x95, 0x02, 0x02, 0x51, 0x1a, 0x57, 0x2f, 0x20, 0x5b, 0x53, 0x63, 0x65, 0x6e, 0x61,
    0x72, 0x69, 0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72,
    0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x63, 0x65,
    0x6e, 0x61, 0x72, 0x69, 0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x05, 0x04, 0x12, 0x04, 0x95, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x05, 0x06, 0x12, 0x04, 0x95, 0x02, 0x0b, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x05, 0x01, 0x12, 0x04, 0x95, 0x02, 0x2c, 0x4c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x05, 0x03, 0x12, 0x04, 0x95, 0x02, 0x4f, 0x50, 0x0a, 0x61, 0x0a, 0x04, 0x04,
    0x1d, 0x02, 0x06, 0x12, 0x04, 0x97, 0x02, 0x02, 0x4d, 0x1a, 0x53, 0x2f, 0x20, 0x5b, 0x53, 0x63,
    0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x63,
    0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x06, 0x04, 0x12, 0x04, 0x97, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x06, 0x06, 0x12, 0x04, 0x97, 0x02, 0x0b, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x06, 0x01, 0x12, 0x04, 0x97, 0x02, 0x2a, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x06, 0x03, 0x12, 0x04, 0x97, 0x02, 0x4b, 0x4c, 0x0a, 0x5d, 0x0a, 0x04, 0x04, 0x1d,
    0x02, 0x07, 0x12, 0x04, 0x99, 0x02, 0x02, 0x49, 0x1a, 0x4f, 0x2f, 0x20, 0x5b, 0x53, 0x74, 0x65,
    0x70, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67,
    0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x07, 0x04, 0x12, 0x04, 0x99, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x07,
    0x06, 0x12, 0x04, 0x99, 0x02, 0x0b, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x07, 0x01,
    0x12, 0x04, 0x99, 0x02, 0x28, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x07, 0x03, 0x12,
    0x04, 0x99, 0x02, 0x47, 0x48, 0x0a, 0x59, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x08, 0x12, 0x04, 0x9b,
    0x02, 0x02, 0x45, 0x1a, 0x4b, 0x2f, 0x20, 0x5b, 0x53, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f,
    0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x08, 0x04, 0x12, 0x04, 0x9b, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x08, 0x06, 0x12, 0x04, 0x9b, 0x02, 0x0b, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x08, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x26, 0x40, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x08, 0x03, 0x12, 0x04, 0x9b, 0x02, 0x43, 0x44, 0x0a, 0x49, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x09, 0x12, 0x04, 0x9d, 0x02, 0x02, 0x36, 0x1a, 0x3b, 0x2f, 0x20, 0x5b, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x53, 0x74, 0x65, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x53, 0x74, 0x65, 0x70, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x09, 0x04,
    0x12, 0x04, 0x9d, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x09, 0x06, 0x12,
    0x04, 0x9d, 0x02, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x09, 0x01, 0x12, 0x04,
    0x9d, 0x02, 0x1e, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x09, 0x03, 0x12, 0x04, 0x9d,
    0x02, 0x33, 0x35, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x0a, 0x12, 0x04, 0x9f, 0x02, 0x02,
    0x3e, 0x1a, 0x43, 0x2f, 0x20, 0x5b, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0a, 0x04, 0x12,
    0x04, 0x9f, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0a, 0x06, 0x12, 0x04,
    0x9f, 0x02, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x9f,
    0x02, 0x22, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x9f, 0x02,
    0x3b, 0x3d, 0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x0b, 0x12, 0x04, 0xa1, 0x02, 0x02, 0x38,
    0x1a, 0x3d, 0x2f, 0x20, 0x5b, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65,
    0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xa1, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0b, 0x06, 0x12, 0x04, 0xa1, 0x02, 0x0b, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x1f, 0x32, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x35, 0x37, 0x0a, 0x4d, 0x0a, 0x04, 0x04,
    0x1d, 0x02, 0x0c, 0x12, 0x04, 0xa3, 0x02, 0x02, 0x3a, 0x1a, 0x3f, 0x2f, 0x20, 0x5b, 0x53, 0x74,
    0x65, 0x70, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x0c, 0x04, 0x12, 0x04, 0xa3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x0c, 0x06, 0x12, 0x04, 0xa3, 0x02, 0x0b, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0c,
    0x01, 0x12, 0x04, 0xa3, 0x02, 0x20, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0c, 0x03,
    0x12, 0x04, 0xa3, 0x02, 0x37, 0x39, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x0d, 0x12, 0x04,
    0xa5, 0x02, 0x02, 0x40, 0x1a, 0x45, 0x2f, 0x20, 0x5b, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xa5, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x0d, 0x06, 0x12, 0x04, 0xa5, 0x02, 0x0b, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x0d, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x23, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0d,
    0x03, 0x12, 0x04, 0xa5, 0x02, 0x3d, 0x3f, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x0e, 0x12,
    0x04, 0xa7, 0x02, 0x02, 0x32, 0x1a, 0x37, 0x2f, 0x20, 0x5b, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61,
    0x6d, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75,
    0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70,
    0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xa7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x0e, 0x06, 0x12, 0x04, 0xa7, 0x02, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x1c, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xa7, 0x02, 0x2f, 0x31, 0x0a, 0x47, 0x0a, 0x04, 0x04, 0x1d,
    0x02, 0x0f, 0x12, 0x04, 0xa9, 0x02, 0x02, 0x34, 0x1a, 0x39, 0x2f, 0x20, 0x5b, 0x53, 0x74, 0x65,
    0x70, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28,
    0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0f, 0x04, 0x12, 0x04, 0xa9, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0f, 0x06, 0x12, 0x04, 0xa9, 0x02, 0x0b,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x1d, 0x2e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xa9, 0x02, 0x31, 0x33, 0x0a,
    0x4f, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x10, 0x12, 0x04, 0xab, 0x02, 0x02, 0x3a, 0x1a, 0x41, 0x2f,
    0x20, 0x5b, 0x53, 0x75, 0x69, 0x74, 0x65, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x75, 0x69, 0x74, 0x65, 0x45, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x29, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x10, 0x04, 0x12, 0x04, 0xab, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x10, 0x06, 0x12, 0x04, 0xab, 0x02, 0x0b, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x10, 0x01, 0x12, 0x04, 0xab, 0x02, 0x20, 0x34, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x10, 0x03, 0x12, 0x04, 0xab, 0x02, 0x37, 0x39, 0x0a, 0x49, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x11, 0x12, 0x04, 0xad, 0x02, 0x02, 0x36, 0x1a, 0x3b, 0x2f, 0x20, 0x5b, 0x4b,
    0x69, 0x6c, 0x6c, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x4b, 0x69, 0x6c, 0x6c, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x11, 0x04,
    0x12, 0x04, 0xad, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x11, 0x06, 0x12,
    0x04, 0xad, 0x02, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x11, 0x01, 0x12, 0x04,
    0xad, 0x02, 0x1e, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x11, 0x03, 0x12, 0x04, 0xad,
    0x02, 0x33, 0x35, 0x0a, 0x5d, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x12, 0x12, 0x04, 0xaf, 0x02, 0x02,
    0x4a, 0x1a, 0x4f, 0x2f, 0x20, 0x5b, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x44, 0x61,
    0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x53, 0x63, 0x65, 0x6e, 0x61, 0x72, 0x69, 0x6f, 0x44, 0x61, 0x74, 0x61,
    0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x12, 0x04, 0x12, 0x04, 0xaf, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x12, 0x06, 0x12, 0x04, 0xaf, 0x02, 0x0b, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x12, 0x01, 0x12, 0x04, 0xaf, 0x02, 0x28, 0x44, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x12, 0x03, 0x12, 0x04, 0xaf, 0x02, 0x47, 0x49, 0x0a, 0x55,
    0x0a, 0x04, 0x04, 0x1d, 0x02, 0x13, 0x12, 0x04, 0xb1, 0x02, 0x02, 0x42, 0x1a, 0x47, 0x2f, 0x20,
    0x5b, 0x53, 0x70, 0x65, 0x63, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e,
    0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67,
    0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x70, 0x65, 0x63, 0x44,
    0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x13, 0x04, 0x12, 0x04,
    0xb1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x13, 0x06, 0x12, 0x04, 0xb1,
    0x02, 0x0b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x13, 0x01, 0x12, 0x04, 0xb1, 0x02,
    0x24, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x13, 0x03, 0x12, 0x04, 0xb1, 0x02, 0x3f,
    0x41, 0x0a, 0x57, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x14, 0x12, 0x04, 0xb3, 0x02, 0x02, 0x44, 0x1a,
    0x49, 0x2f, 0x20, 0x5b, 0x53, 0x75, 0x69, 0x74, 0x65, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53,
    0x75, 0x69, 0x74, 0x65, 0x44, 0x61, 0x74, 0x61, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x6e, 0x69,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x14, 0x04, 0x12, 0x04, 0xb3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x14, 0x06, 0x12, 0x04, 0xb3, 0x02, 0x0b, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x14,
    0x01, 0x12, 0x04, 0xb3, 0x02, 0x25, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x14, 0x03,
    0x12, 0x04, 0xb3, 0x02, 0x41, 0x43, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x15, 0x12, 0x04,
    0xb5, 0x02, 0x02, 0x2f, 0x1a, 0x35, 0x2f, 0x20, 0x5b, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61, 0x6d,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65,
    0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61,
    0x6d, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x15, 0x04, 0x12, 0x04, 0xb5, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x15, 0x06, 0x12, 0x04, 0xb5, 0x02, 0x0b, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x15, 0x01, 0x12, 0x04, 0xb5, 0x02, 0x1b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x15,
    0x03, 0x12, 0x04, 0xb5, 0x02, 0x2c, 0x2e, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x16, 0x12,
    0x04, 0xb7, 0x02, 0x02, 0x31, 0x1a, 0x37, 0x2f, 0x20, 0x5b, 0x53, 0x74, 0x65, 0x70, 0x4e, 0x61,
    0x6d, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75,
    0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x53, 0x74, 0x65, 0x70,
    0x4e, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x16, 0x04, 0x12, 0x04, 0xb7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x16, 0x06, 0x12, 0x04, 0xb7, 0x02, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x16, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x1c, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x16, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x2e, 0x30, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x1d,
    0x02, 0x17, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x30, 0x1a, 0x35, 0x2f, 0x20, 0x5b, 0x52, 0x65, 0x66,
    0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x52, 0x65,
    0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x17, 0x04, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x17, 0x06, 0x12, 0x04, 0xb9, 0x02, 0x0b, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x17, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x1b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x17, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x2d, 0x2f, 0x0a, 0x45, 0x0a, 0x04, 0x04,
    0x1d, 0x02, 0x18, 0x12, 0x04, 0xbb, 0x02, 0x02, 0x32, 0x1a, 0x37, 0x2f, 0x20, 0x5b, 0x52, 0x65,
    0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28,
    0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x18, 0x04, 0x12, 0x04, 0xbb, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x18, 0x06, 0x12, 0x04, 0xbb, 0x02, 0x0b, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x18, 0x01, 0x12, 0x04, 0xbb, 0x02, 0x1c, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x18, 0x03, 0x12, 0x04, 0xbb, 0x02, 0x2f, 0x31, 0x0a, 0x59,
    0x0a, 0x04, 0x04, 0x1d, 0x02, 0x19, 0x12, 0x04, 0xbd, 0x02, 0x02, 0x46, 0x1a, 0x4b, 0x2f, 0x20,
    0x5b, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x73,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x19, 0x04, 0x12, 0x04, 0xbd, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x19,
    0x06, 0x12, 0x04, 0xbd, 0x02, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x19, 0x01,
    0x12, 0x04, 0xbd, 0x02, 0x26, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x19, 0x03, 0x12,
    0x04, 0xbd, 0x02, 0x43, 0x45,
];

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
