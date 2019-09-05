#[doc = "Reader of register ST1"]
pub type R = crate::R<u32, super::ST1>;
#[doc = "Writer for register ST1"]
pub type W = crate::W<u32, super::ST1>;
#[doc = "Register ST1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ST1 {
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
#[doc = "Reader of field `ST1`"]
pub type ST1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ST1`"]
pub struct ST1_W<'a> {
    w: &'a mut W,
}
impl<'a> ST1_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\] of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\] of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st1(&self) -> ST1_R {
        ST1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Sleep Timer count and compare value When read, this register returns the middle bits \\[15:8\\] of the Sleep Timer count. When writing this register sets the middle bits \\[15:8\\] of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st1(&mut self) -> ST1_W {
        ST1_W { w: self }
    }
}
