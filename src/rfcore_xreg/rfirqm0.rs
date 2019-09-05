#[doc = "Reader of register RFIRQM0"]
pub type R = crate::R<u32, super::RFIRQM0>;
#[doc = "Writer for register RFIRQM0"]
pub type W = crate::W<u32, super::RFIRQM0>;
#[doc = "Register RFIRQM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RFIRQM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RFIRQM`"]
pub type RFIRQM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFIRQM`"]
pub struct RFIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIRQM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\] Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
    #[inline(always)]
    pub fn rfirqm(&self) -> RFIRQM_R {
        RFIRQM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
    #[inline(always)]
    pub fn rfirqm(&mut self) -> RFIRQM_W {
        RFIRQM_W { w: self }
    }
}
