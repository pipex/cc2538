#[doc = "Reader of register SRCEXTPENDEN1"]
pub type R = crate::R<u32, super::SRCEXTPENDEN1>;
#[doc = "Writer for register SRCEXTPENDEN1"]
pub type W = crate::W<u32, super::SRCEXTPENDEN1>;
#[doc = "Register SRCEXTPENDEN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCEXTPENDEN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRCEXTPENDEN1`"]
pub type SRCEXTPENDEN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCEXTPENDEN1`"]
pub struct SRCEXTPENDEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCEXTPENDEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\] bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden1(&self) -> SRCEXTPENDEN1_R {
        SRCEXTPENDEN1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\] bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden1(&mut self) -> SRCEXTPENDEN1_W {
        SRCEXTPENDEN1_W { w: self }
    }
}
