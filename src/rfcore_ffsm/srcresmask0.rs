#[doc = "Reader of register SRCRESMASK0"]
pub type R = crate::R<u32, super::SRCRESMASK0>;
#[doc = "Writer for register SRCRESMASK0"]
pub type W = crate::W<u32, super::SRCRESMASK0>;
#[doc = "Register SRCRESMASK0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCRESMASK0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRCRESMASK0`"]
pub type SRCRESMASK0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCRESMASK0`"]
pub struct SRCRESMASK0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCRESMASK0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask0(&self) -> SRCRESMASK0_R {
        SRCRESMASK0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] Extended address matching When there is a match on entry ext_n, bits 2n and 2n + 1 are set in SRCRESMASK."]
    #[inline(always)]
    pub fn srcresmask0(&mut self) -> SRCRESMASK0_W {
        SRCRESMASK0_W { w: self }
    }
}
