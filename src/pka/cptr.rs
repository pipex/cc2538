#[doc = "Reader of register CPTR"]
pub type R = crate::R<u32, super::CPTR>;
#[doc = "Writer for register CPTR"]
pub type W = crate::W<u32, super::CPTR>;
#[doc = "Register CPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPTR`"]
pub type CPTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CPTR`"]
pub struct CPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - 10:0\\] This register specifies the location of vector C within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\] must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn cptr(&self) -> CPTR_R {
        CPTR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\] This register specifies the location of vector C within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\] must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn cptr(&mut self) -> CPTR_W {
        CPTR_W { w: self }
    }
}
