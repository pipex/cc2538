#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved6`"]
pub type RESERVED6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTSEN`"]
pub type CTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSEN`"]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
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
#[doc = "Reader of field `RTSEN`"]
pub type RTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTSEN`"]
pub struct RTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `RXE`"]
pub type RXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXE`"]
pub struct RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_W<'a> {
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
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
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
#[doc = "Reader of field `LBE`"]
pub type LBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBE`"]
pub struct LBE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBE_W<'a> {
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
#[doc = "Reader of field `LIN`"]
pub type LIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIN`"]
pub struct LIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIN_W<'a> {
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
#[doc = "Reader of field `HSE`"]
pub type HSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSE`"]
pub struct HSE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE_W<'a> {
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
#[doc = "Reader of field `EOT`"]
pub type EOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOT`"]
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SIRLP`"]
pub type SIRLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIRLP`"]
pub struct SIRLP_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRLP_W<'a> {
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
#[doc = "Reader of field `SIREN`"]
pub type SIREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIREN`"]
pub struct SIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIREN_W<'a> {
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
#[doc = "Reader of field `UARTEN`"]
pub type UARTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UARTEN`"]
pub struct UARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTEN_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\] U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\] U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - 13:10\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\] UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
    #[inline(always)]
    pub fn lbe(&self) -> LBE_R {
        LBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
    #[inline(always)]
    pub fn lin(&self) -> LIN_R {
        LIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
    #[inline(always)]
    pub fn hse(&self) -> HSE_R {
        HSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation. Note field should always be written as 0 for correct operation."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn sirlp(&self) -> SIRLP_R {
        SIRLP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
    #[inline(always)]
    pub fn siren(&self) -> SIREN_R {
        SIREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn uarten(&self) -> UARTEN_R {
        UARTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] U1CTS Hardware Flow control enable 1: When U1CTS input is asserted, UART1 can transmit data. 0: U1CTS does not control UART1 data transmission. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] U1RTS Hardware Flow control enable 1: U1RTS indicates the state of UART1 receive FIFO. U1RTS remains asserted until the preprogrammed watermark level is reached, indicating that the UART1 RXFIFO has no space to store additional characters. 0: U1RTS does not indicate state of UART1 RX FIFO. Note: Only used for UART1. This bit is reserved RO for UART0."]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W { w: self }
    }
    #[doc = "Bits 10:13 - 13:10\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] UART receive enable 1: The receive section of the UART is enabled. 0: The receive section of the UART is disabled. If the UART is disabled in the middle of a receive, it completes the current character before stopping. Note: To enable reception, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W {
        RXE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] UART transmit enable 1: The transmit section of the UART is enabled. 0: The transmit section of the UART is disabled. If the UART is disabled in the middle of a transmission, it completes the current character before stopping. Note: To enable transmission, the UARTEN bit must also be set."]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] UART loop back enable 1: The UnTx path is fed through the UnRx path. 0: Normal operation"]
    #[inline(always)]
    pub fn lbe(&mut self) -> LBE_W {
        LBE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] LIN mode enable 1: The UART operates in LIN mode. 0: Normal operation"]
    #[inline(always)]
    pub fn lin(&mut self) -> LIN_W {
        LIN_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] High-speed enable 0: The UART is clocked using the system clock divided by 16. 1: The UART is clocked using the system clock divided by 8. Note: System clock used is also dependent on the baud-rate divisor configuration (See Universal Asynchronous Receivers/Transmitters - Baud-Rate Generation)."]
    #[inline(always)]
    pub fn hse(&mut self) -> HSE_W {
        HSE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] End of transmission This bit determines the behavior of the TXRIS bit in the UARTRIS register. 1: The TXRIS bit is set only after all transmitted data, including stop bits, have cleared the serializer. 0: The TXRIS bit is set when the transmit FIFO condition specified in UARTIFLS is met."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation. Note field should always be written as 0 for correct operation."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] UART SIR low-power mode This bit selects the IrDA encoding mode. 1: The UART operates in SIR Low-Power mode. Low-level bits are transmitted with a pulse width which is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. 0: Low-level bits are transmitted as an active high pulse with a width of 3/16th of the bit period. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn sirlp(&mut self) -> SIRLP_W {
        SIRLP_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] UART SIR enable 1: The IrDA SIR block is enabled, and the UART transmits and receives data using SIR protocol. 0: Normal operation."]
    #[inline(always)]
    pub fn siren(&mut self) -> SIREN_W {
        SIREN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] UART enable 1: The UART is enabled. 0: The UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn uarten(&mut self) -> UARTEN_W {
        UARTEN_W { w: self }
    }
}
