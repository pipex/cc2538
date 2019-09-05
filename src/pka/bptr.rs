#[doc = "Reader of register BPTR"]
pub type R = crate::R<u32, super::BPTR>;
#[doc = "Writer for register BPTR"]
pub type W = crate::W<u32, super::BPTR>;
#[doc = "Register BPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::BPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BPTR`"]
pub type BPTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BPTR`"]
pub struct BPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - 10:0\\] This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\] must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn bptr(&self) -> BPTR_R {
        BPTR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\] This register specifies the location of vector B within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\] must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn bptr(&mut self) -> BPTR_W {
        BPTR_W { w: self }
    }
}
