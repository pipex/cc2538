#[doc = "Reader of register CHASGN"]
pub type R = crate::R<u32, super::CHASGN>;
#[doc = "Writer for register CHASGN"]
pub type W = crate::W<u32, super::CHASGN>;
#[doc = "Register CHASGN `reset()`'s with value 0"]
impl crate::ResetValue for super::CHASGN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHASGN`"]
pub type CHASGN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHASGN`"]
pub struct CHASGN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHASGN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
    #[inline(always)]
    pub fn chasgn(&self) -> CHASGN_R {
        CHASGN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
    #[inline(always)]
    pub fn chasgn(&mut self) -> CHASGN_W {
        CHASGN_W { w: self }
    }
}
