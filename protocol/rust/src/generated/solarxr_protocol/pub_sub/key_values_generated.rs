// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum KeyValuesOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct KeyValues<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for KeyValues<'a> {
  type Inner = KeyValues<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> KeyValues<'a> {
  pub const VT_KEYS: flatbuffers::VOffsetT = 4;
  pub const VT_VALUES: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    KeyValues { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args KeyValuesArgs<'args>
  ) -> flatbuffers::WIPOffset<KeyValues<'bldr>> {
    let mut builder = KeyValuesBuilder::new(_fbb);
    if let Some(x) = args.values { builder.add_values(x); }
    if let Some(x) = args.keys { builder.add_keys(x); }
    builder.finish()
  }


  #[inline]
  pub fn keys(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(KeyValues::VT_KEYS, None)
  }
  #[inline]
  pub fn values(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(KeyValues::VT_VALUES, None)
  }
}

impl flatbuffers::Verifiable for KeyValues<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>("keys", Self::VT_KEYS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>("values", Self::VT_VALUES, false)?
     .finish();
    Ok(())
  }
}
pub struct KeyValuesArgs<'a> {
    pub keys: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
    pub values: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
}
impl<'a> Default for KeyValuesArgs<'a> {
  #[inline]
  fn default() -> Self {
    KeyValuesArgs {
      keys: None,
      values: None,
    }
  }
}

pub struct KeyValuesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> KeyValuesBuilder<'a, 'b> {
  #[inline]
  pub fn add_keys(&mut self, keys: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValues::VT_KEYS, keys);
  }
  #[inline]
  pub fn add_values(&mut self, values: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValues::VT_VALUES, values);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> KeyValuesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    KeyValuesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<KeyValues<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for KeyValues<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("KeyValues");
      ds.field("keys", &self.keys());
      ds.field("values", &self.values());
      ds.finish()
  }
}
