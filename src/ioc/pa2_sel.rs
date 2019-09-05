#[doc = "Reader of register PA2_SEL"]
pub type R = crate::R<u32, super::PA2_SEL>;
#[doc = "Writer for register PA2_SEL"]
pub type W = crate::W<u32, super::PA2_SEL>;
#[doc = "Register PA2_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PA2_SEL {
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
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `PA2_sel`"]
pub type PA2_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PA2_sel`"]
pub struct PA2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PA2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - 31:5\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bits 0:4 - 4:0\\] Select one peripheral signal output for PA2."]
    #[inline(always)]
    pub fn pa2_sel(&self) -> PA2_SEL_R {
        PA2_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Select one peripheral signal output for PA2."]
    #[inline(always)]
    pub fn pa2_sel(&mut self) -> PA2_SEL_W {
        PA2_SEL_W { w: self }
    }
}