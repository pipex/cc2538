#[doc = "Reader of register SRCRESINDEX"]
pub type R = crate::R<u32, super::SRCRESINDEX>;
#[doc = "Writer for register SRCRESINDEX"]
pub type W = crate::W<u32, super::SRCRESINDEX>;
#[doc = "Register SRCRESINDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCRESINDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRCRESINDEX`"]
pub type SRCRESINDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRCRESINDEX`"]
pub struct SRCRESINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCRESINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\] The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
    #[inline(always)]
    pub fn srcresindex(&self) -> SRCRESINDEX_R {
        SRCRESINDEX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\] The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
    #[inline(always)]
    pub fn srcresindex(&mut self) -> SRCRESINDEX_W {
        SRCRESINDEX_W { w: self }
    }
}
