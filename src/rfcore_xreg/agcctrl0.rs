#[doc = "Reader of register AGCCTRL0"]
pub type R = crate::R<u32, super::AGCCTRL0>;
#[doc = "Writer for register AGCCTRL0"]
pub type W = crate::W<u32, super::AGCCTRL0>;
#[doc = "Register AGCCTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::AGCCTRL0 {
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
#[doc = "Reader of field `AGC_DR_XTND_EN`"]
pub type AGC_DR_XTND_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AGC_DR_XTND_EN`"]
pub struct AGC_DR_XTND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_DR_XTND_EN_W<'a> {
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
#[doc = "Reader of field `AGC_DR_XTND_THR`"]
pub type AGC_DR_XTND_THR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AGC_DR_XTND_THR`"]
pub struct AGC_DR_XTND_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_DR_XTND_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
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
    #[doc = "Bit 6 - 6:6\\] 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
    #[inline(always)]
    pub fn agc_dr_xtnd_en(&self) -> AGC_DR_XTND_EN_R {
        AGC_DR_XTND_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - 5:0\\] If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
    #[inline(always)]
    pub fn agc_dr_xtnd_thr(&self) -> AGC_DR_XTND_THR_R {
        AGC_DR_XTND_THR_R::new((self.bits & 0x3f) as u8)
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
    #[doc = "Bit 6 - 6:6\\] 0: The AGC performs no adjustment of attenuation in the AAF. 1: The AGC adjusts the gain in the AAF to achieve extra dynamic range for the receiver."]
    #[inline(always)]
    pub fn agc_dr_xtnd_en(&mut self) -> AGC_DR_XTND_EN_W {
        AGC_DR_XTND_EN_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] If the measured error between the AGC reference magnitude and the actual magnitude in dB is larger than this threshold, the extra attenuation is enabled in the front end. This threshold must be set higher than 0x0C. This feature is enabled by AGC_DR_XTND_EN."]
    #[inline(always)]
    pub fn agc_dr_xtnd_thr(&mut self) -> AGC_DR_XTND_THR_W {
        AGC_DR_XTND_THR_W { w: self }
    }
}
