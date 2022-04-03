// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum ImuDataOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Contains all the relevant data from the IMU
pub struct ImuData<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ImuData<'a> {
  type Inner = ImuData<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> ImuData<'a> {
  pub const VT_ORIENTATION: flatbuffers::VOffsetT = 4;
  pub const VT_POSITION: flatbuffers::VOffsetT = 6;
  pub const VT_RAW_ROT_VEL: flatbuffers::VOffsetT = 8;
  pub const VT_RAW_TRANS_ACCEL: flatbuffers::VOffsetT = 10;
  pub const VT_TEMP: flatbuffers::VOffsetT = 12;
  pub const VT_POLL_RATE: flatbuffers::VOffsetT = 14;
  pub const VT_MOUNTING_ROTATION: flatbuffers::VOffsetT = 16;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ImuData { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ImuDataArgs<'args>
  ) -> flatbuffers::WIPOffset<ImuData<'bldr>> {
    let mut builder = ImuDataBuilder::new(_fbb);
    if let Some(x) = args.poll_rate { builder.add_poll_rate(x); }
    if let Some(x) = args.temp { builder.add_temp(x); }
    if let Some(x) = args.raw_trans_accel { builder.add_raw_trans_accel(x); }
    if let Some(x) = args.raw_rot_vel { builder.add_raw_rot_vel(x); }
    if let Some(x) = args.position { builder.add_position(x); }
    if let Some(x) = args.orientation { builder.add_orientation(x); }
    if let Some(x) = args.mounting_rotation { builder.add_mounting_rotation(x); }
    builder.finish()
  }


  #[inline]
  pub fn orientation(&self) -> Option<&'a super::math::Quat> {
    self._tab.get::<super::math::Quat>(ImuData::VT_ORIENTATION, None)
  }
  /// Position, in meters
  #[inline]
  pub fn position(&self) -> Option<&'a super::math::Vec3f> {
    self._tab.get::<super::math::Vec3f>(ImuData::VT_POSITION, None)
  }
  /// Raw rotational velocity, in euler angles
  #[inline]
  pub fn raw_rot_vel(&self) -> Option<&'a super::math::Vec3f> {
    self._tab.get::<super::math::Vec3f>(ImuData::VT_RAW_ROT_VEL, None)
  }
  /// Raw translational acceleration, in m/s^2
  #[inline]
  pub fn raw_trans_accel(&self) -> Option<&'a super::math::Vec3f> {
    self._tab.get::<super::math::Vec3f>(ImuData::VT_RAW_TRANS_ACCEL, None)
  }
  /// Temperature in degrees celsius
  #[inline]
  pub fn temp(&self) -> Option<f32> {
    self._tab.get::<f32>(ImuData::VT_TEMP, None)
  }
  /// average samples per second
  #[inline]
  pub fn poll_rate(&self) -> Option<f32> {
    self._tab.get::<f32>(ImuData::VT_POLL_RATE, None)
  }
  #[inline]
  pub fn mounting_rotation(&self) -> Option<u16> {
    self._tab.get::<u16>(ImuData::VT_MOUNTING_ROTATION, None)
  }
}

impl flatbuffers::Verifiable for ImuData<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<super::math::Quat>("orientation", Self::VT_ORIENTATION, false)?
     .visit_field::<super::math::Vec3f>("position", Self::VT_POSITION, false)?
     .visit_field::<super::math::Vec3f>("raw_rot_vel", Self::VT_RAW_ROT_VEL, false)?
     .visit_field::<super::math::Vec3f>("raw_trans_accel", Self::VT_RAW_TRANS_ACCEL, false)?
     .visit_field::<f32>("temp", Self::VT_TEMP, false)?
     .visit_field::<f32>("poll_rate", Self::VT_POLL_RATE, false)?
     .visit_field::<u16>("mounting_rotation", Self::VT_MOUNTING_ROTATION, false)?
     .finish();
    Ok(())
  }
}
pub struct ImuDataArgs<'a> {
    pub orientation: Option<&'a super::math::Quat>,
    pub position: Option<&'a super::math::Vec3f>,
    pub raw_rot_vel: Option<&'a super::math::Vec3f>,
    pub raw_trans_accel: Option<&'a super::math::Vec3f>,
    pub temp: Option<f32>,
    pub poll_rate: Option<f32>,
    pub mounting_rotation: Option<u16>,
}
impl<'a> Default for ImuDataArgs<'a> {
  #[inline]
  fn default() -> Self {
    ImuDataArgs {
      orientation: None,
      position: None,
      raw_rot_vel: None,
      raw_trans_accel: None,
      temp: None,
      poll_rate: None,
      mounting_rotation: None,
    }
  }
}

pub struct ImuDataBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ImuDataBuilder<'a, 'b> {
  #[inline]
  pub fn add_orientation(&mut self, orientation: &super::math::Quat) {
    self.fbb_.push_slot_always::<&super::math::Quat>(ImuData::VT_ORIENTATION, orientation);
  }
  #[inline]
  pub fn add_position(&mut self, position: &super::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::math::Vec3f>(ImuData::VT_POSITION, position);
  }
  #[inline]
  pub fn add_raw_rot_vel(&mut self, raw_rot_vel: &super::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::math::Vec3f>(ImuData::VT_RAW_ROT_VEL, raw_rot_vel);
  }
  #[inline]
  pub fn add_raw_trans_accel(&mut self, raw_trans_accel: &super::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::math::Vec3f>(ImuData::VT_RAW_TRANS_ACCEL, raw_trans_accel);
  }
  #[inline]
  pub fn add_temp(&mut self, temp: f32) {
    self.fbb_.push_slot_always::<f32>(ImuData::VT_TEMP, temp);
  }
  #[inline]
  pub fn add_poll_rate(&mut self, poll_rate: f32) {
    self.fbb_.push_slot_always::<f32>(ImuData::VT_POLL_RATE, poll_rate);
  }
  #[inline]
  pub fn add_mounting_rotation(&mut self, mounting_rotation: u16) {
    self.fbb_.push_slot_always::<u16>(ImuData::VT_MOUNTING_ROTATION, mounting_rotation);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ImuDataBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ImuDataBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ImuData<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for ImuData<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("ImuData");
      ds.field("orientation", &self.orientation());
      ds.field("position", &self.position());
      ds.field("raw_rot_vel", &self.raw_rot_vel());
      ds.field("raw_trans_accel", &self.raw_trans_accel());
      ds.field("temp", &self.temp());
      ds.field("poll_rate", &self.poll_rate());
      ds.field("mounting_rotation", &self.mounting_rotation());
      ds.finish()
  }
}