#[doc = "Reader of register STCV3"]
pub type R = crate::R<u32, super::STCV3>;
#[doc = "Writer for register STCV3"]
pub type W = crate::W<u32, super::STCV3>;
#[doc = "Register STCV3 `reset()`'s with value 0"]
impl crate::ResetValue for super::STCV3 {
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
#[doc = "Reader of field `STCV3`"]
pub type STCV3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STCV3`"]
pub struct STCV3_W<'a> {
    w: &'a mut W,
}
impl<'a> STCV3_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] Bits \\[32:24\\] of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv3(&self) -> STCV3_R {
        STCV3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Bits \\[32:24\\] of Sleep Timer capture value"]
    #[inline(always)]
    pub fn stcv3(&mut self) -> STCV3_W {
        STCV3_W { w: self }
    }
}
