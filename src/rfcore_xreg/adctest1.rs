#[doc = "Reader of register ADCTEST1"]
pub type R = crate::R<u32, super::ADCTEST1>;
#[doc = "Writer for register ADCTEST1"]
pub type W = crate::W<u32, super::ADCTEST1>;
#[doc = "Register ADCTEST1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCTEST1 {
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
#[doc = "Reader of field `ADC_TEST_CTRL`"]
pub type ADC_TEST_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_TEST_CTRL`"]
pub struct ADC_TEST_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_TEST_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC_C2_ADJ`"]
pub type ADC_C2_ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_C2_ADJ`"]
pub struct ADC_C2_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_C2_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADC_C3_ADJ`"]
pub type ADC_C3_ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_C3_ADJ`"]
pub struct ADC_C3_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_C3_ADJ_W<'a> {
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
    #[doc = "Bits 4:7 - 7:4\\] ADC test mode selector"]
    #[inline(always)]
    pub fn adc_test_ctrl(&self) -> ADC_TEST_CTRL_R {
        ADC_TEST_CTRL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\] Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c2_adj(&self) -> ADC_C2_ADJ_R {
        ADC_C2_ADJ_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c3_adj(&self) -> ADC_C3_ADJ_R {
        ADC_C3_ADJ_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] ADC test mode selector"]
    #[inline(always)]
    pub fn adc_test_ctrl(&mut self) -> ADC_TEST_CTRL_W {
        ADC_TEST_CTRL_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c2_adj(&mut self) -> ADC_C2_ADJ_W {
        ADC_C2_ADJ_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Used to adjust capacitor values in ADC"]
    #[inline(always)]
    pub fn adc_c3_adj(&mut self) -> ADC_C3_ADJ_W {
        ADC_C3_ADJ_W { w: self }
    }
}
