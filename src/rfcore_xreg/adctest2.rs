#[doc = "Reader of register ADCTEST2"]
pub type R = crate::R<u32, super::ADCTEST2>;
#[doc = "Writer for register ADCTEST2"]
pub type W = crate::W<u32, super::ADCTEST2>;
#[doc = "Register ADCTEST2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCTEST2 {
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
#[doc = "Reader of field `ADC_TEST_MODE`"]
pub type ADC_TEST_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_TEST_MODE`"]
pub struct ADC_TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_TEST_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `AAF_RS`"]
pub type AAF_RS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AAF_RS`"]
pub struct AAF_RS_W<'a> {
    w: &'a mut W,
}
impl<'a> AAF_RS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `ADC_FF_ADJ`"]
pub type ADC_FF_ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_FF_ADJ`"]
pub struct ADC_FF_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_FF_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADC_DAC_ROT`"]
pub type ADC_DAC_ROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_DAC_ROT`"]
pub struct ADC_DAC_ROT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DAC_ROT_W<'a> {
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
    #[doc = "Bits 5:6 - 6:5\\] Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    pub fn adc_test_mode(&self) -> ADC_TEST_MODE_R {
        ADC_TEST_MODE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\] Controls series resistance of AAF"]
    #[inline(always)]
    pub fn aaf_rs(&self) -> AAF_RS_R {
        AAF_RS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - 2:1\\] Adjust feed forward"]
    #[inline(always)]
    pub fn adc_ff_adj(&self) -> ADC_FF_ADJ_R {
        ADC_FF_ADJ_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    pub fn adc_dac_rot(&self) -> ADC_DAC_ROT_R {
        ADC_DAC_ROT_R::new((self.bits & 0x01) != 0)
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
    #[doc = "Bits 5:6 - 6:5\\] Test mode to enable output of ADC data from demodulator. When enabled, raw ADC data is clocked out on the GPIO pins. 00: Test mode disabled 01: Data from the I and Q ADCs are output (data rate 76 MHz) 10: Data from the I ADC is output. Two and two ADC samples grouped (data rate 38 MHz) 11: Data from the Q ADC is output. Two and two ADC samples grouped (data rate 38 MHz)"]
    #[inline(always)]
    pub fn adc_test_mode(&mut self) -> ADC_TEST_MODE_W {
        ADC_TEST_MODE_W { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] Controls series resistance of AAF"]
    #[inline(always)]
    pub fn aaf_rs(&mut self) -> AAF_RS_W {
        AAF_RS_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] Adjust feed forward"]
    #[inline(always)]
    pub fn adc_ff_adj(&mut self) -> ADC_FF_ADJ_W {
        ADC_FF_ADJ_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Control of DAC DWA scheme 0 = DWA (scrambling) disabled 1 = DWA enabled"]
    #[inline(always)]
    pub fn adc_dac_rot(&mut self) -> ADC_DAC_ROT_W {
        ADC_DAC_ROT_W { w: self }
    }
}
