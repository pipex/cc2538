#[doc = "Reader of register FSCAL0"]
pub type R = crate::R<u32, super::FSCAL0>;
#[doc = "Writer for register FSCAL0"]
pub type W = crate::W<u32, super::FSCAL0>;
#[doc = "Register FSCAL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSCAL0 {
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
#[doc = "Reader of field `VCO_CURR_COMP_EN_OV`"]
pub type VCO_CURR_COMP_EN_OV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCO_CURR_COMP_EN_OV`"]
pub struct VCO_CURR_COMP_EN_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_CURR_COMP_EN_OV_W<'a> {
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
#[doc = "Reader of field `CHP_DISABLE`"]
pub type CHP_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHP_DISABLE`"]
pub struct CHP_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP_DISABLE_W<'a> {
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
#[doc = "Reader of field `CHP_CURRENT`"]
pub type CHP_CURRENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHP_CURRENT`"]
pub struct CHP_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `BW_BOOST_MODE`"]
pub type BW_BOOST_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BW_BOOST_MODE`"]
pub struct BW_BOOST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BW_BOOST_MODE_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
    #[inline(always)]
    pub fn vco_curr_comp_en_ov(&self) -> VCO_CURR_COMP_EN_OV_R {
        VCO_CURR_COMP_EN_OV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
    #[inline(always)]
    pub fn chp_disable(&self) -> CHP_DISABLE_R {
        CHP_DISABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - 5:2\\] Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
    #[inline(always)]
    pub fn chp_current(&self) -> CHP_CURRENT_R {
        CHP_CURRENT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
    #[inline(always)]
    pub fn bw_boost_mode(&self) -> BW_BOOST_MODE_R {
        BW_BOOST_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
    #[inline(always)]
    pub fn vco_curr_comp_en_ov(&mut self) -> VCO_CURR_COMP_EN_OV_W {
        VCO_CURR_COMP_EN_OV_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
    #[inline(always)]
    pub fn chp_disable(&mut self) -> CHP_DISABLE_W {
        CHP_DISABLE_W { w: self }
    }
    #[doc = "Bits 2:5 - 5:2\\] Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
    #[inline(always)]
    pub fn chp_current(&mut self) -> CHP_CURRENT_W {
        CHP_CURRENT_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
    #[inline(always)]
    pub fn bw_boost_mode(&mut self) -> BW_BOOST_MODE_W {
        BW_BOOST_MODE_W { w: self }
    }
}
