#[doc = "Reader of register FSCAL1"]
pub type R = crate::R<u32, super::FSCAL1>;
#[doc = "Writer for register FSCAL1"]
pub type W = crate::W<u32, super::FSCAL1>;
#[doc = "Register FSCAL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSCAL1 {
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
#[doc = "Reader of field `VCO_CURR_CAL_OE`"]
pub type VCO_CURR_CAL_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCO_CURR_CAL_OE`"]
pub struct VCO_CURR_CAL_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CURR_CAL_OE_W<'a> {
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
#[doc = "Reader of field `VCO_CURR_CAL`"]
pub type VCO_CURR_CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCO_CURR_CAL`"]
pub struct VCO_CURR_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CURR_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `VCO_CURR`"]
pub type VCO_CURR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCO_CURR`"]
pub struct VCO_CURR_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CURR_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Override current calibration"]
    #[inline(always)]
    pub fn vco_curr_cal_oe(&self) -> VCO_CURR_CAL_OE_R {
        VCO_CURR_CAL_OE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - 6:2\\] Calibration result Override value if VCO_CURR_CAL_OE = 1"]
    #[inline(always)]
    pub fn vco_curr_cal(&self) -> VCO_CURR_CAL_R {
        VCO_CURR_CAL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
    #[inline(always)]
    pub fn vco_curr(&self) -> VCO_CURR_R {
        VCO_CURR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Override current calibration"]
    #[inline(always)]
    pub fn vco_curr_cal_oe(&mut self) -> VCO_CURR_CAL_OE_W {
        VCO_CURR_CAL_OE_W { w: self }
    }
    #[doc = "Bits 2:6 - 6:2\\] Calibration result Override value if VCO_CURR_CAL_OE = 1"]
    #[inline(always)]
    pub fn vco_curr_cal(&mut self) -> VCO_CURR_CAL_W {
        VCO_CURR_CAL_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
    #[inline(always)]
    pub fn vco_curr(&mut self) -> VCO_CURR_W {
        VCO_CURR_W { w: self }
    }
}
