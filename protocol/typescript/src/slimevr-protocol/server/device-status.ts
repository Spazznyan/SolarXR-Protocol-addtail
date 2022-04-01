// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { Quat, QuatT } from '../../slimevr-protocol/datatypes/quat';
import { Vec3f, Vec3fT } from '../../slimevr-protocol/datatypes/vec3f';
import { TrackerRole } from '../../slimevr-protocol/server/tracker-role';
import { TrackerStatus } from '../../slimevr-protocol/server/tracker-status';


export class DeviceStatus {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):DeviceStatus {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsDeviceStatus(bb:flatbuffers.ByteBuffer, obj?:DeviceStatus):DeviceStatus {
  return (obj || new DeviceStatus()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsDeviceStatus(bb:flatbuffers.ByteBuffer, obj?:DeviceStatus):DeviceStatus {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new DeviceStatus()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

name():string|null
name(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
name(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

role():TrackerRole {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : TrackerRole.NONE;
}

rotation(obj?:Quat):Quat|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? (obj || new Quat()).__init(this.bb_pos + offset, this.bb!) : null;
}

position(obj?:Vec3f):Vec3f|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? (obj || new Vec3f()).__init(this.bb_pos + offset, this.bb!) : null;
}

status():TrackerStatus {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : TrackerStatus.NONE;
}

battery():number {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : 0;
}

computed():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 16);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

signal():number {
  const offset = this.bb!.__offset(this.bb_pos, 18);
  return offset ? this.bb!.readInt16(this.bb_pos + offset) : 0;
}

tps():number {
  const offset = this.bb!.__offset(this.bb_pos, 20);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : 0;
}

ping():number {
  const offset = this.bb!.__offset(this.bb_pos, 22);
  return offset ? this.bb!.readUint16(this.bb_pos + offset) : 0;
}

static startDeviceStatus(builder:flatbuffers.Builder) {
  builder.startObject(10);
}

static addName(builder:flatbuffers.Builder, nameOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, nameOffset, 0);
}

static addRole(builder:flatbuffers.Builder, role:TrackerRole) {
  builder.addFieldInt8(1, role, TrackerRole.NONE);
}

static addRotation(builder:flatbuffers.Builder, rotationOffset:flatbuffers.Offset) {
  builder.addFieldStruct(2, rotationOffset, 0);
}

static addPosition(builder:flatbuffers.Builder, positionOffset:flatbuffers.Offset) {
  builder.addFieldStruct(3, positionOffset, 0);
}

static addStatus(builder:flatbuffers.Builder, status:TrackerStatus) {
  builder.addFieldInt8(4, status, TrackerStatus.NONE);
}

static addBattery(builder:flatbuffers.Builder, battery:number) {
  builder.addFieldInt8(5, battery, 0);
}

static addComputed(builder:flatbuffers.Builder, computed:boolean) {
  builder.addFieldInt8(6, +computed, +false);
}

static addSignal(builder:flatbuffers.Builder, signal:number) {
  builder.addFieldInt16(7, signal, 0);
}

static addTps(builder:flatbuffers.Builder, tps:number) {
  builder.addFieldInt8(8, tps, 0);
}

static addPing(builder:flatbuffers.Builder, ping:number) {
  builder.addFieldInt16(9, ping, 0);
}

static endDeviceStatus(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}


unpack(): DeviceStatusT {
  return new DeviceStatusT(
    this.name(),
    this.role(),
    (this.rotation() !== null ? this.rotation()!.unpack() : null),
    (this.position() !== null ? this.position()!.unpack() : null),
    this.status(),
    this.battery(),
    this.computed(),
    this.signal(),
    this.tps(),
    this.ping()
  );
}


unpackTo(_o: DeviceStatusT): void {
  _o.name = this.name();
  _o.role = this.role();
  _o.rotation = (this.rotation() !== null ? this.rotation()!.unpack() : null);
  _o.position = (this.position() !== null ? this.position()!.unpack() : null);
  _o.status = this.status();
  _o.battery = this.battery();
  _o.computed = this.computed();
  _o.signal = this.signal();
  _o.tps = this.tps();
  _o.ping = this.ping();
}
}

export class DeviceStatusT {
constructor(
  public name: string|Uint8Array|null = null,
  public role: TrackerRole = TrackerRole.NONE,
  public rotation: QuatT|null = null,
  public position: Vec3fT|null = null,
  public status: TrackerStatus = TrackerStatus.NONE,
  public battery: number = 0,
  public computed: boolean = false,
  public signal: number = 0,
  public tps: number = 0,
  public ping: number = 0
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const name = (this.name !== null ? builder.createString(this.name!) : 0);

  DeviceStatus.startDeviceStatus(builder);
  DeviceStatus.addName(builder, name);
  DeviceStatus.addRole(builder, this.role);
  DeviceStatus.addRotation(builder, (this.rotation !== null ? this.rotation!.pack(builder) : 0));
  DeviceStatus.addPosition(builder, (this.position !== null ? this.position!.pack(builder) : 0));
  DeviceStatus.addStatus(builder, this.status);
  DeviceStatus.addBattery(builder, this.battery);
  DeviceStatus.addComputed(builder, this.computed);
  DeviceStatus.addSignal(builder, this.signal);
  DeviceStatus.addTps(builder, this.tps);
  DeviceStatus.addPing(builder, this.ping);

  return DeviceStatus.endDeviceStatus(builder);
}
}