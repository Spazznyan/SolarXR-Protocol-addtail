// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum OverlayDisplayModeResponseOffset {}
#[derive(Copy, Clone, PartialEq)]

/// The current state of the overlay's display mode.
pub struct OverlayDisplayModeResponse<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for OverlayDisplayModeResponse<'a> {
  type Inner = OverlayDisplayModeResponse<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> OverlayDisplayModeResponse<'a> {
  pub const VT_IS_VISIBLE: flatbuffers::VOffsetT = 4;
  pub const VT_IS_MIRRORED: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    OverlayDisplayModeResponse { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args OverlayDisplayModeResponseArgs
  ) -> flatbuffers::WIPOffset<OverlayDisplayModeResponse<'bldr>> {
    let mut builder = OverlayDisplayModeResponseBuilder::new(_fbb);
    builder.add_is_mirrored(args.is_mirrored);
    builder.add_is_visible(args.is_visible);
    builder.finish()
  }


  #[inline]
  pub fn is_visible(&self) -> bool {
    self._tab.get::<bool>(OverlayDisplayModeResponse::VT_IS_VISIBLE, Some(false)).unwrap()
  }
  #[inline]
  pub fn is_mirrored(&self) -> bool {
    self._tab.get::<bool>(OverlayDisplayModeResponse::VT_IS_MIRRORED, Some(false)).unwrap()
  }
}

impl flatbuffers::Verifiable for OverlayDisplayModeResponse<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("is_visible", Self::VT_IS_VISIBLE, false)?
     .visit_field::<bool>("is_mirrored", Self::VT_IS_MIRRORED, false)?
     .finish();
    Ok(())
  }
}
pub struct OverlayDisplayModeResponseArgs {
    pub is_visible: bool,
    pub is_mirrored: bool,
}
impl<'a> Default for OverlayDisplayModeResponseArgs {
  #[inline]
  fn default() -> Self {
    OverlayDisplayModeResponseArgs {
      is_visible: false,
      is_mirrored: false,
    }
  }
}

pub struct OverlayDisplayModeResponseBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> OverlayDisplayModeResponseBuilder<'a, 'b> {
  #[inline]
  pub fn add_is_visible(&mut self, is_visible: bool) {
    self.fbb_.push_slot::<bool>(OverlayDisplayModeResponse::VT_IS_VISIBLE, is_visible, false);
  }
  #[inline]
  pub fn add_is_mirrored(&mut self, is_mirrored: bool) {
    self.fbb_.push_slot::<bool>(OverlayDisplayModeResponse::VT_IS_MIRRORED, is_mirrored, false);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> OverlayDisplayModeResponseBuilder<'a, 'b> {
    let start = _fbb.start_table();
    OverlayDisplayModeResponseBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<OverlayDisplayModeResponse<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for OverlayDisplayModeResponse<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("OverlayDisplayModeResponse");
      ds.field("is_visible", &self.is_visible());
      ds.field("is_mirrored", &self.is_mirrored());
      ds.finish()
  }
}
