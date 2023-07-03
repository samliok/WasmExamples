#include "vector.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// vector.IntVector



// clang-format off
extern "C" {
void* __rust_proto_thunk__vector_IntVector_new(){return new ::vector::IntVector(); }
void __rust_proto_thunk__vector_IntVector_delete(void* ptr) { delete static_cast<::vector::IntVector*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__vector_IntVector_serialize(::vector::IntVector* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__vector_IntVector_deserialize(::vector::IntVector* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

}  // extern "C"
// clang-format on


