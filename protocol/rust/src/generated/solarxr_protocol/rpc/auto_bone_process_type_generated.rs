// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_AUTO_BONE_PROCESS_TYPE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_AUTO_BONE_PROCESS_TYPE: u8 = 4;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_AUTO_BONE_PROCESS_TYPE: [AutoBoneProcessType; 5] = [
  AutoBoneProcessType::NONE,
  AutoBoneProcessType::RECORD,
  AutoBoneProcessType::SAVE,
  AutoBoneProcessType::PROCESS,
  AutoBoneProcessType::APPLY,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AutoBoneProcessType(pub u8);
#[allow(non_upper_case_globals)]
impl AutoBoneProcessType {
  pub const NONE: Self = Self(0);
  pub const RECORD: Self = Self(1);
  pub const SAVE: Self = Self(2);
  pub const PROCESS: Self = Self(3);
  pub const APPLY: Self = Self(4);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 4;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::RECORD,
    Self::SAVE,
    Self::PROCESS,
    Self::APPLY,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::RECORD => Some("RECORD"),
      Self::SAVE => Some("SAVE"),
      Self::PROCESS => Some("PROCESS"),
      Self::APPLY => Some("APPLY"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for AutoBoneProcessType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for AutoBoneProcessType {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for AutoBoneProcessType {
    type Output = AutoBoneProcessType;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for AutoBoneProcessType {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for AutoBoneProcessType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for AutoBoneProcessType {}