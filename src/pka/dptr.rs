#[doc = "Reader of register DPTR"]
pub type R = crate::R<u32, super::DPTR>;
#[doc = "Writer for register DPTR"]
pub type W = crate::W<u32, super::DPTR>;
#[doc = "Register DPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPTR`"]
pub type DPTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DPTR`"]
pub struct DPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - 10:0\\] This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\] must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn dptr(&self) -> DPTR_R {
        DPTR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\] This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\] must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn dptr(&mut self) -> DPTR_W {
        DPTR_W { w: self }
    }
}
