#[doc = "Reader of register CPSR"]
pub type R = crate::R<u32, super::CPSR>;
#[doc = "Writer for register CPSR"]
pub type W = crate::W<u32, super::CPSR>;
#[doc = "Register CPSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `Reserved16`"]
pub type RESERVED16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CPSDVSR`"]
pub type CPSDVSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPSDVSR`"]
pub struct CPSDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\] Reserved, read unpredictable, should be written as 0."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\] SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Reserved, read unpredictable, should be written as 0."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W {
        CPSDVSR_W { w: self }
    }
}
