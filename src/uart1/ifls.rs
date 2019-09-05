#[doc = "Reader of register IFLS"]
pub type R = crate::R<u32, super::IFLS>;
#[doc = "Writer for register IFLS"]
pub type W = crate::W<u32, super::IFLS>;
#[doc = "Register IFLS `reset()`'s with value 0"]
impl crate::ResetValue for super::IFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved26`"]
pub type RESERVED26_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved26`"]
pub struct RESERVED26_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `RXIFLSEL`"]
pub type RXIFLSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXIFLSEL`"]
pub struct RXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIFLSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `TXIFLSEL`"]
pub type TXIFLSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXIFLSEL`"]
pub struct TXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIFLSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 3:5 - 5:3\\] UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\] UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved26(&mut self) -> RESERVED26_W {
        RESERVED26_W { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\] UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W {
        RXIFLSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W {
        TXIFLSEL_W { w: self }
    }
}
