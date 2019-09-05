#[doc = "Reader of register CHIS"]
pub type R = crate::R<u32, super::CHIS>;
#[doc = "Writer for register CHIS"]
pub type W = crate::W<u32, super::CHIS>;
#[doc = "Register CHIS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHIS`"]
pub type CHIS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHIS`"]
pub struct CHIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn chis(&self) -> CHIS_R {
        CHIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn chis(&mut self) -> CHIS_W {
        CHIS_W { w: self }
    }
}
