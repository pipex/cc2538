#[doc = "Reader of register ALENGTH"]
pub type R = crate::R<u32, super::ALENGTH>;
#[doc = "Writer for register ALENGTH"]
pub type W = crate::W<u32, super::ALENGTH>;
#[doc = "Register ALENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::ALENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALENGTH`"]
pub type ALENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ALENGTH`"]
pub struct ALENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ALENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - 8:0\\] This register specifies the length (in 32-bit words) of Vector A."]
    #[inline(always)]
    pub fn alength(&self) -> ALENGTH_R {
        ALENGTH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\] This register specifies the length (in 32-bit words) of Vector A."]
    #[inline(always)]
    pub fn alength(&mut self) -> ALENGTH_W {
        ALENGTH_W { w: self }
    }
}
