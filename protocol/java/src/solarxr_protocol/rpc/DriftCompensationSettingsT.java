// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class DriftCompensationSettingsT {
  private boolean enabled;
  private float amount;
  private int maxResets;

  public boolean getEnabled() { return enabled; }

  public void setEnabled(boolean enabled) { this.enabled = enabled; }

  public float getAmount() { return amount; }

  public void setAmount(float amount) { this.amount = amount; }

  public int getMaxResets() { return maxResets; }

  public void setMaxResets(int maxResets) { this.maxResets = maxResets; }


  public DriftCompensationSettingsT() {
    this.enabled = false;
    this.amount = 0.0f;
    this.maxResets = 0;
  }
}
