#[doc = "Reader of register ADCL"]
pub type R = crate::R<u32, super::ADCL>;
#[doc = "Writer for register ADCL"]
pub type W = crate::W<u32, super::ADCL>;
#[doc = "Register ADCL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCL {
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
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC`"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
    #[doc = "Bits 2:7 - 7:2\\] Least-significant part of ADC conversion result"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Reserved. Always read as 0."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\] Least-significant part of ADC conversion result"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Reserved. Always read as 0."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
}
