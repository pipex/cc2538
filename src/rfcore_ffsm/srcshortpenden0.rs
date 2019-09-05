#[doc = "Reader of register SRCSHORTPENDEN0"]
pub type R = crate::R<u32, super::SRCSHORTPENDEN0>;
#[doc = "Writer for register SRCSHORTPENDEN0"]
pub type W = crate::W<u32, super::SRCSHORTPENDEN0>;
#[doc = "Register SRCSHORTPENDEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCSHORTPENDEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRCSHORTPENDEN0`"]
pub type SRCSHORTPENDEN0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCSHORTPENDEN0`"]
pub struct SRCSHORTPENDEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCSHORTPENDEN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden0(&self) -> SRCSHORTPENDEN0_R {
        SRCSHORTPENDEN0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden0(&mut self) -> SRCSHORTPENDEN0_W {
        SRCSHORTPENDEN0_W { w: self }
    }
}
