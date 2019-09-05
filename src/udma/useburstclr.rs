#[doc = "Reader of register USEBURSTCLR"]
pub type R = crate::R<u32, super::USEBURSTCLR>;
#[doc = "Writer for register USEBURSTCLR"]
pub type W = crate::W<u32, super::USEBURSTCLR>;
#[doc = "Register USEBURSTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::USEBURSTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLR`"]
pub type CLR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLR`"]
pub struct CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] useburst clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\] bit in the DMAUSEBURSTSET register meaning that uDMA channel \\[n\\] responds to single and burst requests."]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] useburst clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\] bit in the DMAUSEBURSTSET register meaning that uDMA channel \\[n\\] responds to single and burst requests."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
}
