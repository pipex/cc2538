#[doc = "Reader of register AGCCTRL2"]
pub type R = crate::R<u32, super::AGCCTRL2>;
#[doc = "Writer for register AGCCTRL2"]
pub type W = crate::W<u32, super::AGCCTRL2>;
#[doc = "Register AGCCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AGCCTRL2 {
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
#[doc = "Reader of field `LNA1_CURRENT`"]
pub type LNA1_CURRENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNA1_CURRENT`"]
pub struct LNA1_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA1_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `LNA2_CURRENT`"]
pub type LNA2_CURRENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNA2_CURRENT`"]
pub struct LNA2_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA2_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `LNA3_CURRENT`"]
pub type LNA3_CURRENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNA3_CURRENT`"]
pub struct LNA3_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA3_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `LNA_CURRENT_OE`"]
pub type LNA_CURRENT_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNA_CURRENT_OE`"]
pub struct LNA_CURRENT_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CURRENT_OE_W<'a> {
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
    #[doc = "Bits 6:7 - 7:6\\] Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    pub fn lna1_current(&self) -> LNA1_CURRENT_R {
        LNA1_CURRENT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\] Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    pub fn lna2_current(&self) -> LNA2_CURRENT_R {
        LNA2_CURRENT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 1:2 - 2:1\\] Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    pub fn lna3_current(&self) -> LNA3_CURRENT_R {
        LNA3_CURRENT_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
    #[inline(always)]
    pub fn lna_current_oe(&self) -> LNA_CURRENT_OE_R {
        LNA_CURRENT_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Overrride value for LNA 1 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: Reserved 11: 6-dB gain"]
    #[inline(always)]
    pub fn lna1_current(&mut self) -> LNA1_CURRENT_W {
        LNA1_CURRENT_W { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\] Overrride value for LNA 2 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 000: 0-dB gain (reference level) 001: 3-dB gain 010: 6-dB gain 011: 9-dB gain 100: 12-dB gain 101: 15-dB gain 110: 18-dB gain 111: 21-dB gain"]
    #[inline(always)]
    pub fn lna2_current(&mut self) -> LNA2_CURRENT_W {
        LNA2_CURRENT_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] Overrride value for LNA 3 Used only when LNA_CURRENT_OE = 1 When read, this register returns the current applied gain setting. 00: 0-dB gain (reference level) 01: 3-dB gain 10: 6-dB gain 11: 9-dB gain"]
    #[inline(always)]
    pub fn lna3_current(&mut self) -> LNA3_CURRENT_W {
        LNA3_CURRENT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Write 1 to override the AGC LNA current setting with the values above (LNA1_CURRENT, LNA2_CURRENT, and LNA3_CURRENT)."]
    #[inline(always)]
    pub fn lna_current_oe(&mut self) -> LNA_CURRENT_OE_W {
        LNA_CURRENT_OE_W { w: self }
    }
}
