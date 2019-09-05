#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LME5IC`"]
pub type LME5IC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LME5IC`"]
pub struct LME5IC_W<'a> {
    w: &'a mut W,
}
impl<'a> LME5IC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `LME1IC`"]
pub type LME1IC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LME1IC`"]
pub struct LME1IC_W<'a> {
    w: &'a mut W,
}
impl<'a> LME1IC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `LMSBIC`"]
pub type LMSBIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LMSBIC`"]
pub struct LMSBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> LMSBIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `NINEBITIC`"]
pub type NINEBITIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NINEBITIC`"]
pub struct NINEBITIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NINEBITIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `OEIC`"]
pub type OEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEIC`"]
pub struct OEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BEIC`"]
pub type BEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIC`"]
pub struct BEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PEIC`"]
pub type PEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEIC`"]
pub struct PEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `FEIC`"]
pub type FEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEIC`"]
pub struct FEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIC_W<'a> {
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
#[doc = "Reader of field `RTIC`"]
pub type RTIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIC`"]
pub struct RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIC_W<'a> {
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
#[doc = "Reader of field `TXIC`"]
pub type TXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIC`"]
pub struct TXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIC_W<'a> {
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
#[doc = "Reader of field `RXIC`"]
pub type RXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIC`"]
pub struct RXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIC_W<'a> {
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
#[doc = "Reader of field `Reserved4`"]
pub type RESERVED4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\] LIN mode edge 5 interrupt clear Writing 1 to this bit clears the LME5RIS bit in the UARTRIS register and the LME5MIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lme5ic(&self) -> LME5IC_R {
        LME5IC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\] LIN mode edge 1 interrupt clear Writing 1 to this bit clears the LME1RIS bit in the UARTRIS register and the LME1MIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lme1ic(&self) -> LME1IC_R {
        LME1IC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\] LIN mode sync break interrupt clear Writing 1 to this bit clears the LMSBRIS bit in the UARTRIS register and the LMSBMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lmsbic(&self) -> LMSBIC_R {
        LMSBIC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\] 9-bit mode interrupt clear Writing 1 to this bit clears the 9BITRIS bit in the UARTRIS register and the 9BITMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn ninebitic(&self) -> NINEBITIC_R {
        NINEBITIC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] Overrun error interrupt clear Writing 1 to this bit clears the OERIS bit in the UARTRIS register and the OEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn oeic(&self) -> OEIC_R {
        OEIC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] Break error interrupt clear Writing 1 to this bit clears the BERIS bit in the UARTRIS register and the BEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn beic(&self) -> BEIC_R {
        BEIC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] Parity error interrupt clear Writing 1 to this bit clears the PERIS bit in the UARTRIS register and the PEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn peic(&self) -> PEIC_R {
        PEIC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] Framing error interrupt clear Writing 1 to this bit clears the FERIS bit in the UARTRIS register and the FEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn feic(&self) -> FEIC_R {
        FEIC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Receive time-out interrupt clear Writing 1 to this bit clears the RTRIS bit in the UARTRIS register and the RTMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Transmit interrupt clear Writing 1 to this bit clears the TXRIS bit in the UARTRIS register and the TXMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Receive interrupt clear Writing 1 to this bit clears the RXRIS bit in the UARTRIS register and the RXMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - 3:0\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] LIN mode edge 5 interrupt clear Writing 1 to this bit clears the LME5RIS bit in the UARTRIS register and the LME5MIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lme5ic(&mut self) -> LME5IC_W {
        LME5IC_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] LIN mode edge 1 interrupt clear Writing 1 to this bit clears the LME1RIS bit in the UARTRIS register and the LME1MIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lme1ic(&mut self) -> LME1IC_W {
        LME1IC_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] LIN mode sync break interrupt clear Writing 1 to this bit clears the LMSBRIS bit in the UARTRIS register and the LMSBMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lmsbic(&mut self) -> LMSBIC_W {
        LMSBIC_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 9-bit mode interrupt clear Writing 1 to this bit clears the 9BITRIS bit in the UARTRIS register and the 9BITMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn ninebitic(&mut self) -> NINEBITIC_W {
        NINEBITIC_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Overrun error interrupt clear Writing 1 to this bit clears the OERIS bit in the UARTRIS register and the OEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn oeic(&mut self) -> OEIC_W {
        OEIC_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Break error interrupt clear Writing 1 to this bit clears the BERIS bit in the UARTRIS register and the BEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn beic(&mut self) -> BEIC_W {
        BEIC_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Parity error interrupt clear Writing 1 to this bit clears the PERIS bit in the UARTRIS register and the PEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn peic(&mut self) -> PEIC_W {
        PEIC_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Framing error interrupt clear Writing 1 to this bit clears the FERIS bit in the UARTRIS register and the FEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn feic(&mut self) -> FEIC_W {
        FEIC_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Receive time-out interrupt clear Writing 1 to this bit clears the RTRIS bit in the UARTRIS register and the RTMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Transmit interrupt clear Writing 1 to this bit clears the TXRIS bit in the UARTRIS register and the TXMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Receive interrupt clear Writing 1 to this bit clears the RXRIS bit in the UARTRIS register and the RXMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
}
