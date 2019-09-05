#[doc = "Reader of register TXPOWER"]
pub type R = crate::R<u32, super::TXPOWER>;
#[doc = "Writer for register TXPOWER"]
pub type W = crate::W<u32, super::TXPOWER>;
#[doc = "Register TXPOWER `reset()`'s with value 0"]
impl crate::ResetValue for super::TXPOWER {
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
#[doc = "Reader of field `PA_POWER`"]
pub type PA_POWER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PA_POWER`"]
pub struct PA_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_POWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PA_BIAS`"]
pub type PA_BIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PA_BIAS`"]
pub struct PA_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 4:7 - 7:4\\] PA power control"]
    #[inline(always)]
    pub fn pa_power(&self) -> PA_POWER_R {
        PA_POWER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\] PA bias control"]
    #[inline(always)]
    pub fn pa_bias(&self) -> PA_BIAS_R {
        PA_BIAS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] PA power control"]
    #[inline(always)]
    pub fn pa_power(&mut self) -> PA_POWER_W {
        PA_POWER_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] PA bias control"]
    #[inline(always)]
    pub fn pa_bias(&mut self) -> PA_BIAS_W {
        PA_BIAS_W { w: self }
    }
}
