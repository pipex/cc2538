#[doc = "Reader of register SRCRESMASK1"]
pub type R = crate::R<u32, super::SRCRESMASK1>;
#[doc = "Writer for register SRCRESMASK1"]
pub type W = crate::W<u32, super::SRCRESMASK1>;
#[doc = "Register SRCRESMASK1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCRESMASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRCRESMASK1`"]
pub type SRCRESMASK1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCRESMASK1`"]
pub struct SRCRESMASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCRESMASK1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask1(&self) -> SRCRESMASK1_R {
        SRCRESMASK1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] Short address matching When there is a match on entry panid_n + short_n, bit n is set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask1(&mut self) -> SRCRESMASK1_W {
        SRCRESMASK1_W { w: self }
    }
}
