#[doc = "Reader of register CCACTRL0"]
pub type R = crate::R<u32, super::CCACTRL0>;
#[doc = "Writer for register CCACTRL0"]
pub type W = crate::W<u32, super::CCACTRL0>;
#[doc = "Register CCACTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCACTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CCA_THR`"]
pub type CCA_THR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCA_THR`"]
pub struct CCA_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCA_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\] Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
    #[inline(always)]
    pub fn cca_thr(&self) -> CCA_THR_R {
        CCA_THR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
    #[inline(always)]
    pub fn cca_thr(&mut self) -> CCA_THR_W {
        CCA_THR_W { w: self }
    }
}
