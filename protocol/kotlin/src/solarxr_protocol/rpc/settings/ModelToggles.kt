// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc.settings

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * Settings for the skeletal model that are toggles.
 */
@Suppress("unused")
class ModelToggles : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : ModelToggles {
        __init(_i, _bb)
        return this
    }
    val extendedSpine : Boolean?
        get() {
            val o = __offset(4)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    val extendedPelvis : Boolean?
        get() {
            val o = __offset(6)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    val extendedKnee : Boolean?
        get() {
            val o = __offset(8)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    val forceArmsFromHmd : Boolean?
        get() {
            val o = __offset(10)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    val floorClip : Boolean?
        get() {
            val o = __offset(12)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    val skatingCorrection : Boolean?
        get() {
            val o = __offset(14)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    val viveEmulation : Boolean?
        get() {
            val o = __offset(16)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    val toeSnap : Boolean?
        get() {
            val o = __offset(18)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    val footPlant : Boolean?
        get() {
            val o = __offset(20)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else null
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsModelToggles(_bb: ByteBuffer): ModelToggles = getRootAsModelToggles(_bb, ModelToggles())
        @JvmStatic
        fun getRootAsModelToggles(_bb: ByteBuffer, obj: ModelToggles): ModelToggles {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createModelToggles(builder: FlatBufferBuilder, extendedSpine: Boolean?, extendedPelvis: Boolean?, extendedKnee: Boolean?, forceArmsFromHmd: Boolean?, floorClip: Boolean?, skatingCorrection: Boolean?, viveEmulation: Boolean?, toeSnap: Boolean?, footPlant: Boolean?) : Int {
            builder.startTable(9)
            footPlant?.run { addFootPlant(builder, footPlant) }
            toeSnap?.run { addToeSnap(builder, toeSnap) }
            viveEmulation?.run { addViveEmulation(builder, viveEmulation) }
            skatingCorrection?.run { addSkatingCorrection(builder, skatingCorrection) }
            floorClip?.run { addFloorClip(builder, floorClip) }
            forceArmsFromHmd?.run { addForceArmsFromHmd(builder, forceArmsFromHmd) }
            extendedKnee?.run { addExtendedKnee(builder, extendedKnee) }
            extendedPelvis?.run { addExtendedPelvis(builder, extendedPelvis) }
            extendedSpine?.run { addExtendedSpine(builder, extendedSpine) }
            return endModelToggles(builder)
        }
        @JvmStatic
        fun startModelToggles(builder: FlatBufferBuilder) = builder.startTable(9)
        @JvmStatic
        fun addExtendedSpine(builder: FlatBufferBuilder, extendedSpine: Boolean) = builder.addBoolean(0, extendedSpine, false)
        @JvmStatic
        fun addExtendedPelvis(builder: FlatBufferBuilder, extendedPelvis: Boolean) = builder.addBoolean(1, extendedPelvis, false)
        @JvmStatic
        fun addExtendedKnee(builder: FlatBufferBuilder, extendedKnee: Boolean) = builder.addBoolean(2, extendedKnee, false)
        @JvmStatic
        fun addForceArmsFromHmd(builder: FlatBufferBuilder, forceArmsFromHmd: Boolean) = builder.addBoolean(3, forceArmsFromHmd, false)
        @JvmStatic
        fun addFloorClip(builder: FlatBufferBuilder, floorClip: Boolean) = builder.addBoolean(4, floorClip, false)
        @JvmStatic
        fun addSkatingCorrection(builder: FlatBufferBuilder, skatingCorrection: Boolean) = builder.addBoolean(5, skatingCorrection, false)
        @JvmStatic
        fun addViveEmulation(builder: FlatBufferBuilder, viveEmulation: Boolean) = builder.addBoolean(6, viveEmulation, false)
        @JvmStatic
        fun addToeSnap(builder: FlatBufferBuilder, toeSnap: Boolean) = builder.addBoolean(7, toeSnap, false)
        @JvmStatic
        fun addFootPlant(builder: FlatBufferBuilder, footPlant: Boolean) = builder.addBoolean(8, footPlant, false)
        @JvmStatic
        fun endModelToggles(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
