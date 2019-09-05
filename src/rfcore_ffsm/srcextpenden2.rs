#[doc = "Reader of register SRCEXTPENDEN2"]
pub type R = crate::R<u32, super::SRCEXTPENDEN2>;
#[doc = "Writer for register SRCEXTPENDEN2"]
pub type W = crate::W<u32, super::SRCEXTPENDEN2>;
#[doc = "Register SRCEXTPENDEN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCEXTPENDEN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRCEXTPENDEN2`"]
pub type SRCEXTPENDEN2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCEXTPENDEN2`"]
pub struct SRCEXTPENDEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCEXTPENDEN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\] bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden2(&self) -> SRCEXTPENDEN2_R {
        SRCEXTPENDEN2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\] bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden2(&mut self) -> SRCEXTPENDEN2_W {
        SRCEXTPENDEN2_W { w: self }
    }
}
