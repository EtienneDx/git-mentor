use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::Codegen;
use protobuf_codegen::Customize;
use protobuf_codegen::CustomizeCallback;
use std::io::Result;

fn main() -> Result<()> {
  struct GenSerde;

  impl CustomizeCallback for GenSerde {
    fn message(&self, _message: &MessageDescriptor) -> Customize {
      Customize::default().before("#[derive(::serde::Serialize, ::serde::Deserialize)]")
    }

    fn field(&self, field: &FieldDescriptor) -> Customize {
      if field.proto().type_() == Type::TYPE_ENUM {
        // `EnumOrUnknown` is not a part of rust-protobuf, so external serializer is needed.
        if field.proto().proto3_optional() {
          Customize::default().before(
            "#[serde(serialize_with = \"crate::proto_utils::serialize_opt_enum_or_unknown\", deserialize_with = \"crate::proto_utils::deserialize_opt_enum_or_unknown\")]")
        } else {
          Customize::default().before(
            "#[serde(serialize_with = \"crate::proto_utils::serialize_enum_or_unknown\", deserialize_with = \"crate::proto_utils::deserialize_enum_or_unknown\")]")
        }
      } else {
        Customize::default()
      }
    }

    fn enumeration(&self, _enum_type: &protobuf::reflect::EnumDescriptor) -> Customize {
      Customize::default().before("#[derive(::serde::Serialize, ::serde::Deserialize)]")
    }

    fn oneof(&self, _oneof: &protobuf::reflect::OneofDescriptor) -> Customize {
      Customize::default().before("#[derive(::serde::Serialize, ::serde::Deserialize)]")
    }

    fn special_field(&self, _message: &MessageDescriptor, _field: &str) -> Customize {
      Customize::default().before("#[serde(skip)]")
    }
  }

  Codegen::new()
    .protoc_extra_arg("--proto_path=../gmt-protobuf/")
    .cargo_out_dir("protos")
    .include("../gmt-protobuf")
    .input("../gmt-protobuf/messages.proto")
    .customize_callback(GenSerde)
    .run_from_script();

  Ok(())
}
