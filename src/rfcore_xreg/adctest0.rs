#[doc = "Reader of register ADCTEST0"]
pub type R = crate::R<u32, super::ADCTEST0>;
#[doc = "Writer for register ADCTEST0"]
pub type W = crate::W<u32, super::ADCTEST0>;
#[doc = "Register ADCTEST0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCTEST0 {
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
#[doc = "Reader of field `ADC_VREF_ADJ`"]
pub type ADC_VREF_ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_VREF_ADJ`"]
pub struct ADC_VREF_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_VREF_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC_QUANT_ADJ`"]
pub type ADC_QUANT_ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_QUANT_ADJ`"]
pub struct ADC_QUANT_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_QUANT_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC_GM_ADJ`"]
pub type ADC_GM_ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_GM_ADJ`"]
pub struct ADC_GM_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_GM_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADC_DAC2_EN`"]
pub type ADC_DAC2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_DAC2_EN`"]
pub struct ADC_DAC2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DAC2_EN_W<'a> {
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
    #[doc = "Bits 6:7 - 7:6\\] Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_vref_adj(&self) -> ADC_VREF_ADJ_R {
        ADC_VREF_ADJ_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\] Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_quant_adj(&self) -> ADC_QUANT_ADJ_R {
        ADC_QUANT_ADJ_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 1:3 - 3:1\\] Gm-control for test and debug"]
    #[inline(always)]
    pub fn adc_gm_adj(&self) -> ADC_GM_ADJ_R {
        ADC_GM_ADJ_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] Enables DAC2 for enhanced ADC stability"]
    #[inline(always)]
    pub fn adc_dac2_en(&self) -> ADC_DAC2_EN_R {
        ADC_DAC2_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_vref_adj(&mut self) -> ADC_VREF_ADJ_W {
        ADC_VREF_ADJ_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Quantizer threshold control for test and debug"]
    #[inline(always)]
    pub fn adc_quant_adj(&mut self) -> ADC_QUANT_ADJ_W {
        ADC_QUANT_ADJ_W { w: self }
    }
    #[doc = "Bits 1:3 - 3:1\\] Gm-control for test and debug"]
    #[inline(always)]
    pub fn adc_gm_adj(&mut self) -> ADC_GM_ADJ_W {
        ADC_GM_ADJ_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enables DAC2 for enhanced ADC stability"]
    #[inline(always)]
    pub fn adc_dac2_en(&mut self) -> ADC_DAC2_EN_W {
        ADC_DAC2_EN_W { w: self }
    }
}
