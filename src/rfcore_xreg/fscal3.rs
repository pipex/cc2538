#[doc = "Reader of register FSCAL3"]
pub type R = crate::R<u32, super::FSCAL3>;
#[doc = "Writer for register FSCAL3"]
pub type W = crate::W<u32, super::FSCAL3>;
#[doc = "Register FSCAL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSCAL3 {
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
#[doc = "Reader of field `VCO_DAC_EN_OV`"]
pub type VCO_DAC_EN_OV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCO_DAC_EN_OV`"]
pub struct VCO_DAC_EN_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_DAC_EN_OV_W<'a> {
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
#[doc = "Reader of field `VCO_VC_DAC`"]
pub type VCO_VC_DAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCO_VC_DAC`"]
pub struct VCO_VC_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_VC_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `VCO_CAPARR_CAL_CTRL`"]
pub type VCO_CAPARR_CAL_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCO_CAPARR_CAL_CTRL`"]
pub struct VCO_CAPARR_CAL_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CAPARR_CAL_CTRL_W<'a> {
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
    #[doc = "Bit 6 - 6:6\\] Enables the VCO DAC when 1"]
    #[inline(always)]
    pub fn vco_dac_en_ov(&self) -> VCO_DAC_EN_OV_R {
        VCO_DAC_EN_OV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - 5:2\\] Bit vector for programming varactor control voltage from VC DAC"]
    #[inline(always)]
    pub fn vco_vc_dac(&self) -> VCO_VC_DAC_R {
        VCO_VC_DAC_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
    #[inline(always)]
    pub fn vco_caparr_cal_ctrl(&self) -> VCO_CAPARR_CAL_CTRL_R {
        VCO_CAPARR_CAL_CTRL_R::new((self.bits & 0x03) as u8)
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
    #[doc = "Bit 6 - 6:6\\] Enables the VCO DAC when 1"]
    #[inline(always)]
    pub fn vco_dac_en_ov(&mut self) -> VCO_DAC_EN_OV_W {
        VCO_DAC_EN_OV_W { w: self }
    }
    #[doc = "Bits 2:5 - 5:2\\] Bit vector for programming varactor control voltage from VC DAC"]
    #[inline(always)]
    pub fn vco_vc_dac(&mut self) -> VCO_VC_DAC_W {
        VCO_VC_DAC_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
    #[inline(always)]
    pub fn vco_caparr_cal_ctrl(&mut self) -> VCO_CAPARR_CAL_CTRL_W {
        VCO_CAPARR_CAL_CTRL_W { w: self }
    }
}
