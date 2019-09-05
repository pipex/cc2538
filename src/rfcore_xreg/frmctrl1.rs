#[doc = "Reader of register FRMCTRL1"]
pub type R = crate::R<u32, super::FRMCTRL1>;
#[doc = "Writer for register FRMCTRL1"]
pub type W = crate::W<u32, super::FRMCTRL1>;
#[doc = "Register FRMCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FRMCTRL1 {
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
#[doc = "Reader of field `PENDING_OR`"]
pub type PENDING_OR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PENDING_OR`"]
pub struct PENDING_OR_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDING_OR_W<'a> {
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
#[doc = "Reader of field `IGNORE_TX_UNDERF`"]
pub type IGNORE_TX_UNDERF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNORE_TX_UNDERF`"]
pub struct IGNORE_TX_UNDERF_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNORE_TX_UNDERF_W<'a> {
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
#[doc = "Reader of field `SET_RXENMASK_ON_TX`"]
pub type SET_RXENMASK_ON_TX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SET_RXENMASK_ON_TX`"]
pub struct SET_RXENMASK_ON_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_RXENMASK_ON_TX_W<'a> {
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
    #[doc = "Bits 3:7 - 7:3\\] Reserved. Read as 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\] Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
    #[inline(always)]
    pub fn pending_or(&self) -> PENDING_OR_R {
        PENDING_OR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
    #[inline(always)]
    pub fn ignore_tx_underf(&self) -> IGNORE_TX_UNDERF_R {
        IGNORE_TX_UNDERF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
    #[inline(always)]
    pub fn set_rxenmask_on_tx(&self) -> SET_RXENMASK_ON_TX_R {
        SET_RXENMASK_ON_TX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Reserved. Read as 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Defines whether the pending data bit in outgoing acknowledgment frames is always set to 1 or controlled by the main FSM and the address filtering 0: Pending data bit is controlled by main FSM and address filtering. 1: Pending data bit is always 1."]
    #[inline(always)]
    pub fn pending_or(&mut self) -> PENDING_OR_W {
        PENDING_OR_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Defines whether or not TX underflow should be ignored 0: Normal TX operation. TX underflow is detected and TX is aborted if underflow occurs. 1: Ignore TX underflow. Transmit the number of bytes given by the frame-length field."]
    #[inline(always)]
    pub fn ignore_tx_underf(&mut self) -> IGNORE_TX_UNDERF_W {
        IGNORE_TX_UNDERF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Defines whether STXON sets bit 6 in the RXENABLE register or leaves it unchanged 0: Does not affect RXENABLE 1: Sets bit 6 in RXENABLE. Used for backward compatibility with the CC2420."]
    #[inline(always)]
    pub fn set_rxenmask_on_tx(&mut self) -> SET_RXENMASK_ON_TX_W {
        SET_RXENMASK_ON_TX_W { w: self }
    }
}
