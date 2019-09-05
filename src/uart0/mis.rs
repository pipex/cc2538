#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Writer for register MIS"]
pub type W = crate::W<u32, super::MIS>;
#[doc = "Register MIS `reset()`'s with value 0"]
impl crate::ResetValue for super::MIS {
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
#[doc = "Reader of field `LME5MIS`"]
pub type LME5MIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LME5MIS`"]
pub struct LME5MIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LME5MIS_W<'a> {
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
#[doc = "Reader of field `LME1MIS`"]
pub type LME1MIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LME1MIS`"]
pub struct LME1MIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LME1MIS_W<'a> {
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
#[doc = "Reader of field `LMSBMIS`"]
pub type LMSBMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LMSBMIS`"]
pub struct LMSBMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LMSBMIS_W<'a> {
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
#[doc = "Reader of field `NINEBITMIS`"]
pub type NINEBITMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NINEBITMIS`"]
pub struct NINEBITMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NINEBITMIS_W<'a> {
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
#[doc = "Reader of field `OEMIS`"]
pub type OEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEMIS`"]
pub struct OEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OEMIS_W<'a> {
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
#[doc = "Reader of field `BEMIS`"]
pub type BEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEMIS`"]
pub struct BEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BEMIS_W<'a> {
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
#[doc = "Reader of field `PEMIS`"]
pub type PEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEMIS`"]
pub struct PEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PEMIS_W<'a> {
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
#[doc = "Reader of field `FEMIS`"]
pub type FEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEMIS`"]
pub struct FEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FEMIS_W<'a> {
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
#[doc = "Reader of field `RTMIS`"]
pub type RTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTMIS`"]
pub struct RTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMIS_W<'a> {
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
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMIS`"]
pub struct TXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIS_W<'a> {
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
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMIS`"]
pub struct RXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMIS_W<'a> {
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
    #[doc = "Bit 15 - 15:15\\] LIN mode edge 5 masked interrupt status 1: An unmasked interrupt was signaled due to the 5th falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5mis(&self) -> LME5MIS_R {
        LME5MIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\] LIN mode edge 1 masked interrupt status 1: An unmasked interrupt was signaled due to the 1st falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1mis(&self) -> LME1MIS_R {
        LME1MIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\] LIN mode sync break masked interrupt status 1: An unmasked interrupt was signaled due to the receipt of a LIN sync break. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbmis(&self) -> LMSBMIS_R {
        LMSBMIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\] 9-bit mode masked interrupt status 1: An unmasked interrupt was signaled due to a receive address match. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitmis(&self) -> NINEBITMIS_R {
        NINEBITMIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] UART overrun error masked interrupt status 1: An unmasked interrupt was signaled due to an overrun error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] UART break error masked interrupt status 1: An unmasked interrupt was signaled due to a break error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] UART parity error masked interrupt status 1: An unmasked interrupt was signaled due to a parity error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] UART framing error masked interrupt status 1: An unmasked interrupt was signaled due to a framing error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] UART receive time-out masked interrupt status 1: An unmasked interrupt was signaled due to a receive time out. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] UART transmit masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified transmit FIFO level (if the EOT bit is clear) or due to the transmission of the last data bit (if the EOT bit is set). 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] UART receive masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified receive FIFO level. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
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
    #[doc = "Bit 15 - 15:15\\] LIN mode edge 5 masked interrupt status 1: An unmasked interrupt was signaled due to the 5th falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME5IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme5mis(&mut self) -> LME5MIS_W {
        LME5MIS_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] LIN mode edge 1 masked interrupt status 1: An unmasked interrupt was signaled due to the 1st falling edge of the LIN sync field. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LME1IC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lme1mis(&mut self) -> LME1MIS_W {
        LME1MIS_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] LIN mode sync break masked interrupt status 1: An unmasked interrupt was signaled due to the receipt of a LIN sync break. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the LMSBIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn lmsbmis(&mut self) -> LMSBMIS_W {
        LMSBMIS_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 9-bit mode masked interrupt status 1: An unmasked interrupt was signaled due to a receive address match. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the 9BITIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn ninebitmis(&mut self) -> NINEBITMIS_W {
        NINEBITMIS_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] UART overrun error masked interrupt status 1: An unmasked interrupt was signaled due to an overrun error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the OEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn oemis(&mut self) -> OEMIS_W {
        OEMIS_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] UART break error masked interrupt status 1: An unmasked interrupt was signaled due to a break error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the BEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn bemis(&mut self) -> BEMIS_W {
        BEMIS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] UART parity error masked interrupt status 1: An unmasked interrupt was signaled due to a parity error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the PEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn pemis(&mut self) -> PEMIS_W {
        PEMIS_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] UART framing error masked interrupt status 1: An unmasked interrupt was signaled due to a framing error. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the FEIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn femis(&mut self) -> FEMIS_W {
        FEMIS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] UART receive time-out masked interrupt status 1: An unmasked interrupt was signaled due to a receive time out. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RTIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rtmis(&mut self) -> RTMIS_W {
        RTMIS_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] UART transmit masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified transmit FIFO level (if the EOT bit is clear) or due to the transmission of the last data bit (if the EOT bit is set). 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the TXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn txmis(&mut self) -> TXMIS_W {
        TXMIS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] UART receive masked interrupt status 1: An unmasked interrupt was signaled due to passing through the specified receive FIFO level. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the RXIC bit in the UARTICR register."]
    #[inline(always)]
    pub fn rxmis(&mut self) -> RXMIS_W {
        RXMIS_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
}
