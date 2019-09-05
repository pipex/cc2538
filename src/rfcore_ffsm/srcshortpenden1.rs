#[doc = "Reader of register SRCSHORTPENDEN1"]
pub type R = crate::R<u32, super::SRCSHORTPENDEN1>;
#[doc = "Writer for register SRCSHORTPENDEN1"]
pub type W = crate::W<u32, super::SRCSHORTPENDEN1>;
#[doc = "Register SRCSHORTPENDEN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCSHORTPENDEN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRCSHORTPENDEN1`"]
pub type SRCSHORTPENDEN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCSHORTPENDEN1`"]
pub struct SRCSHORTPENDEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCSHORTPENDEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden1(&self) -> SRCSHORTPENDEN1_R {
        SRCSHORTPENDEN1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden1(&mut self) -> SRCSHORTPENDEN1_W {
        SRCSHORTPENDEN1_W { w: self }
    }
}
