#[doc = "Reader of register ALTCLR"]
pub type R = crate::R<u32, super::ALTCLR>;
#[doc = "Writer for register ALTCLR"]
pub type W = crate::W<u32, super::ALTCLR>;
#[doc = "Register ALTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTCLR {
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
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] alternate clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\] bit in the DMAALTSET register meaning that channel \\[n\\] is using the primary control structure. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\] Channel \\[n\\] alternate clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\] bit in the DMAALTSET register meaning that channel \\[n\\] is using the primary control structure. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
}
