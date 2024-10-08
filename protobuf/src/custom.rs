// This file is generated by rust-protobuf 3.5.1. Do not edit
// .proto file is parsed by protoc --rust_out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `custom.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_5_1;

// @@protoc_insertion_point(message:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
    // message fields
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.device_friendly_name)
    pub device_friendly_name: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.account_name)
    pub account_name: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.encrypted_password)
    pub encrypted_password: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.encryption_timestamp)
    pub encryption_timestamp: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.remember_login)
    pub remember_login: ::std::option::Option<bool>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.platform_type)
    pub platform_type: ::std::option::Option<::protobuf::EnumOrUnknown<super::steammessages_auth_steamclient::EAuthTokenPlatformType>>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.persistence)
    pub persistence: ::std::option::Option<::protobuf::EnumOrUnknown<super::enums::ESessionPersistence>>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.website_id)
    pub website_id: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.device_details)
    pub device_details: ::protobuf::MessageField<super::steammessages_auth_steamclient::CAuthentication_DeviceDetails>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.guard_data)
    pub guard_data: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.language)
    pub language: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.qos_level)
    pub qos_level: ::std::option::Option<i32>,
    // special fields
    // @@protoc_insertion_point(special_field:CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
    fn default() -> &'a CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
        <CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData as ::protobuf::Message>::default_instance()
    }
}

