#[doc = "Reader of register MDMCTRL0"]
pub type R = crate::R<u32, super::MDMCTRL0>;
#[doc = "Writer for register MDMCTRL0"]
pub type W = crate::W<u32, super::MDMCTRL0>;
#[doc = "Register MDMCTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMCTRL0 {
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
#[doc = "Reader of field `DEM_NUM_ZEROS`"]
pub type DEM_NUM_ZEROS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEM_NUM_ZEROS`"]
pub struct DEM_NUM_ZEROS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEM_NUM_ZEROS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DEMOD_AVG_MODE`"]
pub type DEMOD_AVG_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEMOD_AVG_MODE`"]
pub struct DEMOD_AVG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEMOD_AVG_MODE_W<'a> {
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
#[doc = "Reader of field `PREAMBLE_LENGTH`"]
pub type PREAMBLE_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PREAMBLE_LENGTH`"]
pub struct PREAMBLE_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PREAMBLE_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `TX_FILTER`"]
pub type TX_FILTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FILTER`"]
pub struct TX_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FILTER_W<'a> {
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
    #[doc = "Bits 6:7 - 7:6\\] Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
    #[inline(always)]
    pub fn dem_num_zeros(&self) -> DEM_NUM_ZEROS_R {
        DEM_NUM_ZEROS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\] Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
    #[inline(always)]
    pub fn demod_avg_mode(&self) -> DEMOD_AVG_MODE_R {
        DEMOD_AVG_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\] The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
    #[inline(always)]
    pub fn preamble_length(&self) -> PREAMBLE_LENGTH_R {
        PREAMBLE_LENGTH_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
    #[inline(always)]
    pub fn tx_filter(&self) -> TX_FILTER_R {
        TX_FILTER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Sets how many zero symbols must be detected before the sync word when searching for sync. Only one zero symbol is required to have a correlation value above the correlation threshold set in the MDMCTRL1 register. 00: Reserved 01: 1 zero symbol 10: 2 zero symbols 11: 3 zero symbols"]
    #[inline(always)]
    pub fn dem_num_zeros(&mut self) -> DEM_NUM_ZEROS_W {
        DEM_NUM_ZEROS_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Defines the behavior or the frequency offset averaging filter. 0: Lock average level after preamble match. Restart frequency offset calibration when searching for the next frame. 1: Continuously update average level."]
    #[inline(always)]
    pub fn demod_avg_mode(&mut self) -> DEMOD_AVG_MODE_W {
        DEMOD_AVG_MODE_W { w: self }
    }
    #[doc = "Bits 1:4 - 4:1\\] The number of preamble bytes (two zero-symbols) to be sent in TX mode before the SFD, encoded in steps of 2 symbols (1 byte). The reset value of 2 is compliant with IEEE 802.15.4. 0000: 2 leading-zero bytes 0001: 3 leading-zero bytes 0010: 4 leading-zero bytes ... 1111: 17 leading-zero bytes"]
    #[inline(always)]
    pub fn preamble_length(&mut self) -> PREAMBLE_LENGTH_W {
        PREAMBLE_LENGTH_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Defines the kind of TX filter that is used. The normal TX filter is as defined by the IEEE 802.15.4 standard. Extra filtering may be applied to lower the out-of-band emissions. 0: Normal TX filtering 1: Enable extra filtering"]
    #[inline(always)]
    pub fn tx_filter(&mut self) -> TX_FILTER_W {
        TX_FILTER_W { w: self }
    }
}
