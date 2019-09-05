#[doc = "Reader of register SWREQ"]
pub type R = crate::R<u32, super::SWREQ>;
#[doc = "Writer for register SWREQ"]
pub type W = crate::W<u32, super::SWREQ>;
#[doc = "Register SWREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SWREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWREQ`"]
pub type SWREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SWREQ`"]
pub struct SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] software request These bits generate software requests. Bit 0 corresponds to channel 0. 1: Generate a software request for the corresponding channel 0: No request generated These bits are automatically cleared when the software request has been completed."]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] software request These bits generate software requests. Bit 0 corresponds to channel 0. 1: Generate a software request for the corresponding channel 0: No request generated These bits are automatically cleared when the software request has been completed."]
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W {
        SWREQ_W { w: self }
    }
}
