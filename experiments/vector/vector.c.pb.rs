extern crate protobuf_cpp as __pb;
extern crate std as __std;
pub mod vector {
#[allow(non_camel_case_types)]
pub struct IntVector {
  msg: ::__std::ptr::NonNull<u8>,
}

impl IntVector {
  pub fn new() -> Self {
    Self { msg: unsafe { __rust_proto_thunk__vector_IntVector_new() } }
  }

  pub fn serialize(&self) -> ::__pb::SerializedData {
    unsafe { __rust_proto_thunk__vector_IntVector_serialize(self.msg) }
  }
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      let data = ::__pb::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__vector_IntVector_deserialize(self.msg, data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }

  // values: repeated int32
  // Unsupported! :(

}  // impl IntVector



impl ::__std::ops::Drop for IntVector {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vector_IntVector_delete(self.msg); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vector_IntVector_new() -> ::__std::ptr::NonNull<u8>;
  fn __rust_proto_thunk__vector_IntVector_delete(raw_msg: ::__std::ptr::NonNull<u8>);
  fn __rust_proto_thunk__vector_IntVector_serialize(raw_msg: ::__std::ptr::NonNull<u8>) -> ::__pb::SerializedData;
  fn __rust_proto_thunk__vector_IntVector_deserialize(raw_msg: ::__std::ptr::NonNull<u8>, data: ::__pb::SerializedData) -> bool;

}  // extern "C" for IntVector


impl IntVector {
  pub fn __unstable_cpp_repr_grant_permission_to_break(&mut self) -> ::__std::ptr::NonNull<u8> {
    self.msg
  }
}

} // mod vector
