#[doc = "Reader of register LPBKUART"]
pub type R = crate::R<u32, super::LPBKUART>;
#[doc = "Writer for register LPBKUART"]
pub type W = crate::W<u32, super::LPBKUART>;
#[doc = "Register LPBKUART `reset()`'s with value 0"]
impl crate::ResetValue for super::LPBKUART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPBKUART`"]
pub type LPBKUART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPBKUART`"]
pub struct LPBKUART_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBKUART_W<'a> {
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
    #[doc = "Bit 0 - 0:0\\] UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
    #[inline(always)]
    pub fn lpbkuart(&self) -> LPBKUART_R {
        LPBKUART_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\] UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
    #[inline(always)]
    pub fn lpbkuart(&mut self) -> LPBKUART_W {
        LPBKUART_W { w: self }
    }
}
