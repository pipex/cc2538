#[doc = "Reader of register TXCTRL"]
pub type R = crate::R<u32, super::TXCTRL>;
#[doc = "Writer for register TXCTRL"]
pub type W = crate::W<u32, super::TXCTRL>;
#[doc = "Register TXCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TXCTRL {
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
#[doc = "Reader of field `DAC_CURR`"]
pub type DAC_CURR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_CURR`"]
pub struct DAC_CURR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CURR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DAC_DC`"]
pub type DAC_DC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_DC`"]
pub struct DAC_DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXMIX_CURRENT`"]
pub type TXMIX_CURRENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXMIX_CURRENT`"]
pub struct TXMIX_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIX_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
    #[doc = "Bits 4:6 - 6:4\\] Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&self) -> DAC_CURR_R {
        DAC_CURR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\] Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&self) -> DAC_DC_R {
        DAC_DC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&self) -> TXMIX_CURRENT_R {
        TXMIX_CURRENT_R::new((self.bits & 0x03) as u8)
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
    #[doc = "Bits 4:6 - 6:4\\] Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&mut self) -> DAC_CURR_W {
        DAC_CURR_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&mut self) -> DAC_DC_W {
        DAC_DC_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&mut self) -> TXMIX_CURRENT_W {
        TXMIX_CURRENT_W { w: self }
    }
}
