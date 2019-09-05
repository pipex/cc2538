#[doc = "Reader of register SRCRESMASK2"]
pub type R = crate::R<u32, super::SRCRESMASK2>;
#[doc = "Writer for register SRCRESMASK2"]
pub type W = crate::W<u32, super::SRCRESMASK2>;
#[doc = "Register SRCRESMASK2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCRESMASK2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRCRESMASK2`"]
pub type SRCRESMASK2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCRESMASK2`"]
pub struct SRCRESMASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCRESMASK2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] 24-bit mask that indicates source address match for each individual entry in the source address table"]
    #[inline(always)]
    pub fn srcresmask2(&self) -> SRCRESMASK2_R {
        SRCRESMASK2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] 24-bit mask that indicates source address match for each individual entry in the source address table"]
    #[inline(always)]
    pub fn srcresmask2(&mut self) -> SRCRESMASK2_W {
        SRCRESMASK2_W { w: self }
    }
}
