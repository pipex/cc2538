#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `Reserved16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | (((value as u32) & 0x07ff) << 5);
        self.w
    }
}
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSY`"]
pub struct BSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BSY_W<'a> {
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
#[doc = "Reader of field `RFF`"]
pub type RFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFF`"]
pub struct RFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFF_W<'a> {
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
#[doc = "Reader of field `RNE`"]
pub type RNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNE`"]
pub struct RNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RNE_W<'a> {
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
#[doc = "Reader of field `TNF`"]
pub type TNF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TNF`"]
pub struct TNF_W<'a> {
    w: &'a mut W,
}
impl<'a> TNF_W<'a> {
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
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFE`"]
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 5:15 - 15:5\\] Reserved, read unpredictable, should be written as 0."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 4 - 4:4\\] SSI busy bit (RO) Reset value: 0x0 0: SSI is idle. 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] SSI receive FIFO full (RO) Reset value: 0x0 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] SSI receive FIFO not empty (RO) Reset value: 0x0 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] SSI transmit FIFO not full (RO) Reset value: 0x1 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] SSI transmit FIFO empty (RO) Reset value: 0x1 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 5:15 - 15:5\\] Reserved, read unpredictable, should be written as 0."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] SSI busy bit (RO) Reset value: 0x0 0: SSI is idle. 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&mut self) -> BSY_W {
        BSY_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] SSI receive FIFO full (RO) Reset value: 0x0 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&mut self) -> RFF_W {
        RFF_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] SSI receive FIFO not empty (RO) Reset value: 0x0 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&mut self) -> RNE_W {
        RNE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] SSI transmit FIFO not full (RO) Reset value: 0x1 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&mut self) -> TNF_W {
        TNF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] SSI transmit FIFO empty (RO) Reset value: 0x1 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
}
