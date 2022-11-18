// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum ModelTogglesOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Settings for the skeletal model that are toggles.
pub struct ModelToggles<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ModelToggles<'a> {
  type Inner = ModelToggles<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> ModelToggles<'a> {
  pub const VT_EXTENDED_SPINE: flatbuffers::VOffsetT = 4;
  pub const VT_EXTENDED_PELVIS: flatbuffers::VOffsetT = 6;
  pub const VT_EXTENDED_KNEE: flatbuffers::VOffsetT = 8;
  pub const VT_FORCE_ARMS_FROM_HMD: flatbuffers::VOffsetT = 10;
  pub const VT_FLOOR_CLIP: flatbuffers::VOffsetT = 12;
  pub const VT_SKATING_CORRECTION: flatbuffers::VOffsetT = 14;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ModelToggles { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ModelTogglesArgs
  ) -> flatbuffers::WIPOffset<ModelToggles<'bldr>> {
    let mut builder = ModelTogglesBuilder::new(_fbb);
    if let Some(x) = args.skating_correction { builder.add_skating_correction(x); }
    if let Some(x) = args.floor_clip { builder.add_floor_clip(x); }
    if let Some(x) = args.force_arms_from_hmd { builder.add_force_arms_from_hmd(x); }
    if let Some(x) = args.extended_knee { builder.add_extended_knee(x); }
    if let Some(x) = args.extended_pelvis { builder.add_extended_pelvis(x); }
    if let Some(x) = args.extended_spine { builder.add_extended_spine(x); }
    builder.finish()
  }


  #[inline]
  pub fn extended_spine(&self) -> Option<bool> {
    self._tab.get::<bool>(ModelToggles::VT_EXTENDED_SPINE, None)
  }
  #[inline]
  pub fn extended_pelvis(&self) -> Option<bool> {
    self._tab.get::<bool>(ModelToggles::VT_EXTENDED_PELVIS, None)
  }
  #[inline]
  pub fn extended_knee(&self) -> Option<bool> {
    self._tab.get::<bool>(ModelToggles::VT_EXTENDED_KNEE, None)
  }
  #[inline]
  pub fn force_arms_from_hmd(&self) -> Option<bool> {
    self._tab.get::<bool>(ModelToggles::VT_FORCE_ARMS_FROM_HMD, None)
  }
  #[inline]
  pub fn floor_clip(&self) -> Option<bool> {
    self._tab.get::<bool>(ModelToggles::VT_FLOOR_CLIP, None)
  }
  #[inline]
  pub fn skating_correction(&self) -> Option<bool> {
    self._tab.get::<bool>(ModelToggles::VT_SKATING_CORRECTION, None)
  }
}

impl flatbuffers::Verifiable for ModelToggles<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("extended_spine", Self::VT_EXTENDED_SPINE, false)?
     .visit_field::<bool>("extended_pelvis", Self::VT_EXTENDED_PELVIS, false)?
     .visit_field::<bool>("extended_knee", Self::VT_EXTENDED_KNEE, false)?
     .visit_field::<bool>("force_arms_from_hmd", Self::VT_FORCE_ARMS_FROM_HMD, false)?
     .visit_field::<bool>("floor_clip", Self::VT_FLOOR_CLIP, false)?
     .visit_field::<bool>("skating_correction", Self::VT_SKATING_CORRECTION, false)?
     .finish();
    Ok(())
  }
}
pub struct ModelTogglesArgs {
    pub extended_spine: Option<bool>,
    pub extended_pelvis: Option<bool>,
    pub extended_knee: Option<bool>,
    pub force_arms_from_hmd: Option<bool>,
    pub floor_clip: Option<bool>,
    pub skating_correction: Option<bool>,
}
impl<'a> Default for ModelTogglesArgs {
  #[inline]
  fn default() -> Self {
    ModelTogglesArgs {
      extended_spine: None,
      extended_pelvis: None,
      extended_knee: None,
      force_arms_from_hmd: None,
      floor_clip: None,
      skating_correction: None,
    }
  }
}

pub struct ModelTogglesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ModelTogglesBuilder<'a, 'b> {
  #[inline]
  pub fn add_extended_spine(&mut self, extended_spine: bool) {
    self.fbb_.push_slot_always::<bool>(ModelToggles::VT_EXTENDED_SPINE, extended_spine);
  }
  #[inline]
  pub fn add_extended_pelvis(&mut self, extended_pelvis: bool) {
    self.fbb_.push_slot_always::<bool>(ModelToggles::VT_EXTENDED_PELVIS, extended_pelvis);
  }
  #[inline]
  pub fn add_extended_knee(&mut self, extended_knee: bool) {
    self.fbb_.push_slot_always::<bool>(ModelToggles::VT_EXTENDED_KNEE, extended_knee);
  }
  #[inline]
  pub fn add_force_arms_from_hmd(&mut self, force_arms_from_hmd: bool) {
    self.fbb_.push_slot_always::<bool>(ModelToggles::VT_FORCE_ARMS_FROM_HMD, force_arms_from_hmd);
  }
  #[inline]
  pub fn add_floor_clip(&mut self, floor_clip: bool) {
    self.fbb_.push_slot_always::<bool>(ModelToggles::VT_FLOOR_CLIP, floor_clip);
  }
  #[inline]
  pub fn add_skating_correction(&mut self, skating_correction: bool) {
    self.fbb_.push_slot_always::<bool>(ModelToggles::VT_SKATING_CORRECTION, skating_correction);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ModelTogglesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ModelTogglesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ModelToggles<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for ModelToggles<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("ModelToggles");
      ds.field("extended_spine", &self.extended_spine());
      ds.field("extended_pelvis", &self.extended_pelvis());
      ds.field("extended_knee", &self.extended_knee());
      ds.field("force_arms_from_hmd", &self.force_arms_from_hmd());
      ds.field("floor_clip", &self.floor_clip());
      ds.field("skating_correction", &self.skating_correction());
      ds.finish()
  }
}
