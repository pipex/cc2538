#[doc = "Reader of register PD1_SEL"]
pub type R = crate::R<u32, super::PD1_SEL>;
#[doc = "Writer for register PD1_SEL"]
pub type W = crate::W<u32, super::PD1_SEL>;
#[doc = "Register PD1_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PD1_SEL {
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
#[doc = "Reader of field `PD1_sel`"]
pub type PD1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PD1_sel`"]
pub struct PD1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_SEL_W<'a> {
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
    #[doc = "Bits 0:4 - 4:0\\] Select one peripheral signal output for PD1."]
    #[inline(always)]
    pub fn pd1_sel(&self) -> PD1_SEL_R {
        PD1_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Select one peripheral signal output for PD1."]
    #[inline(always)]
    pub fn pd1_sel(&mut self) -> PD1_SEL_W {
        PD1_SEL_W { w: self }
    }
}
