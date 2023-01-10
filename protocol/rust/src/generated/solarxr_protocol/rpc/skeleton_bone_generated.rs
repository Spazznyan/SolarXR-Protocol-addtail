// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_SKELETON_BONE: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_SKELETON_BONE: u8 = 20;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_SKELETON_BONE: [SkeletonBone; 21] = [
  SkeletonBone::NONE,
  SkeletonBone::HEAD,
  SkeletonBone::NECK,
  SkeletonBone::CHEST,
  SkeletonBone::CHEST_OFFSET,
  SkeletonBone::WAIST,
  SkeletonBone::HIP,
  SkeletonBone::HIP_OFFSET,
  SkeletonBone::HIPS_WIDTH,
  SkeletonBone::UPPER_LEG,
  SkeletonBone::LOWER_LEG,
  SkeletonBone::FOOT_LENGTH,
  SkeletonBone::FOOT_SHIFT,
  SkeletonBone::SKELETON_OFFSET,
  SkeletonBone::SHOULDERS_DISTANCE,
  SkeletonBone::SHOULDERS_WIDTH,
  SkeletonBone::UPPER_ARM,
  SkeletonBone::LOWER_ARM,
  SkeletonBone::CONTROLLER_Y,
  SkeletonBone::CONTROLLER_Z,
  SkeletonBone::ELBOW_OFFSET,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SkeletonBone(pub u8);
#[allow(non_upper_case_globals)]
impl SkeletonBone {
  pub const NONE: Self = Self(0);
  pub const HEAD: Self = Self(1);
  pub const NECK: Self = Self(2);
  pub const CHEST: Self = Self(3);
  pub const CHEST_OFFSET: Self = Self(4);
  pub const WAIST: Self = Self(5);
  pub const HIP: Self = Self(6);
  pub const HIP_OFFSET: Self = Self(7);
  pub const HIPS_WIDTH: Self = Self(8);
  pub const UPPER_LEG: Self = Self(9);
  pub const LOWER_LEG: Self = Self(10);
  pub const FOOT_LENGTH: Self = Self(11);
  pub const FOOT_SHIFT: Self = Self(12);
  pub const SKELETON_OFFSET: Self = Self(13);
  pub const SHOULDERS_DISTANCE: Self = Self(14);
  pub const SHOULDERS_WIDTH: Self = Self(15);
  pub const UPPER_ARM: Self = Self(16);
  pub const LOWER_ARM: Self = Self(17);
  pub const CONTROLLER_Y: Self = Self(18);
  pub const CONTROLLER_Z: Self = Self(19);
  pub const ELBOW_OFFSET: Self = Self(20);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 20;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::HEAD,
    Self::NECK,
    Self::CHEST,
    Self::CHEST_OFFSET,
    Self::WAIST,
    Self::HIP,
    Self::HIP_OFFSET,
    Self::HIPS_WIDTH,
    Self::UPPER_LEG,
    Self::LOWER_LEG,
    Self::FOOT_LENGTH,
    Self::FOOT_SHIFT,
    Self::SKELETON_OFFSET,
    Self::SHOULDERS_DISTANCE,
    Self::SHOULDERS_WIDTH,
    Self::UPPER_ARM,
    Self::LOWER_ARM,
    Self::CONTROLLER_Y,
    Self::CONTROLLER_Z,
    Self::ELBOW_OFFSET,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::HEAD => Some("HEAD"),
      Self::NECK => Some("NECK"),
      Self::CHEST => Some("CHEST"),
      Self::CHEST_OFFSET => Some("CHEST_OFFSET"),
      Self::WAIST => Some("WAIST"),
      Self::HIP => Some("HIP"),
      Self::HIP_OFFSET => Some("HIP_OFFSET"),
      Self::HIPS_WIDTH => Some("HIPS_WIDTH"),
      Self::UPPER_LEG => Some("UPPER_LEG"),
      Self::LOWER_LEG => Some("LOWER_LEG"),
      Self::FOOT_LENGTH => Some("FOOT_LENGTH"),
      Self::FOOT_SHIFT => Some("FOOT_SHIFT"),
      Self::SKELETON_OFFSET => Some("SKELETON_OFFSET"),
      Self::SHOULDERS_DISTANCE => Some("SHOULDERS_DISTANCE"),
      Self::SHOULDERS_WIDTH => Some("SHOULDERS_WIDTH"),
      Self::UPPER_ARM => Some("UPPER_ARM"),
      Self::LOWER_ARM => Some("LOWER_ARM"),
      Self::CONTROLLER_Y => Some("CONTROLLER_Y"),
      Self::CONTROLLER_Z => Some("CONTROLLER_Z"),
      Self::ELBOW_OFFSET => Some("ELBOW_OFFSET"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for SkeletonBone {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for SkeletonBone {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for SkeletonBone {
    type Output = SkeletonBone;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for SkeletonBone {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for SkeletonBone {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for SkeletonBone {}
