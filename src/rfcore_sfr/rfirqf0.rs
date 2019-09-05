#[doc = "Reader of register RFIRQF0"]
pub type R = crate::R<u32, super::RFIRQF0>;
#[doc = "Writer for register RFIRQF0"]
pub type W = crate::W<u32, super::RFIRQF0>;
#[doc = "Register RFIRQF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RFIRQF0 {
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
#[doc = "Reader of field `RXMASKZERO`"]
pub type RXMASKZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMASKZERO`"]
pub struct RXMASKZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMASKZERO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RXPKTDONE`"]
pub type RXPKTDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPKTDONE`"]
pub struct RXPKTDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPKTDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FRAME_ACCEPTED`"]
pub type FRAME_ACCEPTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAME_ACCEPTED`"]
pub struct FRAME_ACCEPTED_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_ACCEPTED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SRC_MATCH_FOUND`"]
pub type SRC_MATCH_FOUND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC_MATCH_FOUND`"]
pub struct SRC_MATCH_FOUND_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MATCH_FOUND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SRC_MATCH_DONE`"]
pub type SRC_MATCH_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC_MATCH_DONE`"]
pub struct SRC_MATCH_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MATCH_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FIFOP`"]
pub type FIFOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOP`"]
pub struct FIFOP_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOP_W<'a> {
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
#[doc = "Reader of field `SFD`"]
pub type SFD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFD`"]
pub struct SFD_W<'a> {
    w: &'a mut W,
}
impl<'a> SFD_W<'a> {
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
#[doc = "Reader of field `ACT_UNUSED`"]
pub type ACT_UNUSED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACT_UNUSED`"]
pub struct ACT_UNUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_UNUSED_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxmaskzero(&self) -> RXMASKZERO_R {
        RXMASKZERO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxpktdone(&self) -> RXPKTDONE_R {
        RXPKTDONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn frame_accepted(&self) -> FRAME_ACCEPTED_R {
        FRAME_ACCEPTED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_found(&self) -> SRC_MATCH_FOUND_R {
        SRC_MATCH_FOUND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_done(&self) -> SRC_MATCH_DONE_R {
        SRC_MATCH_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn fifop(&self) -> FIFOP_R {
        FIFOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Reserved 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn act_unused(&self) -> ACT_UNUSED_R {
        ACT_UNUSED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxmaskzero(&mut self) -> RXMASKZERO_W {
        RXMASKZERO_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxpktdone(&mut self) -> RXPKTDONE_W {
        RXPKTDONE_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn frame_accepted(&mut self) -> FRAME_ACCEPTED_W {
        FRAME_ACCEPTED_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_found(&mut self) -> SRC_MATCH_FOUND_W {
        SRC_MATCH_FOUND_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_done(&mut self) -> SRC_MATCH_DONE_W {
        SRC_MATCH_DONE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn fifop(&mut self) -> FIFOP_W {
        FIFOP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SFD_W {
        SFD_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Reserved 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn act_unused(&mut self) -> ACT_UNUSED_W {
        ACT_UNUSED_W { w: self }
    }
}
