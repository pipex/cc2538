#[doc = "Reader of register WAITSTAT"]
pub type R = crate::R<u32, super::WAITSTAT>;
#[doc = "Writer for register WAITSTAT"]
pub type W = crate::W<u32, super::WAITSTAT>;
#[doc = "Register WAITSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::WAITSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAITREQ`"]
pub type WAITREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WAITREQ`"]
pub struct WAITREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] wait status These bits provide the tchannel wait-on-request status. Bit 0 corresponds to channel 0. 1: The corresponding channel is waiting on a request. 0: The corresponding channel is not waiting on a request."]
    #[inline(always)]
    pub fn waitreq(&self) -> WAITREQ_R {
        WAITREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] wait status These bits provide the tchannel wait-on-request status. Bit 0 corresponds to channel 0. 1: The corresponding channel is waiting on a request. 0: The corresponding channel is not waiting on a request."]
    #[inline(always)]
    pub fn waitreq(&mut self) -> WAITREQ_W {
        WAITREQ_W { w: self }
    }
}
