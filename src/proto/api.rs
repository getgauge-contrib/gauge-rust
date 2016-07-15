// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct GetProjectRootRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetProjectRootRequest {}

impl GetProjectRootRequest {
    pub fn new() -> GetProjectRootRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetProjectRootRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetProjectRootRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetProjectRootRequest,
        };
        unsafe {
            instance.get(|| {
                GetProjectRootRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetProjectRootRequest {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetProjectRootRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetProjectRootRequest {
    fn new() -> GetProjectRootRequest {
        GetProjectRootRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetProjectRootRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetProjectRootRequest>(
                    "GetProjectRootRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetProjectRootRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetProjectRootRequest {
    fn eq(&self, other: &GetProjectRootRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetProjectRootRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetProjectRootResponse {
    // message fields
    projectRoot: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetProjectRootResponse {}

impl GetProjectRootResponse {
    pub fn new() -> GetProjectRootResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetProjectRootResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetProjectRootResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetProjectRootResponse,
        };
        unsafe {
            instance.get(|| {
                GetProjectRootResponse {
                    projectRoot: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string projectRoot = 1;

    pub fn clear_projectRoot(&mut self) {
        self.projectRoot.clear();
    }

    pub fn has_projectRoot(&self) -> bool {
        self.projectRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_projectRoot(&mut self, v: ::std::string::String) {
        self.projectRoot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_projectRoot(&mut self) -> &mut ::std::string::String {
        if self.projectRoot.is_none() {
            self.projectRoot.set_default();
        };
        self.projectRoot.as_mut().unwrap()
    }

    // Take field
    pub fn take_projectRoot(&mut self) -> ::std::string::String {
        self.projectRoot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_projectRoot(&self) -> &str {
        match self.projectRoot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetProjectRootResponse {
    fn is_initialized(&self) -> bool {
        if self.projectRoot.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.projectRoot));
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
        for value in &self.projectRoot {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.projectRoot.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetProjectRootResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetProjectRootResponse {
    fn new() -> GetProjectRootResponse {
        GetProjectRootResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetProjectRootResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "projectRoot",
                    GetProjectRootResponse::has_projectRoot,
                    GetProjectRootResponse::get_projectRoot,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetProjectRootResponse>(
                    "GetProjectRootResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetProjectRootResponse {
    fn clear(&mut self) {
        self.clear_projectRoot();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetProjectRootResponse {
    fn eq(&self, other: &GetProjectRootResponse) -> bool {
        self.projectRoot == other.projectRoot &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetProjectRootResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetInstallationRootRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetInstallationRootRequest {}

impl GetInstallationRootRequest {
    pub fn new() -> GetInstallationRootRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetInstallationRootRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetInstallationRootRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetInstallationRootRequest,
        };
        unsafe {
            instance.get(|| {
                GetInstallationRootRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetInstallationRootRequest {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetInstallationRootRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetInstallationRootRequest {
    fn new() -> GetInstallationRootRequest {
        GetInstallationRootRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetInstallationRootRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetInstallationRootRequest>(
                    "GetInstallationRootRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetInstallationRootRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetInstallationRootRequest {
    fn eq(&self, other: &GetInstallationRootRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetInstallationRootRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetInstallationRootResponse {
    // message fields
    installationRoot: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetInstallationRootResponse {}

impl GetInstallationRootResponse {
    pub fn new() -> GetInstallationRootResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetInstallationRootResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetInstallationRootResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetInstallationRootResponse,
        };
        unsafe {
            instance.get(|| {
                GetInstallationRootResponse {
                    installationRoot: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string installationRoot = 1;

    pub fn clear_installationRoot(&mut self) {
        self.installationRoot.clear();
    }

    pub fn has_installationRoot(&self) -> bool {
        self.installationRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_installationRoot(&mut self, v: ::std::string::String) {
        self.installationRoot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_installationRoot(&mut self) -> &mut ::std::string::String {
        if self.installationRoot.is_none() {
            self.installationRoot.set_default();
        };
        self.installationRoot.as_mut().unwrap()
    }

    // Take field
    pub fn take_installationRoot(&mut self) -> ::std::string::String {
        self.installationRoot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_installationRoot(&self) -> &str {
        match self.installationRoot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetInstallationRootResponse {
    fn is_initialized(&self) -> bool {
        if self.installationRoot.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.installationRoot));
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
        for value in &self.installationRoot {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.installationRoot.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetInstallationRootResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetInstallationRootResponse {
    fn new() -> GetInstallationRootResponse {
        GetInstallationRootResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetInstallationRootResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "installationRoot",
                    GetInstallationRootResponse::has_installationRoot,
                    GetInstallationRootResponse::get_installationRoot,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetInstallationRootResponse>(
                    "GetInstallationRootResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetInstallationRootResponse {
    fn clear(&mut self) {
        self.clear_installationRoot();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetInstallationRootResponse {
    fn eq(&self, other: &GetInstallationRootResponse) -> bool {
        self.installationRoot == other.installationRoot &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetInstallationRootResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAllStepsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAllStepsRequest {}

impl GetAllStepsRequest {
    pub fn new() -> GetAllStepsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAllStepsRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetAllStepsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAllStepsRequest,
        };
        unsafe {
            instance.get(|| {
                GetAllStepsRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetAllStepsRequest {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetAllStepsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAllStepsRequest {
    fn new() -> GetAllStepsRequest {
        GetAllStepsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAllStepsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetAllStepsRequest>(
                    "GetAllStepsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAllStepsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAllStepsRequest {
    fn eq(&self, other: &GetAllStepsRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAllStepsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAllStepsResponse {
    // message fields
    allSteps: ::protobuf::RepeatedField<super::spec::ProtoStepValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAllStepsResponse {}

impl GetAllStepsResponse {
    pub fn new() -> GetAllStepsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAllStepsResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetAllStepsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAllStepsResponse,
        };
        unsafe {
            instance.get(|| {
                GetAllStepsResponse {
                    allSteps: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .gauge.messages.ProtoStepValue allSteps = 1;

    pub fn clear_allSteps(&mut self) {
        self.allSteps.clear();
    }

    // Param is passed by value, moved
    pub fn set_allSteps(&mut self, v: ::protobuf::RepeatedField<super::spec::ProtoStepValue>) {
        self.allSteps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_allSteps(&mut self) -> &mut ::protobuf::RepeatedField<super::spec::ProtoStepValue> {
        &mut self.allSteps
    }

    // Take field
    pub fn take_allSteps(&mut self) -> ::protobuf::RepeatedField<super::spec::ProtoStepValue> {
        ::std::mem::replace(&mut self.allSteps, ::protobuf::RepeatedField::new())
    }

    pub fn get_allSteps(&self) -> &[super::spec::ProtoStepValue] {
        &self.allSteps
    }
}

impl ::protobuf::Message for GetAllStepsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.allSteps));
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
        for value in &self.allSteps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.allSteps {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetAllStepsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAllStepsResponse {
    fn new() -> GetAllStepsResponse {
        GetAllStepsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAllStepsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "allSteps",
                    GetAllStepsResponse::get_allSteps,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAllStepsResponse>(
                    "GetAllStepsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAllStepsResponse {
    fn clear(&mut self) {
        self.clear_allSteps();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAllStepsResponse {
    fn eq(&self, other: &GetAllStepsResponse) -> bool {
        self.allSteps == other.allSteps &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAllStepsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAllSpecsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAllSpecsRequest {}

impl GetAllSpecsRequest {
    pub fn new() -> GetAllSpecsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAllSpecsRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetAllSpecsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAllSpecsRequest,
        };
        unsafe {
            instance.get(|| {
                GetAllSpecsRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetAllSpecsRequest {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetAllSpecsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAllSpecsRequest {
    fn new() -> GetAllSpecsRequest {
        GetAllSpecsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAllSpecsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetAllSpecsRequest>(
                    "GetAllSpecsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAllSpecsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAllSpecsRequest {
    fn eq(&self, other: &GetAllSpecsRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAllSpecsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAllSpecsResponse {
    // message fields
    specs: ::protobuf::RepeatedField<super::spec::ProtoSpec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAllSpecsResponse {}

impl GetAllSpecsResponse {
    pub fn new() -> GetAllSpecsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAllSpecsResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetAllSpecsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAllSpecsResponse,
        };
        unsafe {
            instance.get(|| {
                GetAllSpecsResponse {
                    specs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .gauge.messages.ProtoSpec specs = 1;

    pub fn clear_specs(&mut self) {
        self.specs.clear();
    }

    // Param is passed by value, moved
    pub fn set_specs(&mut self, v: ::protobuf::RepeatedField<super::spec::ProtoSpec>) {
        self.specs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_specs(&mut self) -> &mut ::protobuf::RepeatedField<super::spec::ProtoSpec> {
        &mut self.specs
    }

    // Take field
    pub fn take_specs(&mut self) -> ::protobuf::RepeatedField<super::spec::ProtoSpec> {
        ::std::mem::replace(&mut self.specs, ::protobuf::RepeatedField::new())
    }

    pub fn get_specs(&self) -> &[super::spec::ProtoSpec] {
        &self.specs
    }
}

impl ::protobuf::Message for GetAllSpecsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.specs));
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
        for value in &self.specs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.specs {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetAllSpecsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAllSpecsResponse {
    fn new() -> GetAllSpecsResponse {
        GetAllSpecsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAllSpecsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "specs",
                    GetAllSpecsResponse::get_specs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAllSpecsResponse>(
                    "GetAllSpecsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAllSpecsResponse {
    fn clear(&mut self) {
        self.clear_specs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAllSpecsResponse {
    fn eq(&self, other: &GetAllSpecsResponse) -> bool {
        self.specs == other.specs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAllSpecsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAllConceptsRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAllConceptsRequest {}

impl GetAllConceptsRequest {
    pub fn new() -> GetAllConceptsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAllConceptsRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetAllConceptsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAllConceptsRequest,
        };
        unsafe {
            instance.get(|| {
                GetAllConceptsRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetAllConceptsRequest {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetAllConceptsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAllConceptsRequest {
    fn new() -> GetAllConceptsRequest {
        GetAllConceptsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAllConceptsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetAllConceptsRequest>(
                    "GetAllConceptsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAllConceptsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAllConceptsRequest {
    fn eq(&self, other: &GetAllConceptsRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAllConceptsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAllConceptsResponse {
    // message fields
    concepts: ::protobuf::RepeatedField<ConceptInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetAllConceptsResponse {}

impl GetAllConceptsResponse {
    pub fn new() -> GetAllConceptsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAllConceptsResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetAllConceptsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAllConceptsResponse,
        };
        unsafe {
            instance.get(|| {
                GetAllConceptsResponse {
                    concepts: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .gauge.messages.ConceptInfo concepts = 1;

    pub fn clear_concepts(&mut self) {
        self.concepts.clear();
    }

    // Param is passed by value, moved
    pub fn set_concepts(&mut self, v: ::protobuf::RepeatedField<ConceptInfo>) {
        self.concepts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_concepts(&mut self) -> &mut ::protobuf::RepeatedField<ConceptInfo> {
        &mut self.concepts
    }

    // Take field
    pub fn take_concepts(&mut self) -> ::protobuf::RepeatedField<ConceptInfo> {
        ::std::mem::replace(&mut self.concepts, ::protobuf::RepeatedField::new())
    }

    pub fn get_concepts(&self) -> &[ConceptInfo] {
        &self.concepts
    }
}

impl ::protobuf::Message for GetAllConceptsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.concepts));
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
        for value in &self.concepts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.concepts {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetAllConceptsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAllConceptsResponse {
    fn new() -> GetAllConceptsResponse {
        GetAllConceptsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAllConceptsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "concepts",
                    GetAllConceptsResponse::get_concepts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAllConceptsResponse>(
                    "GetAllConceptsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAllConceptsResponse {
    fn clear(&mut self) {
        self.clear_concepts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAllConceptsResponse {
    fn eq(&self, other: &GetAllConceptsResponse) -> bool {
        self.concepts == other.concepts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAllConceptsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ConceptInfo {
    // message fields
    stepValue: ::protobuf::SingularPtrField<super::spec::ProtoStepValue>,
    filepath: ::protobuf::SingularField<::std::string::String>,
    lineNumber: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConceptInfo {}

impl ConceptInfo {
    pub fn new() -> ConceptInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConceptInfo {
        static mut instance: ::protobuf::lazy::Lazy<ConceptInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConceptInfo,
        };
        unsafe {
            instance.get(|| {
                ConceptInfo {
                    stepValue: ::protobuf::SingularPtrField::none(),
                    filepath: ::protobuf::SingularField::none(),
                    lineNumber: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.ProtoStepValue stepValue = 1;

    pub fn clear_stepValue(&mut self) {
        self.stepValue.clear();
    }

    pub fn has_stepValue(&self) -> bool {
        self.stepValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepValue(&mut self, v: super::spec::ProtoStepValue) {
        self.stepValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepValue(&mut self) -> &mut super::spec::ProtoStepValue {
        if self.stepValue.is_none() {
            self.stepValue.set_default();
        };
        self.stepValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepValue(&mut self) -> super::spec::ProtoStepValue {
        self.stepValue.take().unwrap_or_else(|| super::spec::ProtoStepValue::new())
    }

    pub fn get_stepValue(&self) -> &super::spec::ProtoStepValue {
        self.stepValue.as_ref().unwrap_or_else(|| super::spec::ProtoStepValue::default_instance())
    }

    // required string filepath = 2;

    pub fn clear_filepath(&mut self) {
        self.filepath.clear();
    }

    pub fn has_filepath(&self) -> bool {
        self.filepath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filepath(&mut self, v: ::std::string::String) {
        self.filepath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filepath(&mut self) -> &mut ::std::string::String {
        if self.filepath.is_none() {
            self.filepath.set_default();
        };
        self.filepath.as_mut().unwrap()
    }

    // Take field
    pub fn take_filepath(&mut self) -> ::std::string::String {
        self.filepath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filepath(&self) -> &str {
        match self.filepath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required int32 lineNumber = 3;

    pub fn clear_lineNumber(&mut self) {
        self.lineNumber = ::std::option::Option::None;
    }

    pub fn has_lineNumber(&self) -> bool {
        self.lineNumber.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lineNumber(&mut self, v: i32) {
        self.lineNumber = ::std::option::Option::Some(v);
    }

    pub fn get_lineNumber(&self) -> i32 {
        self.lineNumber.unwrap_or(0)
    }
}

impl ::protobuf::Message for ConceptInfo {
    fn is_initialized(&self) -> bool {
        if self.stepValue.is_none() {
            return false;
        };
        if self.filepath.is_none() {
            return false;
        };
        if self.lineNumber.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepValue));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filepath));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.lineNumber = ::std::option::Option::Some(tmp);
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
        for value in &self.stepValue {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.filepath {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.lineNumber {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stepValue.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.filepath.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.lineNumber {
            try!(os.write_int32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ConceptInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConceptInfo {
    fn new() -> ConceptInfo {
        ConceptInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConceptInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepValue",
                    ConceptInfo::has_stepValue,
                    ConceptInfo::get_stepValue,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "filepath",
                    ConceptInfo::has_filepath,
                    ConceptInfo::get_filepath,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "lineNumber",
                    ConceptInfo::has_lineNumber,
                    ConceptInfo::get_lineNumber,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConceptInfo>(
                    "ConceptInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConceptInfo {
    fn clear(&mut self) {
        self.clear_stepValue();
        self.clear_filepath();
        self.clear_lineNumber();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ConceptInfo {
    fn eq(&self, other: &ConceptInfo) -> bool {
        self.stepValue == other.stepValue &&
        self.filepath == other.filepath &&
        self.lineNumber == other.lineNumber &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ConceptInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetStepValueRequest {
    // message fields
    stepText: ::protobuf::SingularField<::std::string::String>,
    hasInlineTable: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetStepValueRequest {}

impl GetStepValueRequest {
    pub fn new() -> GetStepValueRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetStepValueRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetStepValueRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetStepValueRequest,
        };
        unsafe {
            instance.get(|| {
                GetStepValueRequest {
                    stepText: ::protobuf::SingularField::none(),
                    hasInlineTable: ::std::option::Option::None,
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
    pub fn mut_stepText(&mut self) -> &mut ::std::string::String {
        if self.stepText.is_none() {
            self.stepText.set_default();
        };
        self.stepText.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepText(&mut self) -> ::std::string::String {
        self.stepText.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_stepText(&self) -> &str {
        match self.stepText.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool hasInlineTable = 2;

    pub fn clear_hasInlineTable(&mut self) {
        self.hasInlineTable = ::std::option::Option::None;
    }

    pub fn has_hasInlineTable(&self) -> bool {
        self.hasInlineTable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hasInlineTable(&mut self, v: bool) {
        self.hasInlineTable = ::std::option::Option::Some(v);
    }

    pub fn get_hasInlineTable(&self) -> bool {
        self.hasInlineTable.unwrap_or(false)
    }
}

impl ::protobuf::Message for GetStepValueRequest {
    fn is_initialized(&self) -> bool {
        if self.stepText.is_none() {
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
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.hasInlineTable = ::std::option::Option::Some(tmp);
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
        for value in &self.stepText {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.hasInlineTable.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stepText.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.hasInlineTable {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetStepValueRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetStepValueRequest {
    fn new() -> GetStepValueRequest {
        GetStepValueRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetStepValueRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "stepText",
                    GetStepValueRequest::has_stepText,
                    GetStepValueRequest::get_stepText,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "hasInlineTable",
                    GetStepValueRequest::has_hasInlineTable,
                    GetStepValueRequest::get_hasInlineTable,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetStepValueRequest>(
                    "GetStepValueRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetStepValueRequest {
    fn clear(&mut self) {
        self.clear_stepText();
        self.clear_hasInlineTable();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetStepValueRequest {
    fn eq(&self, other: &GetStepValueRequest) -> bool {
        self.stepText == other.stepText &&
        self.hasInlineTable == other.hasInlineTable &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetStepValueRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetStepValueResponse {
    // message fields
    stepValue: ::protobuf::SingularPtrField<super::spec::ProtoStepValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetStepValueResponse {}

impl GetStepValueResponse {
    pub fn new() -> GetStepValueResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetStepValueResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetStepValueResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetStepValueResponse,
        };
        unsafe {
            instance.get(|| {
                GetStepValueResponse {
                    stepValue: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.ProtoStepValue stepValue = 1;

    pub fn clear_stepValue(&mut self) {
        self.stepValue.clear();
    }

    pub fn has_stepValue(&self) -> bool {
        self.stepValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepValue(&mut self, v: super::spec::ProtoStepValue) {
        self.stepValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepValue(&mut self) -> &mut super::spec::ProtoStepValue {
        if self.stepValue.is_none() {
            self.stepValue.set_default();
        };
        self.stepValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepValue(&mut self) -> super::spec::ProtoStepValue {
        self.stepValue.take().unwrap_or_else(|| super::spec::ProtoStepValue::new())
    }

    pub fn get_stepValue(&self) -> &super::spec::ProtoStepValue {
        self.stepValue.as_ref().unwrap_or_else(|| super::spec::ProtoStepValue::default_instance())
    }
}

impl ::protobuf::Message for GetStepValueResponse {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepValue));
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
        for value in &self.stepValue {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stepValue.as_ref() {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetStepValueResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetStepValueResponse {
    fn new() -> GetStepValueResponse {
        GetStepValueResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetStepValueResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepValue",
                    GetStepValueResponse::has_stepValue,
                    GetStepValueResponse::get_stepValue,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetStepValueResponse>(
                    "GetStepValueResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetStepValueResponse {
    fn clear(&mut self) {
        self.clear_stepValue();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetStepValueResponse {
    fn eq(&self, other: &GetStepValueResponse) -> bool {
        self.stepValue == other.stepValue &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetStepValueResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetLanguagePluginLibPathRequest {
    // message fields
    language: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetLanguagePluginLibPathRequest {}

impl GetLanguagePluginLibPathRequest {
    pub fn new() -> GetLanguagePluginLibPathRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetLanguagePluginLibPathRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetLanguagePluginLibPathRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetLanguagePluginLibPathRequest,
        };
        unsafe {
            instance.get(|| {
                GetLanguagePluginLibPathRequest {
                    language: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string language = 1;

    pub fn clear_language(&mut self) {
        self.language.clear();
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: ::std::string::String) {
        self.language = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_language(&mut self) -> &mut ::std::string::String {
        if self.language.is_none() {
            self.language.set_default();
        };
        self.language.as_mut().unwrap()
    }

    // Take field
    pub fn take_language(&mut self) -> ::std::string::String {
        self.language.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_language(&self) -> &str {
        match self.language.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetLanguagePluginLibPathRequest {
    fn is_initialized(&self) -> bool {
        if self.language.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.language));
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
        for value in &self.language {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.language.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetLanguagePluginLibPathRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetLanguagePluginLibPathRequest {
    fn new() -> GetLanguagePluginLibPathRequest {
        GetLanguagePluginLibPathRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetLanguagePluginLibPathRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "language",
                    GetLanguagePluginLibPathRequest::has_language,
                    GetLanguagePluginLibPathRequest::get_language,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetLanguagePluginLibPathRequest>(
                    "GetLanguagePluginLibPathRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetLanguagePluginLibPathRequest {
    fn clear(&mut self) {
        self.clear_language();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetLanguagePluginLibPathRequest {
    fn eq(&self, other: &GetLanguagePluginLibPathRequest) -> bool {
        self.language == other.language &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetLanguagePluginLibPathRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetLanguagePluginLibPathResponse {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetLanguagePluginLibPathResponse {}

impl GetLanguagePluginLibPathResponse {
    pub fn new() -> GetLanguagePluginLibPathResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetLanguagePluginLibPathResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetLanguagePluginLibPathResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetLanguagePluginLibPathResponse,
        };
        unsafe {
            instance.get(|| {
                GetLanguagePluginLibPathResponse {
                    path: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetLanguagePluginLibPathResponse {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
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
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetLanguagePluginLibPathResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetLanguagePluginLibPathResponse {
    fn new() -> GetLanguagePluginLibPathResponse {
        GetLanguagePluginLibPathResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetLanguagePluginLibPathResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    GetLanguagePluginLibPathResponse::has_path,
                    GetLanguagePluginLibPathResponse::get_path,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetLanguagePluginLibPathResponse>(
                    "GetLanguagePluginLibPathResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetLanguagePluginLibPathResponse {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetLanguagePluginLibPathResponse {
    fn eq(&self, other: &GetLanguagePluginLibPathResponse) -> bool {
        self.path == other.path &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetLanguagePluginLibPathResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ErrorResponse {
    // message fields
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ErrorResponse {}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ErrorResponse {
        static mut instance: ::protobuf::lazy::Lazy<ErrorResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErrorResponse,
        };
        unsafe {
            instance.get(|| {
                ErrorResponse {
                    error: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string error = 1;

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
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ErrorResponse {
    fn is_initialized(&self) -> bool {
        if self.error.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error));
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
        for value in &self.error {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ErrorResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ErrorResponse {
    fn new() -> ErrorResponse {
        ErrorResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ErrorResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    ErrorResponse::has_error,
                    ErrorResponse::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ErrorResponse>(
                    "ErrorResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ErrorResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ErrorResponse {
    fn eq(&self, other: &ErrorResponse) -> bool {
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ErrorResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PerformRefactoringRequest {
    // message fields
    oldStep: ::protobuf::SingularField<::std::string::String>,
    newStep: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PerformRefactoringRequest {}

impl PerformRefactoringRequest {
    pub fn new() -> PerformRefactoringRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PerformRefactoringRequest {
        static mut instance: ::protobuf::lazy::Lazy<PerformRefactoringRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PerformRefactoringRequest,
        };
        unsafe {
            instance.get(|| {
                PerformRefactoringRequest {
                    oldStep: ::protobuf::SingularField::none(),
                    newStep: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string oldStep = 1;

    pub fn clear_oldStep(&mut self) {
        self.oldStep.clear();
    }

    pub fn has_oldStep(&self) -> bool {
        self.oldStep.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oldStep(&mut self, v: ::std::string::String) {
        self.oldStep = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oldStep(&mut self) -> &mut ::std::string::String {
        if self.oldStep.is_none() {
            self.oldStep.set_default();
        };
        self.oldStep.as_mut().unwrap()
    }

    // Take field
    pub fn take_oldStep(&mut self) -> ::std::string::String {
        self.oldStep.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_oldStep(&self) -> &str {
        match self.oldStep.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string newStep = 2;

    pub fn clear_newStep(&mut self) {
        self.newStep.clear();
    }

    pub fn has_newStep(&self) -> bool {
        self.newStep.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newStep(&mut self, v: ::std::string::String) {
        self.newStep = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_newStep(&mut self) -> &mut ::std::string::String {
        if self.newStep.is_none() {
            self.newStep.set_default();
        };
        self.newStep.as_mut().unwrap()
    }

    // Take field
    pub fn take_newStep(&mut self) -> ::std::string::String {
        self.newStep.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_newStep(&self) -> &str {
        match self.newStep.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for PerformRefactoringRequest {
    fn is_initialized(&self) -> bool {
        if self.oldStep.is_none() {
            return false;
        };
        if self.newStep.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.oldStep));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.newStep));
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
        for value in &self.oldStep {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.newStep {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.oldStep.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.newStep.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PerformRefactoringRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PerformRefactoringRequest {
    fn new() -> PerformRefactoringRequest {
        PerformRefactoringRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PerformRefactoringRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "oldStep",
                    PerformRefactoringRequest::has_oldStep,
                    PerformRefactoringRequest::get_oldStep,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "newStep",
                    PerformRefactoringRequest::has_newStep,
                    PerformRefactoringRequest::get_newStep,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PerformRefactoringRequest>(
                    "PerformRefactoringRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PerformRefactoringRequest {
    fn clear(&mut self) {
        self.clear_oldStep();
        self.clear_newStep();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PerformRefactoringRequest {
    fn eq(&self, other: &PerformRefactoringRequest) -> bool {
        self.oldStep == other.oldStep &&
        self.newStep == other.newStep &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PerformRefactoringRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PerformRefactoringResponse {
    // message fields
    success: ::std::option::Option<bool>,
    errors: ::protobuf::RepeatedField<::std::string::String>,
    filesChanged: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PerformRefactoringResponse {}

impl PerformRefactoringResponse {
    pub fn new() -> PerformRefactoringResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PerformRefactoringResponse {
        static mut instance: ::protobuf::lazy::Lazy<PerformRefactoringResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PerformRefactoringResponse,
        };
        unsafe {
            instance.get(|| {
                PerformRefactoringResponse {
                    success: ::std::option::Option::None,
                    errors: ::protobuf::RepeatedField::new(),
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

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // repeated string errors = 2;

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_errors(&self) -> &[::std::string::String] {
        &self.errors
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
    pub fn mut_filesChanged(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.filesChanged
    }

    // Take field
    pub fn take_filesChanged(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.filesChanged, ::protobuf::RepeatedField::new())
    }

    pub fn get_filesChanged(&self) -> &[::std::string::String] {
        &self.filesChanged
    }
}

impl ::protobuf::Message for PerformRefactoringResponse {
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
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.errors));
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
        for value in &self.errors {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.filesChanged {
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
        for v in &self.errors {
            try!(os.write_string(2, &v));
        };
        for v in &self.filesChanged {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PerformRefactoringResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PerformRefactoringResponse {
    fn new() -> PerformRefactoringResponse {
        PerformRefactoringResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PerformRefactoringResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    PerformRefactoringResponse::has_success,
                    PerformRefactoringResponse::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "errors",
                    PerformRefactoringResponse::get_errors,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "filesChanged",
                    PerformRefactoringResponse::get_filesChanged,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PerformRefactoringResponse>(
                    "PerformRefactoringResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PerformRefactoringResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_errors();
        self.clear_filesChanged();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PerformRefactoringResponse {
    fn eq(&self, other: &PerformRefactoringResponse) -> bool {
        self.success == other.success &&
        self.errors == other.errors &&
        self.filesChanged == other.filesChanged &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PerformRefactoringResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExtractConceptInfoRequest {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExtractConceptInfoRequest {}

impl ExtractConceptInfoRequest {
    pub fn new() -> ExtractConceptInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtractConceptInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<ExtractConceptInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtractConceptInfoRequest,
        };
        unsafe {
            instance.get(|| {
                ExtractConceptInfoRequest {
                    text: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ExtractConceptInfoRequest {
    fn is_initialized(&self) -> bool {
        if self.text.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
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
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.text.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExtractConceptInfoRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExtractConceptInfoRequest {
    fn new() -> ExtractConceptInfoRequest {
        ExtractConceptInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtractConceptInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    ExtractConceptInfoRequest::has_text,
                    ExtractConceptInfoRequest::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtractConceptInfoRequest>(
                    "ExtractConceptInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtractConceptInfoRequest {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExtractConceptInfoRequest {
    fn eq(&self, other: &ExtractConceptInfoRequest) -> bool {
        self.text == other.text &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExtractConceptInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExtractConceptRequest {
    // message fields
    conceptName: ::protobuf::SingularPtrField<step>,
    steps: ::protobuf::RepeatedField<step>,
    changeAcrossProject: ::std::option::Option<bool>,
    conceptFileName: ::protobuf::SingularField<::std::string::String>,
    selectedTextInfo: ::protobuf::SingularPtrField<textInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExtractConceptRequest {}

impl ExtractConceptRequest {
    pub fn new() -> ExtractConceptRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtractConceptRequest {
        static mut instance: ::protobuf::lazy::Lazy<ExtractConceptRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtractConceptRequest,
        };
        unsafe {
            instance.get(|| {
                ExtractConceptRequest {
                    conceptName: ::protobuf::SingularPtrField::none(),
                    steps: ::protobuf::RepeatedField::new(),
                    changeAcrossProject: ::std::option::Option::None,
                    conceptFileName: ::protobuf::SingularField::none(),
                    selectedTextInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.step conceptName = 1;

    pub fn clear_conceptName(&mut self) {
        self.conceptName.clear();
    }

    pub fn has_conceptName(&self) -> bool {
        self.conceptName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conceptName(&mut self, v: step) {
        self.conceptName = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conceptName(&mut self) -> &mut step {
        if self.conceptName.is_none() {
            self.conceptName.set_default();
        };
        self.conceptName.as_mut().unwrap()
    }

    // Take field
    pub fn take_conceptName(&mut self) -> step {
        self.conceptName.take().unwrap_or_else(|| step::new())
    }

    pub fn get_conceptName(&self) -> &step {
        self.conceptName.as_ref().unwrap_or_else(|| step::default_instance())
    }

    // repeated .gauge.messages.step steps = 2;

    pub fn clear_steps(&mut self) {
        self.steps.clear();
    }

    // Param is passed by value, moved
    pub fn set_steps(&mut self, v: ::protobuf::RepeatedField<step>) {
        self.steps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_steps(&mut self) -> &mut ::protobuf::RepeatedField<step> {
        &mut self.steps
    }

    // Take field
    pub fn take_steps(&mut self) -> ::protobuf::RepeatedField<step> {
        ::std::mem::replace(&mut self.steps, ::protobuf::RepeatedField::new())
    }

    pub fn get_steps(&self) -> &[step] {
        &self.steps
    }

    // required bool changeAcrossProject = 3;

    pub fn clear_changeAcrossProject(&mut self) {
        self.changeAcrossProject = ::std::option::Option::None;
    }

    pub fn has_changeAcrossProject(&self) -> bool {
        self.changeAcrossProject.is_some()
    }

    // Param is passed by value, moved
    pub fn set_changeAcrossProject(&mut self, v: bool) {
        self.changeAcrossProject = ::std::option::Option::Some(v);
    }

    pub fn get_changeAcrossProject(&self) -> bool {
        self.changeAcrossProject.unwrap_or(false)
    }

    // required string conceptFileName = 4;

    pub fn clear_conceptFileName(&mut self) {
        self.conceptFileName.clear();
    }

    pub fn has_conceptFileName(&self) -> bool {
        self.conceptFileName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conceptFileName(&mut self, v: ::std::string::String) {
        self.conceptFileName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conceptFileName(&mut self) -> &mut ::std::string::String {
        if self.conceptFileName.is_none() {
            self.conceptFileName.set_default();
        };
        self.conceptFileName.as_mut().unwrap()
    }

    // Take field
    pub fn take_conceptFileName(&mut self) -> ::std::string::String {
        self.conceptFileName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_conceptFileName(&self) -> &str {
        match self.conceptFileName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .gauge.messages.textInfo selectedTextInfo = 5;

    pub fn clear_selectedTextInfo(&mut self) {
        self.selectedTextInfo.clear();
    }

    pub fn has_selectedTextInfo(&self) -> bool {
        self.selectedTextInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selectedTextInfo(&mut self, v: textInfo) {
        self.selectedTextInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selectedTextInfo(&mut self) -> &mut textInfo {
        if self.selectedTextInfo.is_none() {
            self.selectedTextInfo.set_default();
        };
        self.selectedTextInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_selectedTextInfo(&mut self) -> textInfo {
        self.selectedTextInfo.take().unwrap_or_else(|| textInfo::new())
    }

    pub fn get_selectedTextInfo(&self) -> &textInfo {
        self.selectedTextInfo.as_ref().unwrap_or_else(|| textInfo::default_instance())
    }
}

impl ::protobuf::Message for ExtractConceptRequest {
    fn is_initialized(&self) -> bool {
        if self.conceptName.is_none() {
            return false;
        };
        if self.changeAcrossProject.is_none() {
            return false;
        };
        if self.conceptFileName.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.conceptName));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.steps));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.changeAcrossProject = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.conceptFileName));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selectedTextInfo));
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
        for value in &self.conceptName {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.steps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.changeAcrossProject.is_some() {
            my_size += 2;
        };
        for value in &self.conceptFileName {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.selectedTextInfo {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.conceptName.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.steps {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.changeAcrossProject {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.conceptFileName.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.selectedTextInfo.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExtractConceptRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExtractConceptRequest {
    fn new() -> ExtractConceptRequest {
        ExtractConceptRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtractConceptRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "conceptName",
                    ExtractConceptRequest::has_conceptName,
                    ExtractConceptRequest::get_conceptName,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "steps",
                    ExtractConceptRequest::get_steps,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "changeAcrossProject",
                    ExtractConceptRequest::has_changeAcrossProject,
                    ExtractConceptRequest::get_changeAcrossProject,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "conceptFileName",
                    ExtractConceptRequest::has_conceptFileName,
                    ExtractConceptRequest::get_conceptFileName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "selectedTextInfo",
                    ExtractConceptRequest::has_selectedTextInfo,
                    ExtractConceptRequest::get_selectedTextInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtractConceptRequest>(
                    "ExtractConceptRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtractConceptRequest {
    fn clear(&mut self) {
        self.clear_conceptName();
        self.clear_steps();
        self.clear_changeAcrossProject();
        self.clear_conceptFileName();
        self.clear_selectedTextInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExtractConceptRequest {
    fn eq(&self, other: &ExtractConceptRequest) -> bool {
        self.conceptName == other.conceptName &&
        self.steps == other.steps &&
        self.changeAcrossProject == other.changeAcrossProject &&
        self.conceptFileName == other.conceptFileName &&
        self.selectedTextInfo == other.selectedTextInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExtractConceptRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct textInfo {
    // message fields
    fileName: ::protobuf::SingularField<::std::string::String>,
    startingLineNo: ::std::option::Option<i32>,
    endLineNo: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for textInfo {}

impl textInfo {
    pub fn new() -> textInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static textInfo {
        static mut instance: ::protobuf::lazy::Lazy<textInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const textInfo,
        };
        unsafe {
            instance.get(|| {
                textInfo {
                    fileName: ::protobuf::SingularField::none(),
                    startingLineNo: ::std::option::Option::None,
                    endLineNo: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string fileName = 1;

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
    pub fn mut_fileName(&mut self) -> &mut ::std::string::String {
        if self.fileName.is_none() {
            self.fileName.set_default();
        };
        self.fileName.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileName(&mut self) -> ::std::string::String {
        self.fileName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fileName(&self) -> &str {
        match self.fileName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required int32 startingLineNo = 2;

    pub fn clear_startingLineNo(&mut self) {
        self.startingLineNo = ::std::option::Option::None;
    }

    pub fn has_startingLineNo(&self) -> bool {
        self.startingLineNo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startingLineNo(&mut self, v: i32) {
        self.startingLineNo = ::std::option::Option::Some(v);
    }

    pub fn get_startingLineNo(&self) -> i32 {
        self.startingLineNo.unwrap_or(0)
    }

    // required int32 endLineNo = 3;

    pub fn clear_endLineNo(&mut self) {
        self.endLineNo = ::std::option::Option::None;
    }

    pub fn has_endLineNo(&self) -> bool {
        self.endLineNo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endLineNo(&mut self, v: i32) {
        self.endLineNo = ::std::option::Option::Some(v);
    }

    pub fn get_endLineNo(&self) -> i32 {
        self.endLineNo.unwrap_or(0)
    }
}

impl ::protobuf::Message for textInfo {
    fn is_initialized(&self) -> bool {
        if self.fileName.is_none() {
            return false;
        };
        if self.startingLineNo.is_none() {
            return false;
        };
        if self.endLineNo.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fileName));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.startingLineNo = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.endLineNo = ::std::option::Option::Some(tmp);
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
        for value in &self.fileName {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.startingLineNo {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.endLineNo {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fileName.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.startingLineNo {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.endLineNo {
            try!(os.write_int32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<textInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for textInfo {
    fn new() -> textInfo {
        textInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<textInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fileName",
                    textInfo::has_fileName,
                    textInfo::get_fileName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "startingLineNo",
                    textInfo::has_startingLineNo,
                    textInfo::get_startingLineNo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "endLineNo",
                    textInfo::has_endLineNo,
                    textInfo::get_endLineNo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<textInfo>(
                    "textInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for textInfo {
    fn clear(&mut self) {
        self.clear_fileName();
        self.clear_startingLineNo();
        self.clear_endLineNo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for textInfo {
    fn eq(&self, other: &textInfo) -> bool {
        self.fileName == other.fileName &&
        self.startingLineNo == other.startingLineNo &&
        self.endLineNo == other.endLineNo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for textInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct step {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    table: ::protobuf::SingularField<::std::string::String>,
    paramTableName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for step {}

impl step {
    pub fn new() -> step {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static step {
        static mut instance: ::protobuf::lazy::Lazy<step> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const step,
        };
        unsafe {
            instance.get(|| {
                step {
                    name: ::protobuf::SingularField::none(),
                    table: ::protobuf::SingularField::none(),
                    paramTableName: ::protobuf::SingularField::none(),
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
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string table = 2;

    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    pub fn has_table(&self) -> bool {
        self.table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: ::std::string::String) {
        self.table = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut ::std::string::String {
        if self.table.is_none() {
            self.table.set_default();
        };
        self.table.as_mut().unwrap()
    }

    // Take field
    pub fn take_table(&mut self) -> ::std::string::String {
        self.table.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_table(&self) -> &str {
        match self.table.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string paramTableName = 3;

    pub fn clear_paramTableName(&mut self) {
        self.paramTableName.clear();
    }

    pub fn has_paramTableName(&self) -> bool {
        self.paramTableName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_paramTableName(&mut self, v: ::std::string::String) {
        self.paramTableName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_paramTableName(&mut self) -> &mut ::std::string::String {
        if self.paramTableName.is_none() {
            self.paramTableName.set_default();
        };
        self.paramTableName.as_mut().unwrap()
    }

    // Take field
    pub fn take_paramTableName(&mut self) -> ::std::string::String {
        self.paramTableName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_paramTableName(&self) -> &str {
        match self.paramTableName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for step {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.paramTableName));
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.table {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.paramTableName {
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
        if let Some(v) = self.table.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.paramTableName.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<step>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for step {
    fn new() -> step {
        step::new()
    }

    fn descriptor_static(_: ::std::option::Option<step>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    step::has_name,
                    step::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "table",
                    step::has_table,
                    step::get_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "paramTableName",
                    step::has_paramTableName,
                    step::get_paramTableName,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<step>(
                    "step",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for step {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_table();
        self.clear_paramTableName();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for step {
    fn eq(&self, other: &step) -> bool {
        self.name == other.name &&
        self.table == other.table &&
        self.paramTableName == other.paramTableName &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for step {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExtractConceptResponse {
    // message fields
    isSuccess: ::std::option::Option<bool>,
    error: ::protobuf::SingularField<::std::string::String>,
    filesChanged: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExtractConceptResponse {}

impl ExtractConceptResponse {
    pub fn new() -> ExtractConceptResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtractConceptResponse {
        static mut instance: ::protobuf::lazy::Lazy<ExtractConceptResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtractConceptResponse,
        };
        unsafe {
            instance.get(|| {
                ExtractConceptResponse {
                    isSuccess: ::std::option::Option::None,
                    error: ::protobuf::SingularField::none(),
                    filesChanged: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool isSuccess = 1;

    pub fn clear_isSuccess(&mut self) {
        self.isSuccess = ::std::option::Option::None;
    }

    pub fn has_isSuccess(&self) -> bool {
        self.isSuccess.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isSuccess(&mut self, v: bool) {
        self.isSuccess = ::std::option::Option::Some(v);
    }

    pub fn get_isSuccess(&self) -> bool {
        self.isSuccess.unwrap_or(false)
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
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
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
    pub fn mut_filesChanged(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.filesChanged
    }

    // Take field
    pub fn take_filesChanged(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.filesChanged, ::protobuf::RepeatedField::new())
    }

    pub fn get_filesChanged(&self) -> &[::std::string::String] {
        &self.filesChanged
    }
}

impl ::protobuf::Message for ExtractConceptResponse {
    fn is_initialized(&self) -> bool {
        if self.isSuccess.is_none() {
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
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.isSuccess = ::std::option::Option::Some(tmp);
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
        if self.isSuccess.is_some() {
            my_size += 2;
        };
        for value in &self.error {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.filesChanged {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.isSuccess {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.filesChanged {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExtractConceptResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExtractConceptResponse {
    fn new() -> ExtractConceptResponse {
        ExtractConceptResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtractConceptResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isSuccess",
                    ExtractConceptResponse::has_isSuccess,
                    ExtractConceptResponse::get_isSuccess,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    ExtractConceptResponse::has_error,
                    ExtractConceptResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "filesChanged",
                    ExtractConceptResponse::get_filesChanged,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtractConceptResponse>(
                    "ExtractConceptResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtractConceptResponse {
    fn clear(&mut self) {
        self.clear_isSuccess();
        self.clear_error();
        self.clear_filesChanged();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExtractConceptResponse {
    fn eq(&self, other: &ExtractConceptResponse) -> bool {
        self.isSuccess == other.isSuccess &&
        self.error == other.error &&
        self.filesChanged == other.filesChanged &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExtractConceptResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FormatSpecsRequest {
    // message fields
    specs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FormatSpecsRequest {}

impl FormatSpecsRequest {
    pub fn new() -> FormatSpecsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FormatSpecsRequest {
        static mut instance: ::protobuf::lazy::Lazy<FormatSpecsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FormatSpecsRequest,
        };
        unsafe {
            instance.get(|| {
                FormatSpecsRequest {
                    specs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string specs = 1;

    pub fn clear_specs(&mut self) {
        self.specs.clear();
    }

    // Param is passed by value, moved
    pub fn set_specs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.specs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_specs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.specs
    }

    // Take field
    pub fn take_specs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.specs, ::protobuf::RepeatedField::new())
    }

    pub fn get_specs(&self) -> &[::std::string::String] {
        &self.specs
    }
}

impl ::protobuf::Message for FormatSpecsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.specs));
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
        for value in &self.specs {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.specs {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FormatSpecsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FormatSpecsRequest {
    fn new() -> FormatSpecsRequest {
        FormatSpecsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<FormatSpecsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "specs",
                    FormatSpecsRequest::get_specs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FormatSpecsRequest>(
                    "FormatSpecsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FormatSpecsRequest {
    fn clear(&mut self) {
        self.clear_specs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FormatSpecsRequest {
    fn eq(&self, other: &FormatSpecsRequest) -> bool {
        self.specs == other.specs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FormatSpecsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FormatSpecsResponse {
    // message fields
    errors: ::protobuf::RepeatedField<::std::string::String>,
    warnings: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FormatSpecsResponse {}

impl FormatSpecsResponse {
    pub fn new() -> FormatSpecsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FormatSpecsResponse {
        static mut instance: ::protobuf::lazy::Lazy<FormatSpecsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FormatSpecsResponse,
        };
        unsafe {
            instance.get(|| {
                FormatSpecsResponse {
                    errors: ::protobuf::RepeatedField::new(),
                    warnings: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string errors = 1;

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_errors(&self) -> &[::std::string::String] {
        &self.errors
    }

    // repeated string warnings = 2;

    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }

    // Param is passed by value, moved
    pub fn set_warnings(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.warnings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_warnings(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.warnings
    }

    // Take field
    pub fn take_warnings(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.warnings, ::protobuf::RepeatedField::new())
    }

    pub fn get_warnings(&self) -> &[::std::string::String] {
        &self.warnings
    }
}

impl ::protobuf::Message for FormatSpecsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.errors));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.warnings));
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
        for value in &self.errors {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.warnings {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.errors {
            try!(os.write_string(1, &v));
        };
        for v in &self.warnings {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FormatSpecsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FormatSpecsResponse {
    fn new() -> FormatSpecsResponse {
        FormatSpecsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FormatSpecsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "errors",
                    FormatSpecsResponse::get_errors,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "warnings",
                    FormatSpecsResponse::get_warnings,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FormatSpecsResponse>(
                    "FormatSpecsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FormatSpecsResponse {
    fn clear(&mut self) {
        self.clear_errors();
        self.clear_warnings();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FormatSpecsResponse {
    fn eq(&self, other: &FormatSpecsResponse) -> bool {
        self.errors == other.errors &&
        self.warnings == other.warnings &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FormatSpecsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UnsupportedApiMessageResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnsupportedApiMessageResponse {}

impl UnsupportedApiMessageResponse {
    pub fn new() -> UnsupportedApiMessageResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnsupportedApiMessageResponse {
        static mut instance: ::protobuf::lazy::Lazy<UnsupportedApiMessageResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnsupportedApiMessageResponse,
        };
        unsafe {
            instance.get(|| {
                UnsupportedApiMessageResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for UnsupportedApiMessageResponse {
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

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UnsupportedApiMessageResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UnsupportedApiMessageResponse {
    fn new() -> UnsupportedApiMessageResponse {
        UnsupportedApiMessageResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnsupportedApiMessageResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<UnsupportedApiMessageResponse>(
                    "UnsupportedApiMessageResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnsupportedApiMessageResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UnsupportedApiMessageResponse {
    fn eq(&self, other: &UnsupportedApiMessageResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UnsupportedApiMessageResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct APIMessage {
    // message fields
    messageType: ::std::option::Option<APIMessage_APIMessageType>,
    messageId: ::std::option::Option<i64>,
    projectRootRequest: ::protobuf::SingularPtrField<GetProjectRootRequest>,
    projectRootResponse: ::protobuf::SingularPtrField<GetProjectRootResponse>,
    installationRootRequest: ::protobuf::SingularPtrField<GetInstallationRootRequest>,
    installationRootResponse: ::protobuf::SingularPtrField<GetInstallationRootResponse>,
    allStepsRequest: ::protobuf::SingularPtrField<GetAllStepsRequest>,
    allStepsResponse: ::protobuf::SingularPtrField<GetAllStepsResponse>,
    allSpecsRequest: ::protobuf::SingularPtrField<GetAllSpecsRequest>,
    allSpecsResponse: ::protobuf::SingularPtrField<GetAllSpecsResponse>,
    stepValueRequest: ::protobuf::SingularPtrField<GetStepValueRequest>,
    stepValueResponse: ::protobuf::SingularPtrField<GetStepValueResponse>,
    libPathRequest: ::protobuf::SingularPtrField<GetLanguagePluginLibPathRequest>,
    libPathResponse: ::protobuf::SingularPtrField<GetLanguagePluginLibPathResponse>,
    error: ::protobuf::SingularPtrField<ErrorResponse>,
    allConceptsRequest: ::protobuf::SingularPtrField<GetAllConceptsRequest>,
    allConceptsResponse: ::protobuf::SingularPtrField<GetAllConceptsResponse>,
    performRefactoringRequest: ::protobuf::SingularPtrField<PerformRefactoringRequest>,
    performRefactoringResponse: ::protobuf::SingularPtrField<PerformRefactoringResponse>,
    extractConceptRequest: ::protobuf::SingularPtrField<ExtractConceptRequest>,
    extractConceptResponse: ::protobuf::SingularPtrField<ExtractConceptResponse>,
    formatSpecsRequest: ::protobuf::SingularPtrField<FormatSpecsRequest>,
    formatSpecsResponse: ::protobuf::SingularPtrField<FormatSpecsResponse>,
    unsupportedApiMessageResponse: ::protobuf::SingularPtrField<UnsupportedApiMessageResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APIMessage {}

impl APIMessage {
    pub fn new() -> APIMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APIMessage {
        static mut instance: ::protobuf::lazy::Lazy<APIMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APIMessage,
        };
        unsafe {
            instance.get(|| {
                APIMessage {
                    messageType: ::std::option::Option::None,
                    messageId: ::std::option::Option::None,
                    projectRootRequest: ::protobuf::SingularPtrField::none(),
                    projectRootResponse: ::protobuf::SingularPtrField::none(),
                    installationRootRequest: ::protobuf::SingularPtrField::none(),
                    installationRootResponse: ::protobuf::SingularPtrField::none(),
                    allStepsRequest: ::protobuf::SingularPtrField::none(),
                    allStepsResponse: ::protobuf::SingularPtrField::none(),
                    allSpecsRequest: ::protobuf::SingularPtrField::none(),
                    allSpecsResponse: ::protobuf::SingularPtrField::none(),
                    stepValueRequest: ::protobuf::SingularPtrField::none(),
                    stepValueResponse: ::protobuf::SingularPtrField::none(),
                    libPathRequest: ::protobuf::SingularPtrField::none(),
                    libPathResponse: ::protobuf::SingularPtrField::none(),
                    error: ::protobuf::SingularPtrField::none(),
                    allConceptsRequest: ::protobuf::SingularPtrField::none(),
                    allConceptsResponse: ::protobuf::SingularPtrField::none(),
                    performRefactoringRequest: ::protobuf::SingularPtrField::none(),
                    performRefactoringResponse: ::protobuf::SingularPtrField::none(),
                    extractConceptRequest: ::protobuf::SingularPtrField::none(),
                    extractConceptResponse: ::protobuf::SingularPtrField::none(),
                    formatSpecsRequest: ::protobuf::SingularPtrField::none(),
                    formatSpecsResponse: ::protobuf::SingularPtrField::none(),
                    unsupportedApiMessageResponse: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .gauge.messages.APIMessage.APIMessageType messageType = 1;

    pub fn clear_messageType(&mut self) {
        self.messageType = ::std::option::Option::None;
    }

    pub fn has_messageType(&self) -> bool {
        self.messageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_messageType(&mut self, v: APIMessage_APIMessageType) {
        self.messageType = ::std::option::Option::Some(v);
    }

    pub fn get_messageType(&self) -> APIMessage_APIMessageType {
        self.messageType.unwrap_or(APIMessage_APIMessageType::GetProjectRootRequest)
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

    pub fn get_messageId(&self) -> i64 {
        self.messageId.unwrap_or(0)
    }

    // optional .gauge.messages.GetProjectRootRequest projectRootRequest = 3;

    pub fn clear_projectRootRequest(&mut self) {
        self.projectRootRequest.clear();
    }

    pub fn has_projectRootRequest(&self) -> bool {
        self.projectRootRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_projectRootRequest(&mut self, v: GetProjectRootRequest) {
        self.projectRootRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_projectRootRequest(&mut self) -> &mut GetProjectRootRequest {
        if self.projectRootRequest.is_none() {
            self.projectRootRequest.set_default();
        };
        self.projectRootRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_projectRootRequest(&mut self) -> GetProjectRootRequest {
        self.projectRootRequest.take().unwrap_or_else(|| GetProjectRootRequest::new())
    }

    pub fn get_projectRootRequest(&self) -> &GetProjectRootRequest {
        self.projectRootRequest.as_ref().unwrap_or_else(|| GetProjectRootRequest::default_instance())
    }

    // optional .gauge.messages.GetProjectRootResponse projectRootResponse = 4;

    pub fn clear_projectRootResponse(&mut self) {
        self.projectRootResponse.clear();
    }

    pub fn has_projectRootResponse(&self) -> bool {
        self.projectRootResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_projectRootResponse(&mut self, v: GetProjectRootResponse) {
        self.projectRootResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_projectRootResponse(&mut self) -> &mut GetProjectRootResponse {
        if self.projectRootResponse.is_none() {
            self.projectRootResponse.set_default();
        };
        self.projectRootResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_projectRootResponse(&mut self) -> GetProjectRootResponse {
        self.projectRootResponse.take().unwrap_or_else(|| GetProjectRootResponse::new())
    }

    pub fn get_projectRootResponse(&self) -> &GetProjectRootResponse {
        self.projectRootResponse.as_ref().unwrap_or_else(|| GetProjectRootResponse::default_instance())
    }

    // optional .gauge.messages.GetInstallationRootRequest installationRootRequest = 5;

    pub fn clear_installationRootRequest(&mut self) {
        self.installationRootRequest.clear();
    }

    pub fn has_installationRootRequest(&self) -> bool {
        self.installationRootRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_installationRootRequest(&mut self, v: GetInstallationRootRequest) {
        self.installationRootRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_installationRootRequest(&mut self) -> &mut GetInstallationRootRequest {
        if self.installationRootRequest.is_none() {
            self.installationRootRequest.set_default();
        };
        self.installationRootRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_installationRootRequest(&mut self) -> GetInstallationRootRequest {
        self.installationRootRequest.take().unwrap_or_else(|| GetInstallationRootRequest::new())
    }

    pub fn get_installationRootRequest(&self) -> &GetInstallationRootRequest {
        self.installationRootRequest.as_ref().unwrap_or_else(|| GetInstallationRootRequest::default_instance())
    }

    // optional .gauge.messages.GetInstallationRootResponse installationRootResponse = 6;

    pub fn clear_installationRootResponse(&mut self) {
        self.installationRootResponse.clear();
    }

    pub fn has_installationRootResponse(&self) -> bool {
        self.installationRootResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_installationRootResponse(&mut self, v: GetInstallationRootResponse) {
        self.installationRootResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_installationRootResponse(&mut self) -> &mut GetInstallationRootResponse {
        if self.installationRootResponse.is_none() {
            self.installationRootResponse.set_default();
        };
        self.installationRootResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_installationRootResponse(&mut self) -> GetInstallationRootResponse {
        self.installationRootResponse.take().unwrap_or_else(|| GetInstallationRootResponse::new())
    }

    pub fn get_installationRootResponse(&self) -> &GetInstallationRootResponse {
        self.installationRootResponse.as_ref().unwrap_or_else(|| GetInstallationRootResponse::default_instance())
    }

    // optional .gauge.messages.GetAllStepsRequest allStepsRequest = 7;

    pub fn clear_allStepsRequest(&mut self) {
        self.allStepsRequest.clear();
    }

    pub fn has_allStepsRequest(&self) -> bool {
        self.allStepsRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allStepsRequest(&mut self, v: GetAllStepsRequest) {
        self.allStepsRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allStepsRequest(&mut self) -> &mut GetAllStepsRequest {
        if self.allStepsRequest.is_none() {
            self.allStepsRequest.set_default();
        };
        self.allStepsRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_allStepsRequest(&mut self) -> GetAllStepsRequest {
        self.allStepsRequest.take().unwrap_or_else(|| GetAllStepsRequest::new())
    }

    pub fn get_allStepsRequest(&self) -> &GetAllStepsRequest {
        self.allStepsRequest.as_ref().unwrap_or_else(|| GetAllStepsRequest::default_instance())
    }

    // optional .gauge.messages.GetAllStepsResponse allStepsResponse = 8;

    pub fn clear_allStepsResponse(&mut self) {
        self.allStepsResponse.clear();
    }

    pub fn has_allStepsResponse(&self) -> bool {
        self.allStepsResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allStepsResponse(&mut self, v: GetAllStepsResponse) {
        self.allStepsResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allStepsResponse(&mut self) -> &mut GetAllStepsResponse {
        if self.allStepsResponse.is_none() {
            self.allStepsResponse.set_default();
        };
        self.allStepsResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_allStepsResponse(&mut self) -> GetAllStepsResponse {
        self.allStepsResponse.take().unwrap_or_else(|| GetAllStepsResponse::new())
    }

    pub fn get_allStepsResponse(&self) -> &GetAllStepsResponse {
        self.allStepsResponse.as_ref().unwrap_or_else(|| GetAllStepsResponse::default_instance())
    }

    // optional .gauge.messages.GetAllSpecsRequest allSpecsRequest = 9;

    pub fn clear_allSpecsRequest(&mut self) {
        self.allSpecsRequest.clear();
    }

    pub fn has_allSpecsRequest(&self) -> bool {
        self.allSpecsRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allSpecsRequest(&mut self, v: GetAllSpecsRequest) {
        self.allSpecsRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allSpecsRequest(&mut self) -> &mut GetAllSpecsRequest {
        if self.allSpecsRequest.is_none() {
            self.allSpecsRequest.set_default();
        };
        self.allSpecsRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_allSpecsRequest(&mut self) -> GetAllSpecsRequest {
        self.allSpecsRequest.take().unwrap_or_else(|| GetAllSpecsRequest::new())
    }

    pub fn get_allSpecsRequest(&self) -> &GetAllSpecsRequest {
        self.allSpecsRequest.as_ref().unwrap_or_else(|| GetAllSpecsRequest::default_instance())
    }

    // optional .gauge.messages.GetAllSpecsResponse allSpecsResponse = 10;

    pub fn clear_allSpecsResponse(&mut self) {
        self.allSpecsResponse.clear();
    }

    pub fn has_allSpecsResponse(&self) -> bool {
        self.allSpecsResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allSpecsResponse(&mut self, v: GetAllSpecsResponse) {
        self.allSpecsResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allSpecsResponse(&mut self) -> &mut GetAllSpecsResponse {
        if self.allSpecsResponse.is_none() {
            self.allSpecsResponse.set_default();
        };
        self.allSpecsResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_allSpecsResponse(&mut self) -> GetAllSpecsResponse {
        self.allSpecsResponse.take().unwrap_or_else(|| GetAllSpecsResponse::new())
    }

    pub fn get_allSpecsResponse(&self) -> &GetAllSpecsResponse {
        self.allSpecsResponse.as_ref().unwrap_or_else(|| GetAllSpecsResponse::default_instance())
    }

    // optional .gauge.messages.GetStepValueRequest stepValueRequest = 11;

    pub fn clear_stepValueRequest(&mut self) {
        self.stepValueRequest.clear();
    }

    pub fn has_stepValueRequest(&self) -> bool {
        self.stepValueRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepValueRequest(&mut self, v: GetStepValueRequest) {
        self.stepValueRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepValueRequest(&mut self) -> &mut GetStepValueRequest {
        if self.stepValueRequest.is_none() {
            self.stepValueRequest.set_default();
        };
        self.stepValueRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepValueRequest(&mut self) -> GetStepValueRequest {
        self.stepValueRequest.take().unwrap_or_else(|| GetStepValueRequest::new())
    }

    pub fn get_stepValueRequest(&self) -> &GetStepValueRequest {
        self.stepValueRequest.as_ref().unwrap_or_else(|| GetStepValueRequest::default_instance())
    }

    // optional .gauge.messages.GetStepValueResponse stepValueResponse = 12;

    pub fn clear_stepValueResponse(&mut self) {
        self.stepValueResponse.clear();
    }

    pub fn has_stepValueResponse(&self) -> bool {
        self.stepValueResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stepValueResponse(&mut self, v: GetStepValueResponse) {
        self.stepValueResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stepValueResponse(&mut self) -> &mut GetStepValueResponse {
        if self.stepValueResponse.is_none() {
            self.stepValueResponse.set_default();
        };
        self.stepValueResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_stepValueResponse(&mut self) -> GetStepValueResponse {
        self.stepValueResponse.take().unwrap_or_else(|| GetStepValueResponse::new())
    }

    pub fn get_stepValueResponse(&self) -> &GetStepValueResponse {
        self.stepValueResponse.as_ref().unwrap_or_else(|| GetStepValueResponse::default_instance())
    }

    // optional .gauge.messages.GetLanguagePluginLibPathRequest libPathRequest = 13;

    pub fn clear_libPathRequest(&mut self) {
        self.libPathRequest.clear();
    }

    pub fn has_libPathRequest(&self) -> bool {
        self.libPathRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_libPathRequest(&mut self, v: GetLanguagePluginLibPathRequest) {
        self.libPathRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_libPathRequest(&mut self) -> &mut GetLanguagePluginLibPathRequest {
        if self.libPathRequest.is_none() {
            self.libPathRequest.set_default();
        };
        self.libPathRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_libPathRequest(&mut self) -> GetLanguagePluginLibPathRequest {
        self.libPathRequest.take().unwrap_or_else(|| GetLanguagePluginLibPathRequest::new())
    }

    pub fn get_libPathRequest(&self) -> &GetLanguagePluginLibPathRequest {
        self.libPathRequest.as_ref().unwrap_or_else(|| GetLanguagePluginLibPathRequest::default_instance())
    }

    // optional .gauge.messages.GetLanguagePluginLibPathResponse libPathResponse = 14;

    pub fn clear_libPathResponse(&mut self) {
        self.libPathResponse.clear();
    }

    pub fn has_libPathResponse(&self) -> bool {
        self.libPathResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_libPathResponse(&mut self, v: GetLanguagePluginLibPathResponse) {
        self.libPathResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_libPathResponse(&mut self) -> &mut GetLanguagePluginLibPathResponse {
        if self.libPathResponse.is_none() {
            self.libPathResponse.set_default();
        };
        self.libPathResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_libPathResponse(&mut self) -> GetLanguagePluginLibPathResponse {
        self.libPathResponse.take().unwrap_or_else(|| GetLanguagePluginLibPathResponse::new())
    }

    pub fn get_libPathResponse(&self) -> &GetLanguagePluginLibPathResponse {
        self.libPathResponse.as_ref().unwrap_or_else(|| GetLanguagePluginLibPathResponse::default_instance())
    }

    // optional .gauge.messages.ErrorResponse error = 15;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ErrorResponse) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ErrorResponse {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ErrorResponse {
        self.error.take().unwrap_or_else(|| ErrorResponse::new())
    }

    pub fn get_error(&self) -> &ErrorResponse {
        self.error.as_ref().unwrap_or_else(|| ErrorResponse::default_instance())
    }

    // optional .gauge.messages.GetAllConceptsRequest allConceptsRequest = 16;

    pub fn clear_allConceptsRequest(&mut self) {
        self.allConceptsRequest.clear();
    }

    pub fn has_allConceptsRequest(&self) -> bool {
        self.allConceptsRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allConceptsRequest(&mut self, v: GetAllConceptsRequest) {
        self.allConceptsRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allConceptsRequest(&mut self) -> &mut GetAllConceptsRequest {
        if self.allConceptsRequest.is_none() {
            self.allConceptsRequest.set_default();
        };
        self.allConceptsRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_allConceptsRequest(&mut self) -> GetAllConceptsRequest {
        self.allConceptsRequest.take().unwrap_or_else(|| GetAllConceptsRequest::new())
    }

    pub fn get_allConceptsRequest(&self) -> &GetAllConceptsRequest {
        self.allConceptsRequest.as_ref().unwrap_or_else(|| GetAllConceptsRequest::default_instance())
    }

    // optional .gauge.messages.GetAllConceptsResponse allConceptsResponse = 17;

    pub fn clear_allConceptsResponse(&mut self) {
        self.allConceptsResponse.clear();
    }

    pub fn has_allConceptsResponse(&self) -> bool {
        self.allConceptsResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allConceptsResponse(&mut self, v: GetAllConceptsResponse) {
        self.allConceptsResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allConceptsResponse(&mut self) -> &mut GetAllConceptsResponse {
        if self.allConceptsResponse.is_none() {
            self.allConceptsResponse.set_default();
        };
        self.allConceptsResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_allConceptsResponse(&mut self) -> GetAllConceptsResponse {
        self.allConceptsResponse.take().unwrap_or_else(|| GetAllConceptsResponse::new())
    }

    pub fn get_allConceptsResponse(&self) -> &GetAllConceptsResponse {
        self.allConceptsResponse.as_ref().unwrap_or_else(|| GetAllConceptsResponse::default_instance())
    }

    // optional .gauge.messages.PerformRefactoringRequest performRefactoringRequest = 18;

    pub fn clear_performRefactoringRequest(&mut self) {
        self.performRefactoringRequest.clear();
    }

    pub fn has_performRefactoringRequest(&self) -> bool {
        self.performRefactoringRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_performRefactoringRequest(&mut self, v: PerformRefactoringRequest) {
        self.performRefactoringRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_performRefactoringRequest(&mut self) -> &mut PerformRefactoringRequest {
        if self.performRefactoringRequest.is_none() {
            self.performRefactoringRequest.set_default();
        };
        self.performRefactoringRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_performRefactoringRequest(&mut self) -> PerformRefactoringRequest {
        self.performRefactoringRequest.take().unwrap_or_else(|| PerformRefactoringRequest::new())
    }

    pub fn get_performRefactoringRequest(&self) -> &PerformRefactoringRequest {
        self.performRefactoringRequest.as_ref().unwrap_or_else(|| PerformRefactoringRequest::default_instance())
    }

    // optional .gauge.messages.PerformRefactoringResponse performRefactoringResponse = 19;

    pub fn clear_performRefactoringResponse(&mut self) {
        self.performRefactoringResponse.clear();
    }

    pub fn has_performRefactoringResponse(&self) -> bool {
        self.performRefactoringResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_performRefactoringResponse(&mut self, v: PerformRefactoringResponse) {
        self.performRefactoringResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_performRefactoringResponse(&mut self) -> &mut PerformRefactoringResponse {
        if self.performRefactoringResponse.is_none() {
            self.performRefactoringResponse.set_default();
        };
        self.performRefactoringResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_performRefactoringResponse(&mut self) -> PerformRefactoringResponse {
        self.performRefactoringResponse.take().unwrap_or_else(|| PerformRefactoringResponse::new())
    }

    pub fn get_performRefactoringResponse(&self) -> &PerformRefactoringResponse {
        self.performRefactoringResponse.as_ref().unwrap_or_else(|| PerformRefactoringResponse::default_instance())
    }

    // optional .gauge.messages.ExtractConceptRequest extractConceptRequest = 20;

    pub fn clear_extractConceptRequest(&mut self) {
        self.extractConceptRequest.clear();
    }

    pub fn has_extractConceptRequest(&self) -> bool {
        self.extractConceptRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extractConceptRequest(&mut self, v: ExtractConceptRequest) {
        self.extractConceptRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_extractConceptRequest(&mut self) -> &mut ExtractConceptRequest {
        if self.extractConceptRequest.is_none() {
            self.extractConceptRequest.set_default();
        };
        self.extractConceptRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_extractConceptRequest(&mut self) -> ExtractConceptRequest {
        self.extractConceptRequest.take().unwrap_or_else(|| ExtractConceptRequest::new())
    }

    pub fn get_extractConceptRequest(&self) -> &ExtractConceptRequest {
        self.extractConceptRequest.as_ref().unwrap_or_else(|| ExtractConceptRequest::default_instance())
    }

    // optional .gauge.messages.ExtractConceptResponse extractConceptResponse = 21;

    pub fn clear_extractConceptResponse(&mut self) {
        self.extractConceptResponse.clear();
    }

    pub fn has_extractConceptResponse(&self) -> bool {
        self.extractConceptResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extractConceptResponse(&mut self, v: ExtractConceptResponse) {
        self.extractConceptResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_extractConceptResponse(&mut self) -> &mut ExtractConceptResponse {
        if self.extractConceptResponse.is_none() {
            self.extractConceptResponse.set_default();
        };
        self.extractConceptResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_extractConceptResponse(&mut self) -> ExtractConceptResponse {
        self.extractConceptResponse.take().unwrap_or_else(|| ExtractConceptResponse::new())
    }

    pub fn get_extractConceptResponse(&self) -> &ExtractConceptResponse {
        self.extractConceptResponse.as_ref().unwrap_or_else(|| ExtractConceptResponse::default_instance())
    }

    // optional .gauge.messages.FormatSpecsRequest formatSpecsRequest = 22;

    pub fn clear_formatSpecsRequest(&mut self) {
        self.formatSpecsRequest.clear();
    }

    pub fn has_formatSpecsRequest(&self) -> bool {
        self.formatSpecsRequest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_formatSpecsRequest(&mut self, v: FormatSpecsRequest) {
        self.formatSpecsRequest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_formatSpecsRequest(&mut self) -> &mut FormatSpecsRequest {
        if self.formatSpecsRequest.is_none() {
            self.formatSpecsRequest.set_default();
        };
        self.formatSpecsRequest.as_mut().unwrap()
    }

    // Take field
    pub fn take_formatSpecsRequest(&mut self) -> FormatSpecsRequest {
        self.formatSpecsRequest.take().unwrap_or_else(|| FormatSpecsRequest::new())
    }

    pub fn get_formatSpecsRequest(&self) -> &FormatSpecsRequest {
        self.formatSpecsRequest.as_ref().unwrap_or_else(|| FormatSpecsRequest::default_instance())
    }

    // optional .gauge.messages.FormatSpecsResponse formatSpecsResponse = 23;

    pub fn clear_formatSpecsResponse(&mut self) {
        self.formatSpecsResponse.clear();
    }

    pub fn has_formatSpecsResponse(&self) -> bool {
        self.formatSpecsResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_formatSpecsResponse(&mut self, v: FormatSpecsResponse) {
        self.formatSpecsResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_formatSpecsResponse(&mut self) -> &mut FormatSpecsResponse {
        if self.formatSpecsResponse.is_none() {
            self.formatSpecsResponse.set_default();
        };
        self.formatSpecsResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_formatSpecsResponse(&mut self) -> FormatSpecsResponse {
        self.formatSpecsResponse.take().unwrap_or_else(|| FormatSpecsResponse::new())
    }

    pub fn get_formatSpecsResponse(&self) -> &FormatSpecsResponse {
        self.formatSpecsResponse.as_ref().unwrap_or_else(|| FormatSpecsResponse::default_instance())
    }

    // optional .gauge.messages.UnsupportedApiMessageResponse unsupportedApiMessageResponse = 24;

    pub fn clear_unsupportedApiMessageResponse(&mut self) {
        self.unsupportedApiMessageResponse.clear();
    }

    pub fn has_unsupportedApiMessageResponse(&self) -> bool {
        self.unsupportedApiMessageResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unsupportedApiMessageResponse(&mut self, v: UnsupportedApiMessageResponse) {
        self.unsupportedApiMessageResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unsupportedApiMessageResponse(&mut self) -> &mut UnsupportedApiMessageResponse {
        if self.unsupportedApiMessageResponse.is_none() {
            self.unsupportedApiMessageResponse.set_default();
        };
        self.unsupportedApiMessageResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_unsupportedApiMessageResponse(&mut self) -> UnsupportedApiMessageResponse {
        self.unsupportedApiMessageResponse.take().unwrap_or_else(|| UnsupportedApiMessageResponse::new())
    }

    pub fn get_unsupportedApiMessageResponse(&self) -> &UnsupportedApiMessageResponse {
        self.unsupportedApiMessageResponse.as_ref().unwrap_or_else(|| UnsupportedApiMessageResponse::default_instance())
    }
}

impl ::protobuf::Message for APIMessage {
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
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.messageType = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.messageId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.projectRootRequest));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.projectRootResponse));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.installationRootRequest));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.installationRootResponse));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allStepsRequest));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allStepsResponse));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allSpecsRequest));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allSpecsResponse));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepValueRequest));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stepValueResponse));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.libPathRequest));
                },
                14 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.libPathResponse));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                16 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allConceptsRequest));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allConceptsResponse));
                },
                18 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.performRefactoringRequest));
                },
                19 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.performRefactoringResponse));
                },
                20 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.extractConceptRequest));
                },
                21 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.extractConceptResponse));
                },
                22 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.formatSpecsRequest));
                },
                23 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.formatSpecsResponse));
                },
                24 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unsupportedApiMessageResponse));
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
        for value in &self.messageType {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.messageId {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.projectRootRequest {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.projectRootResponse {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.installationRootRequest {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.installationRootResponse {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.allStepsRequest {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.allStepsResponse {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.allSpecsRequest {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.allSpecsResponse {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.stepValueRequest {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.stepValueResponse {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.libPathRequest {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.libPathResponse {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.error {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.allConceptsRequest {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.allConceptsResponse {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.performRefactoringRequest {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.performRefactoringResponse {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.extractConceptRequest {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.extractConceptResponse {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.formatSpecsRequest {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.formatSpecsResponse {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.unsupportedApiMessageResponse {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.messageType {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.messageId {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.projectRootRequest.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.projectRootResponse.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.installationRootRequest.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.installationRootResponse.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.allStepsRequest.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.allStepsResponse.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.allSpecsRequest.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.allSpecsResponse.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepValueRequest.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stepValueResponse.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.libPathRequest.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.libPathResponse.as_ref() {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.allConceptsRequest.as_ref() {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.allConceptsResponse.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.performRefactoringRequest.as_ref() {
            try!(os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.performRefactoringResponse.as_ref() {
            try!(os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.extractConceptRequest.as_ref() {
            try!(os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.extractConceptResponse.as_ref() {
            try!(os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.formatSpecsRequest.as_ref() {
            try!(os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.formatSpecsResponse.as_ref() {
            try!(os.write_tag(23, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.unsupportedApiMessageResponse.as_ref() {
            try!(os.write_tag(24, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<APIMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for APIMessage {
    fn new() -> APIMessage {
        APIMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<APIMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "messageType",
                    APIMessage::has_messageType,
                    APIMessage::get_messageType,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "messageId",
                    APIMessage::has_messageId,
                    APIMessage::get_messageId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "projectRootRequest",
                    APIMessage::has_projectRootRequest,
                    APIMessage::get_projectRootRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "projectRootResponse",
                    APIMessage::has_projectRootResponse,
                    APIMessage::get_projectRootResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "installationRootRequest",
                    APIMessage::has_installationRootRequest,
                    APIMessage::get_installationRootRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "installationRootResponse",
                    APIMessage::has_installationRootResponse,
                    APIMessage::get_installationRootResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "allStepsRequest",
                    APIMessage::has_allStepsRequest,
                    APIMessage::get_allStepsRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "allStepsResponse",
                    APIMessage::has_allStepsResponse,
                    APIMessage::get_allStepsResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "allSpecsRequest",
                    APIMessage::has_allSpecsRequest,
                    APIMessage::get_allSpecsRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "allSpecsResponse",
                    APIMessage::has_allSpecsResponse,
                    APIMessage::get_allSpecsResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepValueRequest",
                    APIMessage::has_stepValueRequest,
                    APIMessage::get_stepValueRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stepValueResponse",
                    APIMessage::has_stepValueResponse,
                    APIMessage::get_stepValueResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "libPathRequest",
                    APIMessage::has_libPathRequest,
                    APIMessage::get_libPathRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "libPathResponse",
                    APIMessage::has_libPathResponse,
                    APIMessage::get_libPathResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    APIMessage::has_error,
                    APIMessage::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "allConceptsRequest",
                    APIMessage::has_allConceptsRequest,
                    APIMessage::get_allConceptsRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "allConceptsResponse",
                    APIMessage::has_allConceptsResponse,
                    APIMessage::get_allConceptsResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "performRefactoringRequest",
                    APIMessage::has_performRefactoringRequest,
                    APIMessage::get_performRefactoringRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "performRefactoringResponse",
                    APIMessage::has_performRefactoringResponse,
                    APIMessage::get_performRefactoringResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "extractConceptRequest",
                    APIMessage::has_extractConceptRequest,
                    APIMessage::get_extractConceptRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "extractConceptResponse",
                    APIMessage::has_extractConceptResponse,
                    APIMessage::get_extractConceptResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "formatSpecsRequest",
                    APIMessage::has_formatSpecsRequest,
                    APIMessage::get_formatSpecsRequest,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "formatSpecsResponse",
                    APIMessage::has_formatSpecsResponse,
                    APIMessage::get_formatSpecsResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "unsupportedApiMessageResponse",
                    APIMessage::has_unsupportedApiMessageResponse,
                    APIMessage::get_unsupportedApiMessageResponse,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APIMessage>(
                    "APIMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APIMessage {
    fn clear(&mut self) {
        self.clear_messageType();
        self.clear_messageId();
        self.clear_projectRootRequest();
        self.clear_projectRootResponse();
        self.clear_installationRootRequest();
        self.clear_installationRootResponse();
        self.clear_allStepsRequest();
        self.clear_allStepsResponse();
        self.clear_allSpecsRequest();
        self.clear_allSpecsResponse();
        self.clear_stepValueRequest();
        self.clear_stepValueResponse();
        self.clear_libPathRequest();
        self.clear_libPathResponse();
        self.clear_error();
        self.clear_allConceptsRequest();
        self.clear_allConceptsResponse();
        self.clear_performRefactoringRequest();
        self.clear_performRefactoringResponse();
        self.clear_extractConceptRequest();
        self.clear_extractConceptResponse();
        self.clear_formatSpecsRequest();
        self.clear_formatSpecsResponse();
        self.clear_unsupportedApiMessageResponse();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for APIMessage {
    fn eq(&self, other: &APIMessage) -> bool {
        self.messageType == other.messageType &&
        self.messageId == other.messageId &&
        self.projectRootRequest == other.projectRootRequest &&
        self.projectRootResponse == other.projectRootResponse &&
        self.installationRootRequest == other.installationRootRequest &&
        self.installationRootResponse == other.installationRootResponse &&
        self.allStepsRequest == other.allStepsRequest &&
        self.allStepsResponse == other.allStepsResponse &&
        self.allSpecsRequest == other.allSpecsRequest &&
        self.allSpecsResponse == other.allSpecsResponse &&
        self.stepValueRequest == other.stepValueRequest &&
        self.stepValueResponse == other.stepValueResponse &&
        self.libPathRequest == other.libPathRequest &&
        self.libPathResponse == other.libPathResponse &&
        self.error == other.error &&
        self.allConceptsRequest == other.allConceptsRequest &&
        self.allConceptsResponse == other.allConceptsResponse &&
        self.performRefactoringRequest == other.performRefactoringRequest &&
        self.performRefactoringResponse == other.performRefactoringResponse &&
        self.extractConceptRequest == other.extractConceptRequest &&
        self.extractConceptResponse == other.extractConceptResponse &&
        self.formatSpecsRequest == other.formatSpecsRequest &&
        self.formatSpecsResponse == other.formatSpecsResponse &&
        self.unsupportedApiMessageResponse == other.unsupportedApiMessageResponse &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for APIMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum APIMessage_APIMessageType {
    GetProjectRootRequest = 1,
    GetProjectRootResponse = 2,
    GetInstallationRootRequest = 3,
    GetInstallationRootResponse = 4,
    GetAllStepsRequest = 5,
    GetAllStepResponse = 6,
    GetAllSpecsRequest = 7,
    GetAllSpecsResponse = 8,
    GetStepValueRequest = 9,
    GetStepValueResponse = 10,
    GetLanguagePluginLibPathRequest = 11,
    GetLanguagePluginLibPathResponse = 12,
    ErrorResponse = 13,
    GetAllConceptsRequest = 14,
    GetAllConceptsResponse = 15,
    PerformRefactoringRequest = 16,
    PerformRefactoringResponse = 17,
    ExtractConceptRequest = 18,
    ExtractConceptResponse = 19,
    FormatSpecsRequest = 20,
    FormatSpecsResponse = 21,
    UnsupportedApiMessageResponse = 22,
}

impl ::protobuf::ProtobufEnum for APIMessage_APIMessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<APIMessage_APIMessageType> {
        match value {
            1 => ::std::option::Option::Some(APIMessage_APIMessageType::GetProjectRootRequest),
            2 => ::std::option::Option::Some(APIMessage_APIMessageType::GetProjectRootResponse),
            3 => ::std::option::Option::Some(APIMessage_APIMessageType::GetInstallationRootRequest),
            4 => ::std::option::Option::Some(APIMessage_APIMessageType::GetInstallationRootResponse),
            5 => ::std::option::Option::Some(APIMessage_APIMessageType::GetAllStepsRequest),
            6 => ::std::option::Option::Some(APIMessage_APIMessageType::GetAllStepResponse),
            7 => ::std::option::Option::Some(APIMessage_APIMessageType::GetAllSpecsRequest),
            8 => ::std::option::Option::Some(APIMessage_APIMessageType::GetAllSpecsResponse),
            9 => ::std::option::Option::Some(APIMessage_APIMessageType::GetStepValueRequest),
            10 => ::std::option::Option::Some(APIMessage_APIMessageType::GetStepValueResponse),
            11 => ::std::option::Option::Some(APIMessage_APIMessageType::GetLanguagePluginLibPathRequest),
            12 => ::std::option::Option::Some(APIMessage_APIMessageType::GetLanguagePluginLibPathResponse),
            13 => ::std::option::Option::Some(APIMessage_APIMessageType::ErrorResponse),
            14 => ::std::option::Option::Some(APIMessage_APIMessageType::GetAllConceptsRequest),
            15 => ::std::option::Option::Some(APIMessage_APIMessageType::GetAllConceptsResponse),
            16 => ::std::option::Option::Some(APIMessage_APIMessageType::PerformRefactoringRequest),
            17 => ::std::option::Option::Some(APIMessage_APIMessageType::PerformRefactoringResponse),
            18 => ::std::option::Option::Some(APIMessage_APIMessageType::ExtractConceptRequest),
            19 => ::std::option::Option::Some(APIMessage_APIMessageType::ExtractConceptResponse),
            20 => ::std::option::Option::Some(APIMessage_APIMessageType::FormatSpecsRequest),
            21 => ::std::option::Option::Some(APIMessage_APIMessageType::FormatSpecsResponse),
            22 => ::std::option::Option::Some(APIMessage_APIMessageType::UnsupportedApiMessageResponse),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [APIMessage_APIMessageType] = &[
            APIMessage_APIMessageType::GetProjectRootRequest,
            APIMessage_APIMessageType::GetProjectRootResponse,
            APIMessage_APIMessageType::GetInstallationRootRequest,
            APIMessage_APIMessageType::GetInstallationRootResponse,
            APIMessage_APIMessageType::GetAllStepsRequest,
            APIMessage_APIMessageType::GetAllStepResponse,
            APIMessage_APIMessageType::GetAllSpecsRequest,
            APIMessage_APIMessageType::GetAllSpecsResponse,
            APIMessage_APIMessageType::GetStepValueRequest,
            APIMessage_APIMessageType::GetStepValueResponse,
            APIMessage_APIMessageType::GetLanguagePluginLibPathRequest,
            APIMessage_APIMessageType::GetLanguagePluginLibPathResponse,
            APIMessage_APIMessageType::ErrorResponse,
            APIMessage_APIMessageType::GetAllConceptsRequest,
            APIMessage_APIMessageType::GetAllConceptsResponse,
            APIMessage_APIMessageType::PerformRefactoringRequest,
            APIMessage_APIMessageType::PerformRefactoringResponse,
            APIMessage_APIMessageType::ExtractConceptRequest,
            APIMessage_APIMessageType::ExtractConceptResponse,
            APIMessage_APIMessageType::FormatSpecsRequest,
            APIMessage_APIMessageType::FormatSpecsResponse,
            APIMessage_APIMessageType::UnsupportedApiMessageResponse,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<APIMessage_APIMessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("APIMessage_APIMessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for APIMessage_APIMessageType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x61, 0x70, 0x69, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x67, 0x61, 0x75,
    0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x1a, 0x0a, 0x73, 0x70, 0x65,
    0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x17, 0x0a, 0x15, 0x47, 0x65, 0x74, 0x50, 0x72,
    0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x22, 0x2d, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x6f,
    0x6f, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x13, 0x0a, 0x0b, 0x70, 0x72,
    0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x6f, 0x6f, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22,
    0x1c, 0x0a, 0x1a, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x37, 0x0a,
    0x1b, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x10,
    0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f, 0x74,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x14, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c,
    0x53, 0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x47, 0x0a, 0x13,
    0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x30, 0x0a, 0x08, 0x61, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70, 0x73, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x53, 0x74, 0x65, 0x70,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x14, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53,
    0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x3f, 0x0a, 0x13, 0x47,
    0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x28, 0x0a, 0x05, 0x73, 0x70, 0x65, 0x63, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x19, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x53, 0x70, 0x65, 0x63, 0x22, 0x17, 0x0a, 0x15,
    0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x47, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x43,
    0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x2d, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1b, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x66,
    0x0a, 0x0b, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x31, 0x0a,
    0x09, 0x73, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x12, 0x10, 0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x70, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x12, 0x0a, 0x0a, 0x6c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x05, 0x22, 0x3f, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x53, 0x74, 0x65,
    0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a,
    0x08, 0x73, 0x74, 0x65, 0x70, 0x54, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x16, 0x0a, 0x0e, 0x68, 0x61, 0x73, 0x49, 0x6e, 0x6c, 0x69, 0x6e, 0x65, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x22, 0x49, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x53, 0x74,
    0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x31, 0x0a, 0x09, 0x73, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x22, 0x33, 0x0a, 0x1f, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67,
    0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x30, 0x0a, 0x20, 0x47, 0x65, 0x74, 0x4c, 0x61,
    0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50,
    0x61, 0x74, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x70,
    0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x1e, 0x0a, 0x0d, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x3d, 0x0a, 0x19, 0x50, 0x65, 0x72,
    0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0f, 0x0a, 0x07, 0x6f, 0x6c, 0x64, 0x53, 0x74, 0x65,
    0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x6e, 0x65, 0x77, 0x53, 0x74,
    0x65, 0x70, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x53, 0x0a, 0x1a, 0x50, 0x65, 0x72, 0x66,
    0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0e, 0x0a, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x66, 0x69, 0x6c, 0x65, 0x73,
    0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x22, 0x29, 0x0a,
    0x19, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x49,
    0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65,
    0x78, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0xd1, 0x01, 0x0a, 0x15, 0x45, 0x78, 0x74,
    0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x29, 0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x4e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x73, 0x74, 0x65, 0x70, 0x12, 0x23, 0x0a,
    0x05, 0x73, 0x74, 0x65, 0x70, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x73, 0x74,
    0x65, 0x70, 0x12, 0x1b, 0x0a, 0x13, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x41, 0x63, 0x72, 0x6f,
    0x73, 0x73, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x12,
    0x17, 0x0a, 0x0f, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x46, 0x69, 0x6c, 0x65, 0x4e, 0x61,
    0x6d, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x32, 0x0a, 0x10, 0x73, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x65, 0x64, 0x54, 0x65, 0x78, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x18, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x47, 0x0a, 0x08,
    0x74, 0x65, 0x78, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65,
    0x4e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x6f, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x65, 0x6e, 0x64, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x6f, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x05, 0x22, 0x3b, 0x0a, 0x04, 0x73, 0x74, 0x65, 0x70, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x16, 0x0a, 0x0e, 0x70, 0x61,
    0x72, 0x61, 0x6d, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x22, 0x50, 0x0a, 0x16, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e,
    0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x11, 0x0a, 0x09,
    0x69, 0x73, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12,
    0x0d, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x14,
    0x0a, 0x0c, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x09, 0x22, 0x23, 0x0a, 0x12, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x53, 0x70,
    0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x70,
    0x65, 0x63, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x22, 0x37, 0x0a, 0x13, 0x46, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x0e, 0x0a, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09,
    0x12, 0x10, 0x0a, 0x08, 0x77, 0x61, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x09, 0x22, 0x1f, 0x0a, 0x1d, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65,
    0x64, 0x41, 0x70, 0x69, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x22, 0xc7, 0x11, 0x0a, 0x0a, 0x41, 0x50, 0x49, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x3e, 0x0a, 0x0b, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x29, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x41, 0x50, 0x49, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x2e, 0x41, 0x50, 0x49, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x49, 0x64, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x41, 0x0a, 0x12, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74,
    0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x25, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x6f, 0x6f,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x43, 0x0a, 0x13, 0x70, 0x72, 0x6f, 0x6a,
    0x65, 0x63, 0x74, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63,
    0x74, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4b, 0x0a,
    0x17, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a,
    0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x47, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52,
    0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x4d, 0x0a, 0x18, 0x69, 0x6e,
    0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65,
    0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3b, 0x0a, 0x0f, 0x61, 0x6c, 0x6c,
    0x53, 0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x22, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3d, 0x0a, 0x10, 0x61, 0x6c, 0x6c, 0x53, 0x74, 0x65,
    0x70, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x23, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x2e, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3b, 0x0a, 0x0f, 0x61, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22,
    0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x3d, 0x0a, 0x10, 0x61, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65,
    0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x3d, 0x0a, 0x10, 0x73, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74,
    0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x3f, 0x0a, 0x11, 0x73, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74,
    0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x47, 0x0a, 0x0e, 0x6c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x67, 0x61, 0x75, 0x67,
    0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x4c, 0x61,
    0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50,
    0x61, 0x74, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x49, 0x0a, 0x0f, 0x6c, 0x69,
    0x62, 0x50, 0x61, 0x74, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65,
    0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2c, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x0f,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x41, 0x0a, 0x12, 0x61, 0x6c, 0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70,
    0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x25, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x2e, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x43, 0x0a, 0x13, 0x61, 0x6c, 0x6c, 0x43, 0x6f, 0x6e,
    0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x11, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65,
    0x70, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4c, 0x0a, 0x19, 0x70,
    0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e,
    0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29,
    0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x50, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x4e, 0x0a, 0x1a, 0x70, 0x65, 0x72,
    0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e,
    0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x50,
    0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e,
    0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x44, 0x0a, 0x15, 0x65, 0x78, 0x74,
    0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65,
    0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63,
    0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x46, 0x0a, 0x16, 0x65, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x26, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x2e, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3e, 0x0a, 0x12, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x16, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x53, 0x70, 0x65, 0x63, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x40, 0x0a, 0x13, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x17,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x53, 0x70, 0x65, 0x63,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x54, 0x0a, 0x1d, 0x75, 0x6e, 0x73,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x41, 0x70, 0x69, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x18, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2d, 0x2e, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x2e, 0x55, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x41, 0x70, 0x69,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0xfb, 0x04, 0x0a, 0x0e, 0x41, 0x50, 0x49, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x19, 0x0a, 0x15, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74,
    0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x01, 0x12, 0x1a, 0x0a,
    0x16, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x6f, 0x6f, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x02, 0x12, 0x1e, 0x0a, 0x1a, 0x47, 0x65, 0x74,
    0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x03, 0x12, 0x1f, 0x0a, 0x1b, 0x47, 0x65, 0x74,
    0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x04, 0x12, 0x16, 0x0a, 0x12, 0x47, 0x65,
    0x74, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x10, 0x05, 0x12, 0x16, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x06, 0x12, 0x16, 0x0a, 0x12, 0x47, 0x65,
    0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x10, 0x07, 0x12, 0x17, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x08, 0x12, 0x17, 0x0a, 0x13, 0x47,
    0x65, 0x74, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x10, 0x09, 0x12, 0x18, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x53, 0x74, 0x65, 0x70, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x0a, 0x12, 0x23,
    0x0a, 0x1f, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x50, 0x6c, 0x75,
    0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x10, 0x0b, 0x12, 0x24, 0x0a, 0x20, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61,
    0x67, 0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x0c, 0x12, 0x11, 0x0a, 0x0d, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x0d, 0x12, 0x19, 0x0a, 0x15,
    0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x0e, 0x12, 0x1a, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x41, 0x6c,
    0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x10, 0x0f, 0x12, 0x1d, 0x0a, 0x19, 0x50, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65,
    0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x10, 0x10, 0x12, 0x1e, 0x0a, 0x1a, 0x50, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66,
    0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x10, 0x11, 0x12, 0x19, 0x0a, 0x15, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e,
    0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x12, 0x12, 0x1a, 0x0a,
    0x16, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x13, 0x12, 0x16, 0x0a, 0x12, 0x46, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10,
    0x14, 0x12, 0x17, 0x0a, 0x13, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x53, 0x70, 0x65, 0x63, 0x73,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x15, 0x12, 0x21, 0x0a, 0x1d, 0x55, 0x6e,
    0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x41, 0x70, 0x69, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x10, 0x16, 0x4a, 0xc8, 0x56,
    0x0a, 0x07, 0x12, 0x05, 0x16, 0x00, 0x98, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x17, 0x08, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x18, 0x07, 0x13, 0x0a, 0x3f,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1b, 0x00, 0x1c, 0x01, 0x1a, 0x33, 0x2f, 0x20, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x52, 0x6f, 0x6f, 0x74, 0x20, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x1d, 0x0a, 0x31, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x1f, 0x00, 0x22, 0x01, 0x1a, 0x25, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63,
    0x74, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x1e, 0x0a, 0x46, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x22, 0x1a, 0x39, 0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x20, 0x70, 0x61,
    0x74, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63,
    0x74, 0x20, 0x52, 0x6f, 0x6f, 0x74, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x20, 0x21, 0x0a, 0x4a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x25, 0x00, 0x26, 0x01, 0x1a, 0x3e, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x6f, 0x6f,
    0x74, 0x20, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x47, 0x61, 0x75, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x25,
    0x08, 0x22, 0x0a, 0x35, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x29, 0x00, 0x2c, 0x01, 0x1a, 0x29,
    0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x47, 0x65,
    0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x29, 0x08, 0x23, 0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x2b,
    0x02, 0x27, 0x1a, 0x3e, 0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x61, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x47, 0x61, 0x75, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61,
    0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x25, 0x26, 0x0a, 0x36, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x2f, 0x00, 0x30, 0x01, 0x1a, 0x2a, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x53, 0x74, 0x65,
    0x70, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63,
    0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x1a, 0x0a, 0x2d,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x33, 0x00, 0x36, 0x01, 0x1a, 0x21, 0x2f, 0x20, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c,
    0x53, 0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x33, 0x08, 0x1b, 0x0a, 0x4c, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x35, 0x02, 0x36, 0x1a, 0x3f, 0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73, 0x20,
    0x61, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20,
    0x53, 0x74, 0x65, 0x70, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x72, 0x65, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72,
    0x6f, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x35, 0x0b, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x29,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x34, 0x35, 0x0a,
    0x36, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x39, 0x00, 0x3a, 0x01, 0x1a, 0x2a, 0x2f, 0x20, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x61, 0x6c,
    0x6c, 0x20, 0x53, 0x70, 0x65, 0x63, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70,
    0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x39, 0x08, 0x1a, 0x0a, 0x2d, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3d, 0x00, 0x40, 0x01, 0x1a,
    0x21, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x47,
    0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x1b, 0x0a, 0x4c,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x2e, 0x1a, 0x3f, 0x2f, 0x20, 0x48,
    0x6f, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x70, 0x65, 0x63, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x3f, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x3f, 0x24, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x3f, 0x2c, 0x2d, 0x0a, 0x39, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x43, 0x00, 0x44, 0x01,
    0x1a, 0x2d, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x67,
    0x65, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x43, 0x08, 0x1d, 0x0a, 0x31, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x04, 0x47, 0x00, 0x4a, 0x01, 0x1a, 0x25, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x43, 0x6f, 0x6e,
    0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x47, 0x08, 0x1e, 0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x00, 0x12, 0x03, 0x49, 0x02, 0x24, 0x1a, 0x42, 0x2f, 0x20, 0x48, 0x6f, 0x6c, 0x64, 0x73,
    0x20, 0x61, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61,
    0x72, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x49, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x49, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x49, 0x22, 0x23, 0x0a, 0x23, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x4d, 0x00, 0x54, 0x01, 0x1a,
    0x17, 0x2f, 0x20, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20,
    0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12,
    0x03, 0x4d, 0x08, 0x13, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x02,
    0x37, 0x1a, 0x22, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e,
    0x63, 0x65, 0x70, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4f, 0x0b,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x29, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x35, 0x36, 0x0a, 0x47, 0x0a,
    0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x51, 0x02, 0x1f, 0x1a, 0x3a, 0x2f, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x61, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43, 0x6f,
    0x6e, 0x63, 0x65, 0x70, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x51, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x51,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x51, 0x12, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x51, 0x1d, 0x1e, 0x0a, 0x49,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x53, 0x02, 0x20, 0x1a, 0x3c, 0x2f, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x53, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x53, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12, 0x03, 0x53, 0x1e,
    0x1f, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x57, 0x00, 0x5c, 0x01, 0x1a, 0x1f, 0x2f,
    0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20,
    0x61, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x57, 0x08, 0x1b, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x00, 0x12, 0x03, 0x59, 0x02, 0x1f, 0x1a, 0x18, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74,
    0x65, 0x78, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x74, 0x65, 0x70, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x59, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x59, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x59, 0x1d, 0x1e, 0x0a, 0x41, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x01, 0x12, 0x03, 0x5b, 0x02, 0x23, 0x1a, 0x34, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x74,
    0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e,
    0x6c, 0x69, 0x6e, 0x65, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x5b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x5b, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x5b, 0x21, 0x22, 0x0a, 0x2e, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x5f, 0x00, 0x62, 0x01,
    0x1a, 0x22, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x47, 0x65, 0x74, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x1c,
    0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x61, 0x02, 0x37, 0x1a, 0x32, 0x2f,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x53, 0x74, 0x65, 0x70, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x61, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x03, 0x61, 0x0b, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x29, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x61, 0x35, 0x36, 0x0a, 0x4d, 0x0a, 0x02, 0x04, 0x0d, 0x12,
    0x04, 0x65, 0x00, 0x68, 0x01, 0x1a, 0x41, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65,
    0x20, 0x70, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x27, 0x73, 0x20, 0x4c, 0x69, 0x62, 0x20, 0x64, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12,
    0x03, 0x65, 0x08, 0x27, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x67, 0x02,
    0x1f, 0x1a, 0x30, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6c, 0x69, 0x62, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x66, 0x6f,
    0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x67, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x67, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x12, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x67, 0x1d, 0x1e, 0x0a, 0x3a, 0x0a, 0x02, 0x04,
    0x0e, 0x12, 0x04, 0x6b, 0x00, 0x6e, 0x01, 0x1a, 0x2e, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61,
    0x67, 0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03,
    0x6b, 0x08, 0x28, 0x0a, 0x43, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x02, 0x1b,
    0x1a, 0x36, 0x2f, 0x20, 0x41, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x20, 0x70, 0x61, 0x74,
    0x68, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x62, 0x20, 0x64, 0x69, 0x72,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61,
    0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x6d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x6d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6d, 0x19, 0x1a,
    0x0a, 0x29, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x71, 0x00, 0x74, 0x01, 0x1a, 0x1d, 0x2f, 0x20,
    0x41, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72,
    0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0f, 0x01, 0x12, 0x03, 0x71, 0x08, 0x15, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12,
    0x03, 0x73, 0x02, 0x1c, 0x1a, 0x17, 0x2f, 0x20, 0x41, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x05, 0x12, 0x03, 0x73, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x73, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x73, 0x1a, 0x1b, 0x0a, 0x2c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x04, 0x77, 0x00, 0x7c,
    0x01, 0x1a, 0x20, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x61, 0x20, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x77, 0x08, 0x21, 0x0a,
    0x20, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x03, 0x79, 0x02, 0x1e, 0x1a, 0x13, 0x2f, 0x20,
    0x53, 0x74, 0x65, 0x70, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x03, 0x79, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x03, 0x79, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x03, 0x12, 0x03, 0x79, 0x1c, 0x1d, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x10, 0x02,
    0x01, 0x12, 0x03, 0x7b, 0x02, 0x1e, 0x1a, 0x14, 0x2f, 0x20, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x6d, 0x61, 0x64, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x7b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x7b, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x7b, 0x1c, 0x1d, 0x0a, 0x35, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x05, 0x7f, 0x00, 0x86, 0x01,
    0x01, 0x1a, 0x28, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x50, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72,
    0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x11, 0x01, 0x12, 0x03, 0x7f, 0x08, 0x22, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12,
    0x04, 0x81, 0x01, 0x02, 0x1c, 0x1a, 0x1a, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x69, 0x6e,
    0x64, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x10, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x01, 0x1a, 0x1b, 0x0a, 0x43, 0x0a,
    0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0x83, 0x01, 0x02, 0x1d, 0x1a, 0x35, 0x2f, 0x20, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20,
    0x77, 0x61, 0x73, 0x20, 0x75, 0x6e, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x66, 0x75, 0x6c,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0x83, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0x83, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0x83, 0x01, 0x12, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0x83, 0x01, 0x1b, 0x1c, 0x0a, 0x52,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0x85, 0x01, 0x02, 0x23, 0x1a, 0x44, 0x2f, 0x20,
    0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69,
    0x6c, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x65, 0x72, 0x65, 0x20, 0x63, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0x85, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0x85, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0x85, 0x01, 0x12, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0x85, 0x01, 0x21, 0x22, 0x0a, 0xb0,
    0x01, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0x8a, 0x01, 0x00, 0x8d, 0x01, 0x01, 0x1a, 0xa1, 0x01,
    0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x65, 0x72,
    0x66, 0x6f, 0x72, 0x6d, 0x20, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72,
    0x69, 0x6e, 0x67, 0x0a, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x65, 0x72,
    0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x64, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x68, 0x65, 0x72,
    0x65, 0x2c, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x61, 0x64, 0x20, 0x69, 0x74, 0x20, 0x70, 0x72,
    0x6f, 0x76, 0x69, 0x64, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x20, 0x65, 0x6e,
    0x61, 0x62, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x44, 0x45, 0x20, 0x74,
    0x6f, 0x20, 0x64, 0x6f, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x21, 0x0a, 0x55,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x01, 0x02, 0x1b, 0x1a, 0x47, 0x2f, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x62, 0x6c, 0x6f, 0x62, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x65, 0x70, 0x73, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65,
    0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x63,
    0x65, 0x70, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x8c, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8c,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x01,
    0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x19,
    0x1a, 0x0a, 0x42, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0x90, 0x01, 0x00, 0x9b, 0x01, 0x01, 0x1a,
    0x34, 0x2f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x65,
    0x72, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f,
    0x72, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0x90, 0x01,
    0x08, 0x1d, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0x92, 0x01, 0x02, 0x20,
    0x1a, 0x25, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x20,
    0x6e, 0x61, 0x6d, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x92, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12,
    0x04, 0x92, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x92, 0x01, 0x10, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0x92,
    0x01, 0x1e, 0x1f, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0x94, 0x01, 0x02,
    0x1a, 0x1a, 0x13, 0x2f, 0x20, 0x73, 0x74, 0x65, 0x70, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x94, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x06, 0x12, 0x04,
    0x94, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94,
    0x01, 0x10, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x01,
    0x18, 0x19, 0x0a, 0x4d, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0x96, 0x01, 0x02, 0x28,
    0x1a, 0x3f, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x69, 0x66, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69,
    0x6e, 0x67, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x6f, 0x6e,
    0x65, 0x20, 0x61, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0x96, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x05, 0x12, 0x04, 0x96, 0x01, 0x0b, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x10, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0x96, 0x01, 0x26, 0x27, 0x0a, 0x4e, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0x98, 0x01, 0x02, 0x26, 0x1a, 0x40, 0x2f, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x6e,
    0x61, 0x6d, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x65, 0x78, 0x74,
    0x72, 0x61, 0x63, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12, 0x04, 0x98, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04, 0x98, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0x98, 0x01, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x03, 0x03, 0x12, 0x04, 0x98, 0x01, 0x24, 0x25, 0x0a, 0x5d, 0x0a, 0x04, 0x04, 0x13, 0x02,
    0x04, 0x12, 0x04, 0x9a, 0x01, 0x02, 0x29, 0x1a, 0x4f, 0x2f, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x20,
    0x72, 0x65, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63,
    0x74, 0x65, 0x64, 0x20, 0x74, 0x65, 0x78, 0x74, 0x2c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72,
    0x65, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x69, 0x66, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x41, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x9a, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x9a, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x9a, 0x01, 0x14, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x9a, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0x9d, 0x01, 0x00, 0xa3,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x10, 0x0a,
    0x43, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x1f, 0x1a, 0x35, 0x2f,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x66, 0x72,
    0x6f, 0x6d, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x78, 0x74, 0x72, 0x61, 0x63,
    0x74, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9f,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9f, 0x01,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x12,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x1d, 0x1e,
    0x0a, 0x4d, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x02, 0x24, 0x1a, 0x3f,
    0x2f, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x65, 0x6e, 0x64, 0x69, 0x6e,
    0x67, 0x20, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66,
    0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x65, 0x78, 0x74, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x02, 0x12, 0x04, 0xa2, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xa2, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xa2, 0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xa2, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xa5, 0x01, 0x00,
    0xac, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x0c,
    0x0a, 0x21, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xa7, 0x01, 0x02, 0x1b, 0x1a, 0x13,
    0x2f, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74,
    0x65, 0x70, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa7, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa7, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x12, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x19, 0x1a, 0x0a,
    0x34, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x02, 0x1c, 0x1a, 0x26, 0x2f,
    0x20, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20,
    0x69, 0x6e, 0x20, 0x73, 0x74, 0x65, 0x70, 0x20, 0x61, 0x73, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xa9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa9,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa9, 0x01,
    0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x1a,
    0x1b, 0x0a, 0x54, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xab, 0x01, 0x02, 0x25, 0x1a,
    0x46, 0x2f, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x20, 0x68, 0x65, 0x61, 0x64,
    0x69, 0x6e, 0x67, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x63, 0x6f, 0x6d, 0x65, 0x73,
    0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x20, 0x74, 0x6f, 0x20, 0x63,
    0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04,
    0x12, 0x04, 0xab, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12,
    0x04, 0xab, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xab, 0x01, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0xab,
    0x01, 0x23, 0x24, 0x0a, 0x43, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xaf, 0x01, 0x00, 0xb6, 0x01,
    0x01, 0x1a, 0x35, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74,
    0x20, 0x74, 0x6f, 0x20, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x20, 0x72, 0x65, 0x66, 0x61,
    0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12,
    0x04, 0xaf, 0x01, 0x08, 0x1e, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xb1,
    0x01, 0x02, 0x1e, 0x1a, 0x1a, 0x2f, 0x20, 0x46, 0x6c, 0x61, 0x67, 0x20, 0x69, 0x6e, 0x64, 0x69,
    0x63, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x53, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x10, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x1c, 0x1d, 0x0a, 0x43, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x1c, 0x1a, 0x35, 0x2f, 0x20, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x61,
    0x73, 0x20, 0x75, 0x6e, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x66, 0x75, 0x6c, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb3, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x1a, 0x1b, 0x0a, 0x52, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x02, 0x12, 0x04, 0xb5, 0x01, 0x02, 0x23, 0x1a, 0x44, 0x2f, 0x20, 0x43, 0x6f,
    0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x65, 0x72, 0x65, 0x20, 0x63, 0x68, 0x61, 0x6e,
    0x67, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb5, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb5, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x12, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb5, 0x01, 0x21, 0x22, 0x0a, 0x2d, 0x0a, 0x02,
    0x04, 0x17, 0x12, 0x06, 0xb9, 0x01, 0x00, 0xbc, 0x01, 0x01, 0x1a, 0x1f, 0x2f, 0x20, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x17, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x08, 0x1a, 0x0a, 0x26, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00,
    0x12, 0x04, 0xbb, 0x01, 0x04, 0x1e, 0x1a, 0x18, 0x2f, 0x20, 0x53, 0x70, 0x65, 0x63, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x74, 0x65, 0x64, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbb, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x1c, 0x1d, 0x0a, 0x32, 0x0a, 0x02,
    0x04, 0x18, 0x12, 0x06, 0xbf, 0x01, 0x00, 0xc4, 0x01, 0x01, 0x1a, 0x24, 0x2f, 0x20, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x70, 0x65, 0x63, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x1b, 0x0a, 0x2e, 0x0a,
    0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x1f, 0x1a, 0x20, 0x2f, 0x20, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x73, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x6f,
    0x6e, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc1, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x14, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x1d, 0x1e, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x18, 0x02,
    0x01, 0x12, 0x04, 0xc3, 0x01, 0x04, 0x21, 0x1a, 0x22, 0x2f, 0x20, 0x57, 0x61, 0x72, 0x6e, 0x69,
    0x6e, 0x67, 0x73, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xc3, 0x01, 0x1f, 0x20, 0x0a, 0x46, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xc7,
    0x01, 0x00, 0xc8, 0x01, 0x01, 0x1a, 0x38, 0x2f, 0x20, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x61, 0x20, 0x41, 0x50, 0x49, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x25, 0x0a, 0xa1, 0x01, 0x0a,
    0x02, 0x04, 0x1a, 0x12, 0x06, 0xcc, 0x01, 0x00, 0x98, 0x02, 0x01, 0x1a, 0x92, 0x01, 0x2f, 0x20,
    0x41, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x6c, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x6f, 0x70, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x2f, 0x20, 0x4f, 0x6e, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2f, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2c, 0x20, 0x64, 0x65,
    0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x12, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x1a, 0x04, 0x00, 0x12, 0x06, 0xcd, 0x01, 0x02, 0xe4, 0x01, 0x03, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x04, 0x00, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x07, 0x15, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1a, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xce, 0x01, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1a, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xce, 0x01, 0x04, 0x19, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xce, 0x01, 0x1c, 0x1d, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x04, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x04, 0x1a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xcf, 0x01, 0x1d, 0x1e,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xd0, 0x01, 0x04, 0x23,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x04,
    0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xd0, 0x01,
    0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xd1, 0x01,
    0x04, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd1,
    0x01, 0x04, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04,
    0xd1, 0x01, 0x22, 0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04,
    0xd2, 0x01, 0x04, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xd2, 0x01, 0x04, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x04, 0xd2, 0x01, 0x19, 0x1a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x04, 0xd3, 0x01, 0x04, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02,
    0x05, 0x02, 0x12, 0x04, 0xd3, 0x01, 0x19, 0x1a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x04, 0xd4, 0x01, 0x04, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00,
    0x02, 0x06, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x04, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04,
    0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0xd4, 0x01, 0x19, 0x1a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a,
    0x04, 0x00, 0x02, 0x07, 0x12, 0x04, 0xd5, 0x01, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a,
    0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x04, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1a, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0xd5, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1a, 0x04, 0x00, 0x02, 0x08, 0x12, 0x04, 0xd6, 0x01, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1a, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x04, 0x17, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x04, 0xd6, 0x01, 0x1a, 0x1b, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x09, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0xd7, 0x01, 0x1b, 0x1d,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x29,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x04,
    0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x04, 0xd8, 0x01,
    0x26, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x04, 0xd9, 0x01,
    0x04, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xd9,
    0x01, 0x04, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x04,
    0xd9, 0x01, 0x27, 0x29, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x04,
    0xda, 0x01, 0x04, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12,
    0x04, 0xda, 0x01, 0x04, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0c, 0x02,
    0x12, 0x04, 0xda, 0x01, 0x14, 0x16, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0d,
    0x12, 0x04, 0xdb, 0x01, 0x04, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x0d,
    0x01, 0x12, 0x04, 0xdb, 0x01, 0x04, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02,
    0x0d, 0x02, 0x12, 0x04, 0xdb, 0x01, 0x1c, 0x1e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00,
    0x02, 0x0e, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00,
    0x02, 0x0e, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04,
    0x00, 0x02, 0x0e, 0x02, 0x12, 0x04, 0xdc, 0x01, 0x1d, 0x1f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a,
    0x04, 0x00, 0x02, 0x0f, 0x12, 0x04, 0xdd, 0x01, 0x04, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a,
    0x04, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1a, 0x04, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x04, 0xdd, 0x01, 0x20, 0x22, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1a, 0x04, 0x00, 0x02, 0x10, 0x12, 0x04, 0xde, 0x01, 0x04, 0x24, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1a, 0x04, 0x00, 0x02, 0x10, 0x01, 0x12, 0x04, 0xde, 0x01, 0x04, 0x1e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x10, 0x02, 0x12, 0x04, 0xde, 0x01, 0x21, 0x23, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x11, 0x12, 0x04, 0xdf, 0x01, 0x04, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x04, 0x19, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x11, 0x02, 0x12, 0x04, 0xdf, 0x01, 0x1c, 0x1e,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x12, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x20,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x12, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x04,
    0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x12, 0x02, 0x12, 0x04, 0xe0, 0x01,
    0x1d, 0x1f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x13, 0x12, 0x04, 0xe1, 0x01,
    0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x13, 0x01, 0x12, 0x04, 0xe1,
    0x01, 0x04, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x13, 0x02, 0x12, 0x04,
    0xe1, 0x01, 0x19, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x14, 0x12, 0x04,
    0xe2, 0x01, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x14, 0x01, 0x12,
    0x04, 0xe2, 0x01, 0x04, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x14, 0x02,
    0x12, 0x04, 0xe2, 0x01, 0x1a, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x15,
    0x12, 0x04, 0xe3, 0x01, 0x04, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x15,
    0x01, 0x12, 0x04, 0xe3, 0x01, 0x04, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02,
    0x15, 0x02, 0x12, 0x04, 0xe3, 0x01, 0x24, 0x26, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00,
    0x12, 0x04, 0xe7, 0x01, 0x02, 0x2a, 0x1a, 0x1e, 0x2f, 0x20, 0x54, 0x79, 0x70, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x41, 0x50, 0x49, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67,
    0x20, 0x6d, 0x61, 0x64, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xe7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xe7, 0x01, 0x0b, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe7,
    0x01, 0x1a, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe7, 0x01,
    0x28, 0x29, 0x0a, 0xa2, 0x01, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0xea, 0x01, 0x02,
    0x1f, 0x1a, 0x93, 0x01, 0x2f, 0x20, 0x41, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x69,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x20, 0x41, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x6f,
    0x70, 0x79, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x2e, 0x0a, 0x2f, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a,
    0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x26, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xea, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xea, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xea, 0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xea,
    0x01, 0x1d, 0x1e, 0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x02, 0x12, 0x04, 0xed, 0x01, 0x02,
    0x38, 0x1a, 0x41, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74,
    0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74,
    0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x04, 0x12, 0x04, 0xed,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x06, 0x12, 0x04, 0xed, 0x01,
    0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x01, 0x12, 0x04, 0xed, 0x01, 0x21,
    0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x03, 0x12, 0x04, 0xed, 0x01, 0x36, 0x37,
    0x0a, 0x51, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x03, 0x12, 0x04, 0xef, 0x01, 0x02, 0x3a, 0x1a, 0x43,
    0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x6f, 0x6f,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67,
    0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x72,
    0x6f, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x04, 0x12, 0x04, 0xef, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x06, 0x12, 0x04, 0xef, 0x01, 0x0b,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x01, 0x12, 0x04, 0xef, 0x01, 0x22, 0x35,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x03, 0x12, 0x04, 0xef, 0x01, 0x38, 0x39, 0x0a,
    0x59, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x02, 0x42, 0x1a, 0x4b, 0x2f,
    0x20, 0x5b, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65,
    0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0xf1, 0x01, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xf1, 0x01, 0x26, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xf1, 0x01, 0x40, 0x41, 0x0a, 0x5b, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x05, 0x12, 0x04,
    0xf3, 0x01, 0x02, 0x44, 0x1a, 0x4d, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x74,
    0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x6f, 0x6f, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x05, 0x04, 0x12, 0x04, 0xf3, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x05, 0x06, 0x12, 0x04, 0xf3, 0x01, 0x0b,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x27, 0x3f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x42, 0x43, 0x0a,
    0x49, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x06, 0x12, 0x04, 0xf5, 0x01, 0x02, 0x32, 0x1a, 0x3b, 0x2f,
    0x20, 0x5b, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x06, 0x04, 0x12, 0x04, 0xf5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x06, 0x06, 0x12, 0x04, 0xf5, 0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x06,
    0x01, 0x12, 0x04, 0xf5, 0x01, 0x1e, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x06, 0x03,
    0x12, 0x04, 0xf5, 0x01, 0x30, 0x31, 0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x07, 0x12, 0x04,
    0xf7, 0x01, 0x02, 0x34, 0x1a, 0x3d, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53,
    0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65,
    0x74, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x65, 0x70, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x07, 0x04, 0x12, 0x04, 0xf7, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x07, 0x06, 0x12, 0x04, 0xf7, 0x01, 0x0b,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x07, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x1f, 0x2f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x07, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x32, 0x33, 0x0a,
    0x49, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x08, 0x12, 0x04, 0xf9, 0x01, 0x02, 0x32, 0x1a, 0x3b, 0x2f,
    0x20, 0x5b, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x08, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x08, 0x06, 0x12, 0x04, 0xf9, 0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x08,
    0x01, 0x12, 0x04, 0xf9, 0x01, 0x1e, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x08, 0x03,
    0x12, 0x04, 0xf9, 0x01, 0x30, 0x31, 0x0a, 0x4b, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x09, 0x12, 0x04,
    0xfb, 0x01, 0x02, 0x35, 0x1a, 0x3d, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x53,
    0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65,
    0x74, 0x41, 0x6c, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x09, 0x04, 0x12, 0x04, 0xfb, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x09, 0x06, 0x12, 0x04, 0xfb, 0x01, 0x0b,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x09, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x1f, 0x2f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x09, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x32, 0x34, 0x0a,
    0x4b, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x0a, 0x12, 0x04, 0xfd, 0x01, 0x02, 0x35, 0x1a, 0x3d, 0x2f,
    0x20, 0x5b, 0x47, 0x65, 0x74, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xfd, 0x01, 0x0b, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x0a, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x1f, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x0a, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x32, 0x34, 0x0a, 0x4d, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x0b,
    0x12, 0x04, 0xff, 0x01, 0x02, 0x37, 0x1a, 0x3f, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x53, 0x74,
    0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d,
    0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x65, 0x70, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0b, 0x04,
    0x12, 0x04, 0xff, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0b, 0x06, 0x12,
    0x04, 0xff, 0x01, 0x0b, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0b, 0x01, 0x12, 0x04,
    0xff, 0x01, 0x20, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xff,
    0x01, 0x34, 0x36, 0x0a, 0x63, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x0c, 0x12, 0x04, 0x81, 0x02, 0x02,
    0x3f, 0x1a, 0x55, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67,
    0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61,
    0x67, 0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0c,
    0x04, 0x12, 0x04, 0x81, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0c, 0x06,
    0x12, 0x04, 0x81, 0x02, 0x0b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0c, 0x01, 0x12,
    0x04, 0x81, 0x02, 0x2b, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0c, 0x03, 0x12, 0x04,
    0x81, 0x02, 0x3c, 0x3e, 0x0a, 0x65, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x0d, 0x12, 0x04, 0x83, 0x02,
    0x02, 0x41, 0x1a, 0x57, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61,
    0x67, 0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74, 0x68, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x4c, 0x61, 0x6e, 0x67,
    0x75, 0x61, 0x67, 0x65, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x4c, 0x69, 0x62, 0x50, 0x61, 0x74,
    0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x83, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x0d, 0x06, 0x12, 0x04, 0x83, 0x02, 0x0b, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x0d, 0x01, 0x12, 0x04, 0x83, 0x02, 0x2c, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0d,
    0x03, 0x12, 0x04, 0x83, 0x02, 0x3e, 0x40, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x0e, 0x12,
    0x04, 0x85, 0x02, 0x02, 0x24, 0x1a, 0x31, 0x2f, 0x20, 0x5b, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0e,
    0x04, 0x12, 0x04, 0x85, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0e, 0x06,
    0x12, 0x04, 0x85, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0e, 0x01, 0x12,
    0x04, 0x85, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0e, 0x03, 0x12, 0x04,
    0x85, 0x02, 0x21, 0x23, 0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x0f, 0x12, 0x04, 0x87, 0x02,
    0x02, 0x39, 0x1a, 0x41, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x43, 0x6f, 0x6e,
    0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65,
    0x74, 0x41, 0x6c, 0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0f, 0x04, 0x12, 0x04,
    0x87, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0f, 0x06, 0x12, 0x04, 0x87,
    0x02, 0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x87, 0x02,
    0x21, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0f, 0x03, 0x12, 0x04, 0x87, 0x02, 0x36,
    0x38, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x10, 0x12, 0x04, 0x89, 0x02, 0x02, 0x3b, 0x1a,
    0x43, 0x2f, 0x20, 0x5b, 0x47, 0x65, 0x74, 0x41, 0x6c, 0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70,
    0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75,
    0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x41,
    0x6c, 0x6c, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x10, 0x04, 0x12, 0x04, 0x89,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x10, 0x06, 0x12, 0x04, 0x89, 0x02,
    0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x10, 0x01, 0x12, 0x04, 0x89, 0x02, 0x22,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x10, 0x03, 0x12, 0x04, 0x89, 0x02, 0x38, 0x3a,
    0x0a, 0x57, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x11, 0x12, 0x04, 0x8b, 0x02, 0x02, 0x44, 0x1a, 0x49,
    0x2f, 0x20, 0x5b, 0x50, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67,
    0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x50, 0x65,
    0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x11, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x11,
    0x06, 0x12, 0x04, 0x8b, 0x02, 0x0b, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x11, 0x01,
    0x12, 0x04, 0x8b, 0x02, 0x25, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x11, 0x03, 0x12,
    0x04, 0x8b, 0x02, 0x41, 0x43, 0x0a, 0x59, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x12, 0x12, 0x04, 0x8d,
    0x02, 0x02, 0x46, 0x1a, 0x4b, 0x2f, 0x20, 0x5b, 0x50, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52,
    0x65, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x50, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x65, 0x66, 0x61, 0x63,
    0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x12, 0x04, 0x12, 0x04, 0x8d, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x12, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x0b, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x12, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x26, 0x40, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x12, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x43, 0x45, 0x0a, 0x4f, 0x0a, 0x04,
    0x04, 0x1a, 0x02, 0x13, 0x12, 0x04, 0x8f, 0x02, 0x02, 0x3c, 0x1a, 0x41, 0x2f, 0x20, 0x5b, 0x45,
    0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e,
    0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x13, 0x04, 0x12, 0x04, 0x8f, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x13, 0x06, 0x12, 0x04, 0x8f, 0x02, 0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x13, 0x01, 0x12, 0x04, 0x8f, 0x02, 0x21, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x13, 0x03, 0x12, 0x04, 0x8f, 0x02, 0x39, 0x3b, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x1a, 0x02,
    0x14, 0x12, 0x04, 0x91, 0x02, 0x02, 0x3e, 0x1a, 0x43, 0x2f, 0x20, 0x5b, 0x45, 0x78, 0x74, 0x72,
    0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65, 0x70, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x5d, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x45, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x43, 0x6f, 0x6e, 0x63, 0x65,
    0x70, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x14, 0x04, 0x12, 0x04, 0x91, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x14, 0x06, 0x12, 0x04, 0x91, 0x02, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x14, 0x01, 0x12, 0x04, 0x91, 0x02, 0x22, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x14, 0x03, 0x12, 0x04, 0x91, 0x02, 0x3b, 0x3d, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x15,
    0x12, 0x04, 0x93, 0x02, 0x02, 0x36, 0x1a, 0x3c, 0x2f, 0x20, 0x5b, 0x46, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5d, 0x20, 0x28,
    0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x15, 0x04, 0x12, 0x04, 0x93,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x15, 0x06, 0x12, 0x04, 0x93, 0x02,
    0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x15, 0x01, 0x12, 0x04, 0x93, 0x02, 0x1e,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x15, 0x03, 0x12, 0x04, 0x93, 0x02, 0x33, 0x35,
    0x0a, 0x4c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x16, 0x12, 0x04, 0x95, 0x02, 0x02, 0x38, 0x1a, 0x3e,
    0x2f, 0x20, 0x5b, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x53, 0x70, 0x65, 0x63, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x20, 0x28, 0x23, 0x67, 0x61, 0x75, 0x67, 0x65, 0x2e,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x53,
    0x70, 0x65, 0x63, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x16, 0x04, 0x12, 0x04, 0x95, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x16, 0x06, 0x12, 0x04, 0x95, 0x02, 0x0b, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x16, 0x01, 0x12, 0x04, 0x95, 0x02, 0x1f, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x16, 0x03, 0x12, 0x04, 0x95, 0x02, 0x35, 0x37, 0x0a, 0x60, 0x0a, 0x04, 0x04, 0x1a,
    0x02, 0x17, 0x12, 0x04, 0x97, 0x02, 0x02, 0x4b, 0x1a, 0x52, 0x2f, 0x20, 0x5b, 0x55, 0x6e, 0x73,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x41, 0x70, 0x69, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x5d, 0x20, 0x28, 0x23, 0x67, 0x61,
    0x75, 0x67, 0x65, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x55, 0x6e, 0x73,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x41, 0x70, 0x69, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x17, 0x04, 0x12, 0x04, 0x97, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x17, 0x06, 0x12, 0x04, 0x97, 0x02, 0x0b, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x17, 0x01, 0x12, 0x04, 0x97, 0x02, 0x29, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x17, 0x03, 0x12, 0x04, 0x97, 0x02, 0x48, 0x4a,
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
