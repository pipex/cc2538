#[doc = "Reader of register SRCSHORTEN2"]
pub type R = crate::R<u32, super::SRCSHORTEN2>;
#[doc = "Writer for register SRCSHORTEN2"]
pub type W = crate::W<u32, super::SRCSHORTEN2>;
#[doc = "Register SRCSHORTEN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCSHORTEN2 {
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
#[doc = "Reader of field `SHORT_ADDR_EN`"]
pub type SHORT_ADDR_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHORT_ADDR_EN`"]
pub struct SHORT_ADDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORT_ADDR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\] 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
    #[inline(always)]
    pub fn short_addr_en(&self) -> SHORT_ADDR_EN_R {
        SHORT_ADDR_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
    #[inline(always)]
    pub fn short_addr_en(&mut self) -> SHORT_ADDR_EN_W {
        SHORT_ADDR_EN_W { w: self }
    }
}
