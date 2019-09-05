#[doc = "Reader of register FSMSTAT1"]
pub type R = crate::R<u32, super::FSMSTAT1>;
#[doc = "Writer for register FSMSTAT1"]
pub type W = crate::W<u32, super::FSMSTAT1>;
#[doc = "Register FSMSTAT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSMSTAT1 {
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
#[doc = "Reader of field `FIFO`"]
pub type FIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO`"]
pub struct FIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CCA`"]
pub type CCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCA`"]
pub struct CCA_W<'a> {
    w: &'a mut W,
}
impl<'a> CCA_W<'a> {
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
#[doc = "Reader of field `SAMPLED_CCA`"]
pub type SAMPLED_CCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAMPLED_CCA`"]
pub struct SAMPLED_CCA_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLED_CCA_W<'a> {
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
#[doc = "Reader of field `LOCK_STATUS`"]
pub type LOCK_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_STATUS`"]
pub struct LOCK_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_STATUS_W<'a> {
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
#[doc = "Reader of field `TX_ACTIVE`"]
pub type TX_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ACTIVE`"]
pub struct TX_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ACTIVE_W<'a> {
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
#[doc = "Reader of field `RX_ACTIVE`"]
pub type RX_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_ACTIVE`"]
pub struct RX_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ACTIVE_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] FIFO is high when there is data in the RX FIFO. FIFO is low during RX FIFO overflow."]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] FIFOP is set high when there are at more than FIFOP_THR bytes of data in the RX FIFO that has passed frame filtering. FIFOP is set high when there is at least one complete frame in the RX FIFO. FIFOP is high during RX FIFO overflow."]
    #[inline(always)]
    pub fn fifop(&self) -> FIFOP_R {
        FIFOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] In TX 0: When a complete frame with SFD was sent or no SFD was sent 1: SFD was sent. In RX 0: When a complete frame was received or no SFD was received 1: SFD was received."]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Clear channel assessment Dependent on CCA_MODE settings. See CCACTRL1 for details."]
    #[inline(always)]
    pub fn cca(&self) -> CCA_R {
        CCA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Contains a sampled value of the CCA The value is updated when a SSAMPLECCA or STXONCCA strobe is issued."]
    #[inline(always)]
    pub fn sampled_cca(&self) -> SAMPLED_CCA_R {
        SAMPLED_CCA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] 1 when PLL is in lock; otherwise 0"]
    #[inline(always)]
    pub fn lock_status(&self) -> LOCK_STATUS_R {
        LOCK_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Status signal Active when FFC is in one of the transmit states"]
    #[inline(always)]
    pub fn tx_active(&self) -> TX_ACTIVE_R {
        TX_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Status signal Active when FFC is in one of the receive states"]
    #[inline(always)]
    pub fn rx_active(&self) -> RX_ACTIVE_R {
        RX_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] FIFO is high when there is data in the RX FIFO. FIFO is low during RX FIFO overflow."]
    #[inline(always)]
    pub fn fifo(&mut self) -> FIFO_W {
        FIFO_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] FIFOP is set high when there are at more than FIFOP_THR bytes of data in the RX FIFO that has passed frame filtering. FIFOP is set high when there is at least one complete frame in the RX FIFO. FIFOP is high during RX FIFO overflow."]
    #[inline(always)]
    pub fn fifop(&mut self) -> FIFOP_W {
        FIFOP_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] In TX 0: When a complete frame with SFD was sent or no SFD was sent 1: SFD was sent. In RX 0: When a complete frame was received or no SFD was received 1: SFD was received."]
    #[inline(always)]
    pub fn sfd(&mut self) -> SFD_W {
        SFD_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Clear channel assessment Dependent on CCA_MODE settings. See CCACTRL1 for details."]
    #[inline(always)]
    pub fn cca(&mut self) -> CCA_W {
        CCA_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Contains a sampled value of the CCA The value is updated when a SSAMPLECCA or STXONCCA strobe is issued."]
    #[inline(always)]
    pub fn sampled_cca(&mut self) -> SAMPLED_CCA_W {
        SAMPLED_CCA_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 1 when PLL is in lock; otherwise 0"]
    #[inline(always)]
    pub fn lock_status(&mut self) -> LOCK_STATUS_W {
        LOCK_STATUS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Status signal Active when FFC is in one of the transmit states"]
    #[inline(always)]
    pub fn tx_active(&mut self) -> TX_ACTIVE_W {
        TX_ACTIVE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Status signal Active when FFC is in one of the receive states"]
    #[inline(always)]
    pub fn rx_active(&mut self) -> RX_ACTIVE_W {
        RX_ACTIVE_W { w: self }
    }
}
