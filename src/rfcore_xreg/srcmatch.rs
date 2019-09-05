#[doc = "Reader of register SRCMATCH"]
pub type R = crate::R<u32, super::SRCMATCH>;
#[doc = "Writer for register SRCMATCH"]
pub type W = crate::W<u32, super::SRCMATCH>;
#[doc = "Register SRCMATCH `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCMATCH {
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
#[doc = "Reader of field `Reserved8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PEND_DATAREQ_ONLY`"]
pub type PEND_DATAREQ_ONLY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEND_DATAREQ_ONLY`"]
pub struct PEND_DATAREQ_ONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PEND_DATAREQ_ONLY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AUTOPEND`"]
pub type AUTOPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOPEND`"]
pub struct AUTOPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPEND_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SRC_MATCH_EN`"]
pub type SRC_MATCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC_MATCH_EN`"]
pub struct SRC_MATCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MATCH_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 3:7 - 7:3\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\] When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
    #[inline(always)]
    pub fn pend_datareq_only(&self) -> PEND_DATAREQ_ONLY_R {
        PEND_DATAREQ_ONLY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
    #[inline(always)]
    pub fn autopend(&self) -> AUTOPEND_R {
        AUTOPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
    #[inline(always)]
    pub fn src_match_en(&self) -> SRC_MATCH_EN_R {
        SRC_MATCH_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
    #[inline(always)]
    pub fn pend_datareq_only(&mut self) -> PEND_DATAREQ_ONLY_W {
        PEND_DATAREQ_ONLY_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
    #[inline(always)]
    pub fn autopend(&mut self) -> AUTOPEND_W {
        AUTOPEND_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
    #[inline(always)]
    pub fn src_match_en(&mut self) -> SRC_MATCH_EN_W {
        SRC_MATCH_EN_W { w: self }
    }
}
