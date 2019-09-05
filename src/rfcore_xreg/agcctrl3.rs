#[doc = "Reader of register AGCCTRL3"]
pub type R = crate::R<u32, super::AGCCTRL3>;
#[doc = "Writer for register AGCCTRL3"]
pub type W = crate::W<u32, super::AGCCTRL3>;
#[doc = "Register AGCCTRL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::AGCCTRL3 {
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
#[doc = "Reader of field `Reserved8`"]
pub type RESERVED8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
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
#[doc = "Reader of field `AGC_SETTLE_WAIT`"]
pub type AGC_SETTLE_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AGC_SETTLE_WAIT`"]
pub struct AGC_SETTLE_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_SETTLE_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `AGC_WIN_SIZE`"]
pub type AGC_WIN_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AGC_WIN_SIZE`"]
pub struct AGC_WIN_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_WIN_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `AAF_RP`"]
pub type AAF_RP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AAF_RP`"]
pub struct AAF_RP_W<'a> {
    w: &'a mut W,
}
impl<'a> AAF_RP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `AAF_RP_OE`"]
pub type AAF_RP_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AAF_RP_OE`"]
pub struct AAF_RP_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> AAF_RP_OE_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\] Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
    #[inline(always)]
    pub fn agc_settle_wait(&self) -> AGC_SETTLE_WAIT_R {
        AGC_SETTLE_WAIT_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\] Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
    #[inline(always)]
    pub fn agc_win_size(&self) -> AGC_WIN_SIZE_R {
        AGC_WIN_SIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - 2:1\\] Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
    #[inline(always)]
    pub fn aaf_rp(&self) -> AAF_RP_R {
        AAF_RP_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] Override the AAF control signals of the AGC with the values stored in AAF_RP."]
    #[inline(always)]
    pub fn aaf_rp_oe(&self) -> AAF_RP_OE_R {
        AAF_RP_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\] Timing for AGC to wait for analog gain to settle after a gain change. During this period, the energy measurement in the AGC is paused. 00: 15 periods 01: 20 periods 10: 25 periods 11: 30 periods"]
    #[inline(always)]
    pub fn agc_settle_wait(&mut self) -> AGC_SETTLE_WAIT_W {
        AGC_SETTLE_WAIT_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] Window size for the accumulate-and-dump function in the AGC. 00: 16 samples 01: 32 samples 10: 64 samples 11: 128 samples"]
    #[inline(always)]
    pub fn agc_win_size(&mut self) -> AGC_WIN_SIZE_W {
        AGC_WIN_SIZE_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] Overrides the control signals of the AGC to AAF when AAF_RP_OE = 1. When read, it returns the applied signal to the AAF. 00: 9-dB attenuation in AAF 01: 6-dB attenuation in AAF 10: 3-dB attenuation in AAF 11: 0-dB attenuation in AAF (reference level)"]
    #[inline(always)]
    pub fn aaf_rp(&mut self) -> AAF_RP_W {
        AAF_RP_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Override the AAF control signals of the AGC with the values stored in AAF_RP."]
    #[inline(always)]
    pub fn aaf_rp_oe(&mut self) -> AAF_RP_OE_W {
        AAF_RP_OE_W { w: self }
    }
}
