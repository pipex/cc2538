#[doc = "Reader of register DACTEST2"]
pub type R = crate::R<u32, super::DACTEST2>;
#[doc = "Writer for register DACTEST2"]
pub type W = crate::W<u32, super::DACTEST2>;
#[doc = "Register DACTEST2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DACTEST2 {
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
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DAC_DEM_EN`"]
pub type DAC_DEM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_DEM_EN`"]
pub struct DAC_DEM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DEM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DAC_CASC_CTRL`"]
pub type DAC_CASC_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_CASC_CTRL`"]
pub struct DAC_CASC_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_CASC_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `DAC_SRC`"]
pub type DAC_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_SRC`"]
pub struct DAC_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\] Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&self) -> DAC_DEM_EN_R {
        DAC_DEM_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\] Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&self) -> DAC_CASC_CTRL_R {
        DAC_CASC_CTRL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\] The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&self) -> DAC_SRC_R {
        DAC_SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Enable and disable dynamic element matching Drives RFR_DAC_DEM_EN"]
    #[inline(always)]
    pub fn dac_dem_en(&mut self) -> DAC_DEM_EN_W {
        DAC_DEM_EN_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] Adjustment of output stage Drives RFR_DAC_CASC_CTRL"]
    #[inline(always)]
    pub fn dac_casc_ctrl(&mut self) -> DAC_CASC_CTRL_W {
        DAC_CASC_CTRL_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] The TX DACs data source is selected by DAC_SRC according to: 000: Normal operation (from modulator) 001: The DAC_I_O and DAC_Q_O override values 010: ADC data after decimation, magnitude controlled by DAC_I_O and DAC_Q_O 011: I/Q after decimation, channel and DC filtering, magnitude controlled by DAC_I_O and DAC_Q_O 100: CORDIC magnitude output and front-end gain is output, magnitude controlled by DAC_I_O and DAC_Q_O 101: RSSI I output on the I DAC 111: Reserved"]
    #[inline(always)]
    pub fn dac_src(&mut self) -> DAC_SRC_W {
        DAC_SRC_W { w: self }
    }
}
