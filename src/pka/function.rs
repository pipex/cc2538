#[doc = "Reader of register FUNCTION"]
pub type R = crate::R<u32, super::FUNCTION>;
#[doc = "Writer for register FUNCTION"]
pub type W = crate::W<u32, super::FUNCTION>;
#[doc = "Register FUNCTION `reset()`'s with value 0"]
impl crate::ResetValue for super::FUNCTION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `STALL_RESULT`"]
pub type STALL_RESULT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL_RESULT`"]
pub struct STALL_RESULT_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_RESULT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUN`"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SEQUENCER_OPERATIONS`"]
pub type SEQUENCER_OPERATIONS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQUENCER_OPERATIONS`"]
pub struct SEQUENCER_OPERATIONS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQUENCER_OPERATIONS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `COPY`"]
pub type COPY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COPY`"]
pub struct COPY_W<'a> {
    w: &'a mut W,
}
impl<'a> COPY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `COMPARE`"]
pub type COMPARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPARE`"]
pub struct COMPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MODULO`"]
pub type MODULO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODULO`"]
pub struct MODULO_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DIVIDE`"]
pub type DIVIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVIDE`"]
pub struct DIVIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `LSHIFT`"]
pub type LSHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSHIFT`"]
pub struct LSHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSHIFT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RSHIFT`"]
pub type RSHIFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSHIFT`"]
pub struct RSHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SUBTRACT`"]
pub type SUBTRACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUBTRACT`"]
pub struct SUBTRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBTRACT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ADD`"]
pub type ADD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD`"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MS_ONE`"]
pub type MS_ONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MS_ONE`"]
pub struct MS_ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_ONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `Reserved3`"]
pub type RESERVED3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADDSUB`"]
pub type ADDSUB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDSUB`"]
pub struct ADDSUB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSUB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MULTIPLY`"]
pub type MULTIPLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MULTIPLY`"]
pub struct MULTIPLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTIPLY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\] Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\] When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
    #[inline(always)]
    pub fn stall_result(&self) -> STALL_RESULT_R {
        STALL_RESULT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\] Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - 15:15\\] The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\] These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
    #[inline(always)]
    pub fn sequencer_operations(&self) -> SEQUENCER_OPERATIONS_R {
        SEQUENCER_OPERATIONS_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - 11:11\\] Perform copy operation"]
    #[inline(always)]
    pub fn copy(&self) -> COPY_R {
        COPY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] Perform compare operation"]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] Perform modulo operation"]
    #[inline(always)]
    pub fn modulo(&self) -> MODULO_R {
        MODULO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] Perform divide operation"]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] Perform left shift operation"]
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Perform right shift operation"]
    #[inline(always)]
    pub fn rshift(&self) -> RSHIFT_R {
        RSHIFT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Perform subtract operation"]
    #[inline(always)]
    pub fn subtract(&self) -> SUBTRACT_R {
        SUBTRACT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Perform add operation"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\] of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
    #[inline(always)]
    pub fn ms_one(&self) -> MS_ONE_R {
        MS_ONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Perform combined add/subtract operation"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Perform multiply operation"]
    #[inline(always)]
    pub fn multiply(&self) -> MULTIPLY_R {
        MULTIPLY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\] Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
    #[inline(always)]
    pub fn stall_result(&mut self) -> STALL_RESULT_W {
        STALL_RESULT_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\] These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
    #[inline(always)]
    pub fn sequencer_operations(&mut self) -> SEQUENCER_OPERATIONS_W {
        SEQUENCER_OPERATIONS_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Perform copy operation"]
    #[inline(always)]
    pub fn copy(&mut self) -> COPY_W {
        COPY_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Perform compare operation"]
    #[inline(always)]
    pub fn compare(&mut self) -> COMPARE_W {
        COMPARE_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Perform modulo operation"]
    #[inline(always)]
    pub fn modulo(&mut self) -> MODULO_W {
        MODULO_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Perform divide operation"]
    #[inline(always)]
    pub fn divide(&mut self) -> DIVIDE_W {
        DIVIDE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Perform left shift operation"]
    #[inline(always)]
    pub fn lshift(&mut self) -> LSHIFT_W {
        LSHIFT_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Perform right shift operation"]
    #[inline(always)]
    pub fn rshift(&mut self) -> RSHIFT_W {
        RSHIFT_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Perform subtract operation"]
    #[inline(always)]
    pub fn subtract(&mut self) -> SUBTRACT_W {
        SUBTRACT_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Perform add operation"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\] of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
    #[inline(always)]
    pub fn ms_one(&mut self) -> MS_ONE_W {
        MS_ONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Perform combined add/subtract operation"]
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W {
        ADDSUB_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Perform multiply operation"]
    #[inline(always)]
    pub fn multiply(&mut self) -> MULTIPLY_W {
        MULTIPLY_W { w: self }
    }
}
