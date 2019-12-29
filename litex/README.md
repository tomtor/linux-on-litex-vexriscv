# Using Rust on Litex VexRiscv (Linux) #

Get started with https://github.com/litex-hub/linux-on-litex-vexriscv
and experiment with it until you are comfortable with using it.

## Enabling the C Riscv extension ##

Because the standard Riscv32 Rust compiler target is: `riscv32imac-unknown-none-elf`
and the default setup for VexRiscv has no `C` extension we need to change that.

See https://github.com/SpinalHDL/VexRiscv/issues/93 for a discussion on this subject.

To add the `C` extension you have to build a new VexRiscv variant:

https://github.com/tomtor/linux-on-litex-vexriscv#generating-the-vexriscv-linux-variant-optional

Apply the following diff before building with `sbt`.

```
diff --git a/src/main/scala/vexriscv/GenCoreDefault.scala b/src/main/scala/vexriscv/GenCoreDefault.scala
index a052205..9066e9e 100644
--- a/src/main/scala/vexriscv/GenCoreDefault.scala
+++ b/src/main/scala/vexriscv/GenCoreDefault.scala
@@ -92,6 +92,7 @@ object GenCoreDefault{
             resetVector = argConfig.resetVector,
             relaxedPcCalculation = argConfig.relaxedPcCalculation,
             prediction = argConfig.prediction,
+            compressedGen = true,
             memoryTranslatorPortConfig = if(linux) MmuPortConfig(portTlbSize = 4),
             config = InstructionCacheConfig(
               cacheSize = argConfig.iCacheSize,
@@ -149,7 +150,7 @@ object GenCoreDefault{
           catchIllegalInstruction = true
         ),
         new RegFilePlugin(
-          regFileReadyKind = plugin.SYNC,
+          regFileReadyKind = plugin.ASYNC,
           zeroBoot = false
         ),
         new IntAluPlugin,
@@ -268,4 +269,4 @@ class ForceRamBlockPhase() extends spinal.core.internals.Phase{
     }
   }
   override def hasNetlistImpact: Boolean = false

```

and copy the result `VexRiscv.v` to the `VexRiscv_Linux.v` of your `litex` tree (I assume `..`):

```
sudo cp VexRiscv.v ../litex/litex/soc/cores/cpu/vexriscv/verilog/VexRiscv_Linux.v
```

Verify that you can still boot the simulation in your `linux-on-litex-vexriscv` directory:
```
./sim.py
```

## Building a new root with the C Riscv extension ##

Clone `https://github.com/tomtor/linux-on-litex-vexriscv` and checkout branch `rv32imac`

This applies the following changes:

```
CONFIG_PACKET=y            # in buildroot/board/litex_vexriscv/linux.config for a working DHCP
BR2_RISCV_ISA_CUSTOM_RVC=y # in buildroot/configs/litex_vexriscv_defconfig Enable C compression
riscv,isa = "rv32imac";    # json2dts.py for a correct /proc/cpuinfo
```

Build the new root: https://github.com/tomtor/linux-on-litex-vexriscv#generating-the-linux-binaries-optional

Copy the result images (`Image` and `rootfs.cpio` to your `linux-on-litex-vexriscv/buildroot` directory
and test again with `./sim.py`).

## Building a new root with the C Riscv extension ##

