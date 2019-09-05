#[doc = "Reader of register DACTEST1"]
pub type R = crate::R<u32, super::DACTEST1>;
#[doc = "Writer for register DACTEST1"]
pub type W = crate::W<u32, super::DACTEST1>;
#[doc = "Register DACTEST1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DACTEST1 {
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
#[doc = "Reader of field `DAC_I_O`"]
pub type DAC_I_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_I_O`"]
pub struct DAC_I_O_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_I_O_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_i_o(&self) -> DAC_I_O_R {
        DAC_I_O_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_i_o(&mut self) -> DAC_I_O_W {
        DAC_I_O_W { w: self }
    }
}
