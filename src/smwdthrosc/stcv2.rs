#[doc = "Reader of register STCV2"]
pub type R = crate::R<u32, super::STCV2>;
#[doc = "Writer for register STCV2"]
pub type W = crate::W<u32, super::STCV2>;
#[doc = "Register STCV2 `reset()`'s with value 0"]
impl crate::ResetValue for super::STCV2 {
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
#[doc = "Reader of field `STCV2`"]
pub type STCV2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STCV2`"]
pub struct STCV2_W<'a> {
    w: &'a mut W,
}
impl<'a> STCV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\] Bits \\[23:16\\] of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv2(&self) -> STCV2_R {
        STCV2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Bits \\[23:16\\] of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv2(&mut self) -> STCV2_W {
        STCV2_W { w: self }
    }
}
