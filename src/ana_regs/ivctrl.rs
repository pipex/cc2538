#[doc = "Reader of register IVCTRL"]
pub type R = crate::R<u32, super::IVCTRL>;
#[doc = "Writer for register IVCTRL"]
pub type W = crate::W<u32, super::IVCTRL>;
#[doc = "Register IVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::IVCTRL {
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
#[doc = "Reader of field `Reserved7`"]
pub type RESERVED7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
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
#[doc = "Reader of field `DAC_CURR_CTRL`"]
pub type DAC_CURR_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_CURR_CTRL`"]
pub struct DAC_CURR_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CURR_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LODIV_BIAS_CTRL`"]
pub type LODIV_BIAS_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LODIV_BIAS_CTRL`"]
pub struct LODIV_BIAS_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_BIAS_CTRL_W<'a> {
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
#[doc = "Reader of field `TXMIX_DC_CTRL`"]
pub type TXMIX_DC_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMIX_DC_CTRL`"]
pub struct TXMIX_DC_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIX_DC_CTRL_W<'a> {
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
#[doc = "Reader of field `PA_BIAS_CTRL`"]
pub type PA_BIAS_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PA_BIAS_CTRL`"]
pub struct PA_BIAS_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_BIAS_CTRL_W<'a> {
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
    #[doc = "Bit 6 - 6:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\] Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&self) -> DAC_CURR_CTRL_R {
        DAC_CURR_CTRL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&self) -> LODIV_BIAS_CTRL_R {
        LODIV_BIAS_CTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&self) -> TXMIX_DC_CTRL_R {
        TXMIX_DC_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\] Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&self) -> PA_BIAS_CTRL_R {
        PA_BIAS_CTRL_R::new((self.bits & 0x03) as u8)
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
    #[doc = "Bit 6 - 6:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&mut self) -> DAC_CURR_CTRL_W {
        DAC_CURR_CTRL_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&mut self) -> LODIV_BIAS_CTRL_W {
        LODIV_BIAS_CTRL_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&mut self) -> TXMIX_DC_CTRL_W {
        TXMIX_DC_CTRL_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&mut self) -> PA_BIAS_CTRL_W {
        PA_BIAS_CTRL_W { w: self }
    }
}
