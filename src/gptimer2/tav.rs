#[doc = "Reader of register TAV"]
pub type R = crate::R<u32, super::TAV>;
#[doc = "Writer for register TAV"]
pub type W = crate::W<u32, super::TAV>;
#[doc = "Register TAV `reset()`'s with value 0"]
impl crate::ResetValue for super::TAV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAV`"]
pub type TAV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TAV`"]
pub struct TAV_W<'a> {
    w: &'a mut W,
}
impl<'a> TAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] GPTM Timer A register"]
    #[inline(always)]
    pub fn tav(&self) -> TAV_R {
        TAV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] GPTM Timer A register"]
    #[inline(always)]
    pub fn tav(&mut self) -> TAV_W {
        TAV_W { w: self }
    }
}