impl CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
    pub fn new() -> CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
        ::std::default::Default::default()
    }

    // optional string device_friendly_name = 1;

    pub fn device_friendly_name(&self) -> &str {
        match self.device_friendly_name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_device_friendly_name(&mut self) {
        self.device_friendly_name = ::std::option::Option::None;
    }

    pub fn has_device_friendly_name(&self) -> bool {
        self.device_friendly_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_friendly_name(&mut self, v: ::std::string::String) {
        self.device_friendly_name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_friendly_name(&mut self) -> &mut ::std::string::String {
        if self.device_friendly_name.is_none() {
            self.device_friendly_name = ::std::option::Option::Some(::std::string::String::new());
        }
        self.device_friendly_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_friendly_name(&mut self) -> ::std::string::String {
        self.device_friendly_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string account_name = 2;

    pub fn account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_account_name(&mut self) {
        self.account_name = ::std::option::Option::None;
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name = ::std::option::Option::Some(::std::string::String::new());
        }
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string encrypted_password = 3;

    pub fn encrypted_password(&self) -> &str {
        match self.encrypted_password.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_encrypted_password(&mut self) {
        self.encrypted_password = ::std::option::Option::None;
    }

    pub fn has_encrypted_password(&self) -> bool {
        self.encrypted_password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_password(&mut self, v: ::std::string::String) {
        self.encrypted_password = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_password(&mut self) -> &mut ::std::string::String {
        if self.encrypted_password.is_none() {
            self.encrypted_password = ::std::option::Option::Some(::std::string::String::new());
        }
        self.encrypted_password.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_password(&mut self) -> ::std::string::String {
        self.encrypted_password.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional uint64 encryption_timestamp = 4;

    pub fn encryption_timestamp(&self) -> u64 {
        self.encryption_timestamp.unwrap_or(0)
    }

    pub fn clear_encryption_timestamp(&mut self) {
        self.encryption_timestamp = ::std::option::Option::None;
    }

    pub fn has_encryption_timestamp(&self) -> bool {
        self.encryption_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryption_timestamp(&mut self, v: u64) {
        self.encryption_timestamp = ::std::option::Option::Some(v);
    }

    // optional bool remember_login = 5;

    pub fn remember_login(&self) -> bool {
        self.remember_login.unwrap_or(false)
    }

    pub fn clear_remember_login(&mut self) {
        self.remember_login = ::std::option::Option::None;
    }

    pub fn has_remember_login(&self) -> bool {
        self.remember_login.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remember_login(&mut self, v: bool) {
        self.remember_login = ::std::option::Option::Some(v);
    }

    // optional .EAuthTokenPlatformType platform_type = 6;

    pub fn platform_type(&self) -> super::steammessages_auth_steamclient::EAuthTokenPlatformType {
        match self.platform_type {
            Some(e) => e.enum_value_or(super::steammessages_auth_steamclient::EAuthTokenPlatformType::k_EAuthTokenPlatformType_Unknown),
            None => super::steammessages_auth_steamclient::EAuthTokenPlatformType::k_EAuthTokenPlatformType_Unknown,
        }
    }

    pub fn clear_platform_type(&mut self) {
        self.platform_type = ::std::option::Option::None;
    }

    pub fn has_platform_type(&self) -> bool {
        self.platform_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_platform_type(&mut self, v: super::steammessages_auth_steamclient::EAuthTokenPlatformType) {
        self.platform_type = ::std::option::Option::Some(::protobuf::EnumOrUnknown::new(v));
    }

    // optional .ESessionPersistence persistence = 7;

    pub fn persistence(&self) -> super::enums::ESessionPersistence {
        match self.persistence {
            Some(e) => e.enum_value_or(super::enums::ESessionPersistence::k_ESessionPersistence_Persistent),
            None => super::enums::ESessionPersistence::k_ESessionPersistence_Persistent,
        }
    }

    pub fn clear_persistence(&mut self) {
        self.persistence = ::std::option::Option::None;
    }

    pub fn has_persistence(&self) -> bool {
        self.persistence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persistence(&mut self, v: super::enums::ESessionPersistence) {
        self.persistence = ::std::option::Option::Some(::protobuf::EnumOrUnknown::new(v));
    }

    // optional string website_id = 8;

    pub fn website_id(&self) -> &str {
        match self.website_id.as_ref() {
            Some(v) => v,
            None => "Unknown",
        }
    }

    pub fn clear_website_id(&mut self) {
        self.website_id = ::std::option::Option::None;
    }

    pub fn has_website_id(&self) -> bool {
        self.website_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_website_id(&mut self, v: ::std::string::String) {
        self.website_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_website_id(&mut self) -> &mut ::std::string::String {
        if self.website_id.is_none() {
            self.website_id = ::std::option::Option::Some(::std::string::String::new());
        }
        self.website_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_website_id(&mut self) -> ::std::string::String {
        self.website_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional bytes guard_data = 10;

    pub fn guard_data(&self) -> &[u8] {
        match self.guard_data.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_guard_data(&mut self) {
        self.guard_data = ::std::option::Option::None;
    }

    pub fn has_guard_data(&self) -> bool {
        self.guard_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guard_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.guard_data = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guard_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.guard_data.is_none() {
            self.guard_data = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.guard_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_guard_data(&mut self) -> ::std::vec::Vec<u8> {
        self.guard_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional uint32 language = 11;

    pub fn language(&self) -> u32 {
        self.language.unwrap_or(0)
    }

    pub fn clear_language(&mut self) {
        self.language = ::std::option::Option::None;
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: u32) {
        self.language = ::std::option::Option::Some(v);
    }

    // optional int32 qos_level = 12;

    pub fn qos_level(&self) -> i32 {
        self.qos_level.unwrap_or(2i32)
    }

    pub fn clear_qos_level(&mut self) {
        self.qos_level = ::std::option::Option::None;
    }

    pub fn has_qos_level(&self) -> bool {
        self.qos_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_qos_level(&mut self, v: i32) {
        self.qos_level = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "device_friendly_name",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.device_friendly_name },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.device_friendly_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "account_name",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.account_name },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.account_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "encrypted_password",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.encrypted_password },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.encrypted_password },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "encryption_timestamp",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.encryption_timestamp },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.encryption_timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "remember_login",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.remember_login },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.remember_login },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "platform_type",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.platform_type },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.platform_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "persistence",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.persistence },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.persistence },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "website_id",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.website_id },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.website_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::steammessages_auth_steamclient::CAuthentication_DeviceDetails>(
            "device_details",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.device_details },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.device_details },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "guard_data",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.guard_data },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.guard_data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "language",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.language },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.language },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "qos_level",
            |m: &CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &m.qos_level },
            |m: &mut CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData| { &mut m.qos_level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData>(
            "CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
    const NAME: &'static str = "CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.device_friendly_name = ::std::option::Option::Some(is.read_string()?);
                },
                18 => {
                    self.account_name = ::std::option::Option::Some(is.read_string()?);
                },
                26 => {
                    self.encrypted_password = ::std::option::Option::Some(is.read_string()?);
                },
                32 => {
                    self.encryption_timestamp = ::std::option::Option::Some(is.read_uint64()?);
                },
                40 => {
                    self.remember_login = ::std::option::Option::Some(is.read_bool()?);
                },
                48 => {
                    self.platform_type = ::std::option::Option::Some(is.read_enum_or_unknown()?);
                },
                56 => {
                    self.persistence = ::std::option::Option::Some(is.read_enum_or_unknown()?);
                },
                66 => {
                    self.website_id = ::std::option::Option::Some(is.read_string()?);
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.device_details)?;
                },
                82 => {
                    self.guard_data = ::std::option::Option::Some(is.read_bytes()?);
                },
                88 => {
                    self.language = ::std::option::Option::Some(is.read_uint32()?);
                },
                96 => {
                    self.qos_level = ::std::option::Option::Some(is.read_int32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.device_friendly_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.encrypted_password.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.encryption_timestamp {
            my_size += ::protobuf::rt::uint64_size(4, v);
        }
        if let Some(v) = self.remember_login {
            my_size += 1 + 1;
        }
        if let Some(v) = self.platform_type {
            my_size += ::protobuf::rt::int32_size(6, v.value());
        }
        if let Some(v) = self.persistence {
            my_size += ::protobuf::rt::int32_size(7, v.value());
        }
        if let Some(v) = self.website_id.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.device_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.guard_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        if let Some(v) = self.language {
            my_size += ::protobuf::rt::uint32_size(11, v);
        }
        if let Some(v) = self.qos_level {
            my_size += ::protobuf::rt::int32_size(12, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.device_friendly_name.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.account_name.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.encrypted_password.as_ref() {
            os.write_string(3, v)?;
        }
        if let Some(v) = self.encryption_timestamp {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.remember_login {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.platform_type {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&v))?;
        }
        if let Some(v) = self.persistence {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&v))?;
        }
        if let Some(v) = self.website_id.as_ref() {
            os.write_string(8, v)?;
        }
        if let Some(v) = self.device_details.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.guard_data.as_ref() {
            os.write_bytes(10, v)?;
        }
        if let Some(v) = self.language {
            os.write_uint32(11, v)?;
        }
        if let Some(v) = self.qos_level {
            os.write_int32(12, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
        CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData::new()
    }

    fn clear(&mut self) {
        self.device_friendly_name = ::std::option::Option::None;
        self.account_name = ::std::option::Option::None;
        self.encrypted_password = ::std::option::Option::None;
        self.encryption_timestamp = ::std::option::Option::None;
        self.remember_login = ::std::option::Option::None;
        self.platform_type = ::std::option::Option::None;
        self.persistence = ::std::option::Option::None;
        self.website_id = ::std::option::Option::None;
        self.device_details.clear();
        self.guard_data = ::std::option::Option::None;
        self.language = ::std::option::Option::None;
        self.qos_level = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
        static instance: CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData = CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
            device_friendly_name: ::std::option::Option::None,
            account_name: ::std::option::Option::None,
            encrypted_password: ::std::option::Option::None,
            encryption_timestamp: ::std::option::Option::None,
            remember_login: ::std::option::Option::None,
            platform_type: ::std::option::Option::None,
            persistence: ::std::option::Option::None,
            website_id: ::std::option::Option::None,
            device_details: ::protobuf::MessageField::none(),
            guard_data: ::std::option::Option::None,
            language: ::std::option::Option::None,
            qos_level: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ccustom.proto\x1a\x18steammessages_base.proto\x1a,steammessages_uni\
    fied_base.steamclient.proto\x1a\x0benums.proto\x1a$steammessages_auth.st\
    eamclient.proto\"\xbf\x08\nFCAuthentication_BeginAuthSessionViaCredentia\
    ls_Request_BinaryGuardData\x120\n\x14device_friendly_name\x18\x01\x20\
    \x01(\tR\x12deviceFriendlyName\x12!\n\x0caccount_name\x18\x02\x20\x01(\t\
    R\x0baccountName\x12V\n\x12encrypted_password\x18\x03\x20\x01(\tR\x11enc\
    ryptedPasswordB'\x82\xb5\x18#password,\x20RSA\x20encrypted\x20client\x20\
    side\x12X\n\x14encryption_timestamp\x18\x04\x20\x01(\x04R\x13encryptionT\
    imestampB%\x82\xb5\x18!timestamp\x20to\x20map\x20to\x20a\x20key\x20-\x20\
    STime\x125\n\x0eremember_login\x18\x05\x20\x01(\x08R\rrememberLoginB\x0e\
    \x82\xb5\x18\ndeprecated\x12^\n\rplatform_type\x18\x06\x20\x01(\x0e2\x17\
    .EAuthTokenPlatformType:\x20k_EAuthTokenPlatformType_UnknownR\x0cplatfor\
    mType\x12\x9c\x01\n\x0bpersistence\x18\x07\x20\x01(\x0e2\x14.ESessionPer\
    sistence:\x20k_ESessionPersistence_PersistentR\x0bpersistenceBB\x82\xb5\
    \x18>whether\x20we\x20are\x20requesting\x20a\x20persistent\x20or\x20an\
    \x20ephemeral\x20session\x12h\n\nwebsite_id\x18\x08\x20\x01(\t:\x07Unkno\
    wnR\twebsiteIdB@\x82\xb5\x18<(EMachineAuthWebDomain)\x20identifier\x20of\
    \x20client\x20requesting\x20auth\x12\x87\x01\n\x0edevice_details\x18\t\
    \x20\x01(\x0b2\x1e.CAuthentication_DeviceDetailsR\rdeviceDetailsB@\x82\
    \xb5\x18<User-supplied\x20details\x20about\x20the\x20device\x20attemptin\
    g\x20to\x20sign\x20in\x12D\n\nguard_data\x18\n\x20\x01(\x0cR\tguardDataB\
    %\x82\xb5\x18!steam\x20guard\x20data\x20for\x20client\x20login\x12\x1a\n\
    \x08language\x18\x0b\x20\x01(\rR\x08language\x12b\n\tqos_level\x18\x0c\
    \x20\x01(\x05:\x012R\x08qosLevelBB\x82\xb5\x18>[ENetQOSLevel]\x20client-\
    specified\x20priority\x20for\x20this\x20auth\x20attemptB\x03\x80\x01\x01\
    J\xe5\t\n\x06\x12\x04\0\0\x14\x01\n\t\n\x02\x03\0\x12\x03\0\0\"\n\t\n\
    \x02\x03\x01\x12\x03\x01\06\n\t\n\x02\x03\x02\x12\x03\x02\0\x15\n\t\n\
    \x02\x03\x03\x12\x03\x03\0.\n\x08\n\x01\x08\x12\x03\x05\0\"\n\t\n\x02\
    \x08\x10\x12\x03\x05\0\"\n\n\n\x02\x04\0\x12\x04\x07\0\x14\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x07\x08N\n\x0b\n\x04\x04\0\x02\0\x12\x03\x08\x02+\n\
    \x0c\n\x05\x04\0\x02\0\x04\x12\x03\x08\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\x08\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x08\x12&\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03\x08)*\n\x0b\n\x04\x04\0\x02\x01\x12\x03\t\
    \x02#\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\t\x02\n\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\t\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\t\
    \x12\x1e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\t!\"\n\x0b\n\x04\x04\0\
    \x02\x02\x12\x03\n\x02a\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\n\x02\n\n\
    \x0c\n\x05\x04\0\x02\x02\x05\x12\x03\n\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\
    \x01\x12\x03\n\x12$\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\n'(\n\x0c\n\
    \x05\x04\0\x02\x02\x08\x12\x03\n)`\n\x0f\n\x08\x04\0\x02\x02\x08\xd0\x86\
    \x03\x12\x03\n*_\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0b\x02a\n\x0c\n\x05\
    \x04\0\x02\x03\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\
    \x03\x0b\x0b\x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0b\x12&\n\x0c\n\
    \x05\x04\0\x02\x03\x03\x12\x03\x0b)*\n\x0c\n\x05\x04\0\x02\x03\x08\x12\
    \x03\x0b+`\n\x0f\n\x08\x04\0\x02\x03\x08\xd0\x86\x03\x12\x03\x0b,_\n\x0b\
    \n\x04\x04\0\x02\x04\x12\x03\x0c\x02B\n\x0c\n\x05\x04\0\x02\x04\x04\x12\
    \x03\x0c\x02\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x0c\x0b\x0f\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03\x0c\x10\x1e\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x03\x0c!\"\n\x0c\n\x05\x04\0\x02\x04\x08\x12\x03\x0c#A\n\x0f\n\x08\
    \x04\0\x02\x04\x08\xd0\x86\x03\x12\x03\x0c$@\n\x0b\n\x04\x04\0\x02\x05\
    \x12\x03\r\x02b\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03\r\x02\n\n\x0c\n\
    \x05\x04\0\x02\x05\x06\x12\x03\r\x0b\"\n\x0c\n\x05\x04\0\x02\x05\x01\x12\
    \x03\r#0\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\r34\n\x0c\n\x05\x04\0\x02\
    \x05\x08\x12\x03\r5a\n\x0c\n\x05\x04\0\x02\x05\x07\x12\x03\r@`\n\x0c\n\
    \x04\x04\0\x02\x06\x12\x04\x0e\x02\xaf\x01\n\x0c\n\x05\x04\0\x02\x06\x04\
    \x12\x03\x0e\x02\n\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03\x0e\x0b\x1f\n\
    \x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x0e\x20+\n\x0c\n\x05\x04\0\x02\x06\
    \x03\x12\x03\x0e./\n\r\n\x05\x04\0\x02\x06\x08\x12\x04\x0e0\xae\x01\n\
    \x0c\n\x05\x04\0\x02\x06\x07\x12\x03\x0e;[\n\x10\n\x08\x04\0\x02\x06\x08\
    \xd0\x86\x03\x12\x04\x0e]\xad\x01\n\x0c\n\x04\x04\0\x02\x07\x12\x04\x0f\
    \x02\x87\x01\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03\x0f\x02\n\n\x0c\n\x05\
    \x04\0\x02\x07\x05\x12\x03\x0f\x0b\x11\n\x0c\n\x05\x04\0\x02\x07\x01\x12\
    \x03\x0f\x12\x1c\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x0f\x1f\x20\n\r\n\
    \x05\x04\0\x02\x07\x08\x12\x04\x0f!\x86\x01\n\x0c\n\x05\x04\0\x02\x07\
    \x07\x12\x03\x0f,5\n\x10\n\x08\x04\0\x02\x07\x08\xd0\x86\x03\x12\x04\x0f\
    7\x85\x01\n\x0c\n\x04\x04\0\x02\x08\x12\x04\x10\x02\x8e\x01\n\x0c\n\x05\
    \x04\0\x02\x08\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\0\x02\x08\x06\x12\
    \x03\x10\x0b)\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x10*8\n\x0c\n\x05\
    \x04\0\x02\x08\x03\x12\x03\x10;<\n\r\n\x05\x04\0\x02\x08\x08\x12\x04\x10\
    =\x8d\x01\n\x10\n\x08\x04\0\x02\x08\x08\xd0\x86\x03\x12\x04\x10>\x8c\x01\
    \n\x0b\n\x04\x04\0\x02\t\x12\x03\x11\x02W\n\x0c\n\x05\x04\0\x02\t\x04\
    \x12\x03\x11\x02\n\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\x11\x0b\x10\n\x0c\
    \n\x05\x04\0\x02\t\x01\x12\x03\x11\x11\x1b\n\x0c\n\x05\x04\0\x02\t\x03\
    \x12\x03\x11\x1e\x20\n\x0c\n\x05\x04\0\x02\t\x08\x12\x03\x11!V\n\x0f\n\
    \x08\x04\0\x02\t\x08\xd0\x86\x03\x12\x03\x11\"U\n\x0b\n\x04\x04\0\x02\n\
    \x12\x03\x12\x02\x20\n\x0c\n\x05\x04\0\x02\n\x04\x12\x03\x12\x02\n\n\x0c\
    \n\x05\x04\0\x02\n\x05\x12\x03\x12\x0b\x11\n\x0c\n\x05\x04\0\x02\n\x01\
    \x12\x03\x12\x12\x1a\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\x12\x1d\x1f\n\
    \x0c\n\x04\x04\0\x02\x0b\x12\x04\x13\x02\x80\x01\n\x0c\n\x05\x04\0\x02\
    \x0b\x04\x12\x03\x13\x02\n\n\x0c\n\x05\x04\0\x02\x0b\x05\x12\x03\x13\x0b\
    \x10\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\x03\x13\x11\x1a\n\x0c\n\x05\x04\0\
    \x02\x0b\x03\x12\x03\x13\x1d\x1f\n\x0c\n\x05\x04\0\x02\x0b\x08\x12\x03\
    \x13\x20\x7f\n\x0c\n\x05\x04\0\x02\x0b\x07\x12\x03\x13+,\n\x0f\n\x08\x04\
    \0\x02\x0b\x08\xd0\x86\x03\x12\x03\x13.~\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::steammessages_base::file_descriptor().clone());
            deps.push(super::steammessages_unified_base_steamclient::file_descriptor().clone());
            deps.push(super::enums::file_descriptor().clone());
            deps.push(super::steammessages_auth_steamclient::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CAuthentication_BeginAuthSessionViaCredentials_Request_BinaryGuardData::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
