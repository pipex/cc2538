#[doc = "Reader of register ST2"]
pub type R = crate::R<u32, super::ST2>;
#[doc = "Writer for register ST2"]
pub type W = crate::W<u32, super::ST2>;
#[doc = "Register ST2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ST2 {
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
#[doc = "Reader of field `ST2`"]
pub type ST2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ST2`"]
pub struct ST2_W<'a> {
    w: &'a mut W,
}
impl<'a> ST2_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\] of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\] of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st2(&self) -> ST2_R {
        ST2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Sleep Timer count and compare value When read, this register returns the high bits \\[23:16\\] of the Sleep Timer count. When writing this register sets the high bits \\[23:16\\] of the compare value. The value read is latched at the time of reading register ST0. The value written is latched when ST0 is written."]
    #[inline(always)]
    pub fn st2(&mut self) -> ST2_W {
        ST2_W { w: self }
    }
}
