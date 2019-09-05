#[doc = "Reader of register TBV"]
pub type R = crate::R<u32, super::TBV>;
#[doc = "Writer for register TBV"]
pub type W = crate::W<u32, super::TBV>;
#[doc = "Register TBV `reset()`'s with value 0"]
impl crate::ResetValue for super::TBV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRE`"]
pub type PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE`"]
pub struct PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TBV`"]
pub type TBV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBV`"]
pub struct TBV_W<'a> {
    w: &'a mut W,
}
impl<'a> TBV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - 23:16\\] GPTM Timer B prescale register (16-bit mode)"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\] GPTM Timer B register"]
    #[inline(always)]
    pub fn tbv(&self) -> TBV_R {
        TBV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\] GPTM Timer B prescale register (16-bit mode)"]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W {
        PRE_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\] GPTM Timer B register"]
    #[inline(always)]
    pub fn tbv(&mut self) -> TBV_W {
        TBV_W { w: self }
    }
}
