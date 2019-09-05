#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
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
#[doc = "Reader of field `LME5IM`"]
pub type LME5IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LME5IM`"]
pub struct LME5IM_W<'a> {
    w: &'a mut W,
}
impl<'a> LME5IM_W<'a> {
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
#[doc = "Reader of field `LME1IM`"]
pub type LME1IM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LME1IM`"]
pub struct LME1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> LME1IM_W<'a> {
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
#[doc = "Reader of field `LMSBIM`"]
pub type LMSBIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LMSBIM`"]
pub struct LMSBIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LMSBIM_W<'a> {
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
#[doc = "Reader of field `NINEBITIM`"]
pub type NINEBITIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NINEBITIM`"]
pub struct NINEBITIM_W<'a> {
    w: &'a mut W,
}
impl<'a> NINEBITIM_W<'a> {
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
#[doc = "Reader of field `OEIM`"]
pub type OEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEIM`"]
pub struct OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIM_W<'a> {
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
#[doc = "Reader of field `BEIM`"]
pub type BEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIM`"]
pub struct BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIM_W<'a> {
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
#[doc = "Reader of field `PEIM`"]
pub type PEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEIM`"]
pub struct PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIM_W<'a> {
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
#[doc = "Reader of field `FEIM`"]
pub type FEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEIM`"]
pub struct FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIM_W<'a> {
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
#[doc = "Reader of field `RTIM`"]
pub type RTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIM`"]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
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
#[doc = "Reader of field `TXIM`"]
pub type TXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIM`"]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
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
#[doc = "Reader of field `RXIM`"]
pub type RXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIM`"]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
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
    #[doc = "Bit 15 - 15:15\\] LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme5im(&self) -> LME5IM_R {
        LME5IM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\] LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme1im(&self) -> LME1IM_R {
        LME1IM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\] LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lmsbim(&self) -> LMSBIM_R {
        LMSBIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\] 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn ninebitim(&self) -> NINEBITIM_R {
        NINEBITIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
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
    #[doc = "Bit 15 - 15:15\\] LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme5im(&mut self) -> LME5IM_W {
        LME5IM_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme1im(&mut self) -> LME1IM_W {
        LME1IM_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lmsbim(&mut self) -> LMSBIM_W {
        LMSBIM_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn ninebitim(&mut self) -> NINEBITIM_W {
        NINEBITIM_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W {
        OEIM_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W {
        BEIM_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W {
        PEIM_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W {
        FEIM_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
}
