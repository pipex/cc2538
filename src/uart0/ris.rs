#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Writer for register RIS"]
pub type W = crate::W<u32, super::RIS>;
#[doc = "Register RIS `reset()`'s with value 0"]
impl crate::ResetValue for super::RIS {
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
#[doc = "Reader of field `LME5RIS`"]
pub type LME5RIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LME5RIS`"]
pub struct LME5RIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LME5RIS_W<'a> {
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
#[doc = "Reader of field `LME1RIS`"]
pub type LME1RIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LME1RIS`"]
pub struct LME1RIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LME1RIS_W<'a> {
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
#[doc = "Reader of field `LMSBRIS`"]
pub type LMSBRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LMSBRIS`"]
pub struct LMSBRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LMSBRIS_W<'a> {
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
#[doc = "Reader of field `NINEBITRIS`"]
pub type NINEBITRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NINEBITRIS`"]
pub struct NINEBITRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NINEBITRIS_W<'a> {
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
#[doc = "Reader of field `OERIS`"]
pub type OERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OERIS`"]
pub struct OERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OERIS_W<'a> {
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
#[doc = "Reader of field `BERIS`"]
pub type BERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERIS`"]
pub struct BERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BERIS_W<'a> {
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
#[doc = "Reader of field `PERIS`"]
pub type PERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERIS`"]
pub struct PERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIS_W<'a> {
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
#[doc = "Reader of field `FERIS`"]
pub type FERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FERIS`"]
pub struct FERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FERIS_W<'a> {
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
#[doc = "Reader of field `RTRIS`"]
pub type RTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTRIS`"]
pub struct RTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIS_W<'a> {
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
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRIS`"]
pub struct TXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRIS_W<'a> {
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
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRIS`"]
pub struct RXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRIS_W<'a> {
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
    #[doc = "Bit 15 - 15:15\\] LIN mode edge 5 raw interrupt status 1: The timer value at the 5th falling edge of the LIN sync field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5ris(&self) -> LME5RIS_R {
        LME5RIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\] LIN mode edge 1 raw interrupt status 1: The timer value at the 1st falling edge of the LIN Sync Field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1ris(&self) -> LME1RIS_R {
        LME1RIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\] LIN mode sync break raw interrupt status 1: A LIN sync break has been detected. 0: No interrupt This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbris(&self) -> LMSBRIS_R {
        LMSBRIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\] 9-mit mode raw interrupt status 1: A receive address match has occurred. 0: No interrupt This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitris(&self) -> NINEBITRIS_R {
        NINEBITRIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] UART overrun error raw interrupt status 1: An overrun error has occurred. 0: No interrupt This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] UART break error raw interrupt status 1: A break error has occurred. 0: No interrupt This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] UART parity error raw interrupt status 1: A parity error has occurred. 0: No interrupt This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] UART framing error raw interrupt status 1: A framing error has occurred. 0: No interrupt This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] UART receive time-out raw interrupt status 1: A receive time out has occurred. 0: No interrupt This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] UART transmit raw interrupt status 1: If the EOT bit in the UARTCTL register is clear, the transmit FIFO level has passed through the condition defined in the UARTIFLS register. If the EOT bit is set, the last bit of all transmitted data and flags has left the serializer. 0: No interrupt This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] UART receive raw interrupt status 1: The receive FIFO level has passed through the condition defined in the UARTIFLS register. 0: No interrupt This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 0x01) != 0)
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
    #[doc = "Bit 15 - 15:15\\] LIN mode edge 5 raw interrupt status 1: The timer value at the 5th falling edge of the LIN sync field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5ris(&mut self) -> LME5RIS_W {
        LME5RIS_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] LIN mode edge 1 raw interrupt status 1: The timer value at the 1st falling edge of the LIN Sync Field has been captured. 0: No interrupt This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1ris(&mut self) -> LME1RIS_W {
        LME1RIS_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] LIN mode sync break raw interrupt status 1: A LIN sync break has been detected. 0: No interrupt This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbris(&mut self) -> LMSBRIS_W {
        LMSBRIS_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 9-mit mode raw interrupt status 1: A receive address match has occurred. 0: No interrupt This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitris(&mut self) -> NINEBITRIS_W {
        NINEBITRIS_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] UART overrun error raw interrupt status 1: An overrun error has occurred. 0: No interrupt This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oeris(&mut self) -> OERIS_W {
        OERIS_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] UART break error raw interrupt status 1: A break error has occurred. 0: No interrupt This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn beris(&mut self) -> BERIS_W {
        BERIS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] UART parity error raw interrupt status 1: A parity error has occurred. 0: No interrupt This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn peris(&mut self) -> PERIS_W {
        PERIS_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] UART framing error raw interrupt status 1: A framing error has occurred. 0: No interrupt This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn feris(&mut self) -> FERIS_W {
        FERIS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] UART receive time-out raw interrupt status 1: A receive time out has occurred. 0: No interrupt This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtris(&mut self) -> RTRIS_W {
        RTRIS_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] UART transmit raw interrupt status 1: If the EOT bit in the UARTCTL register is clear, the transmit FIFO level has passed through the condition defined in the UARTIFLS register. If the EOT bit is set, the last bit of all transmitted data and flags has left the serializer. 0: No interrupt This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txris(&mut self) -> TXRIS_W {
        TXRIS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] UART receive raw interrupt status 1: The receive FIFO level has passed through the condition defined in the UARTIFLS register. 0: No interrupt This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxris(&mut self) -> RXRIS_W {
        RXRIS_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
}
