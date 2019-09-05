#[doc = "Reader of register PTME1"]
pub type R = crate::R<u32, super::PTME1>;
#[doc = "Writer for register PTME1"]
pub type W = crate::W<u32, super::PTME1>;
#[doc = "Register PTME1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PTME1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `UART1TME`"]
pub type UART1TME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1TME`"]
pub struct UART1TME_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1TME_W<'a> {
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
#[doc = "Reader of field `UART0TME`"]
pub type UART0TME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0TME`"]
pub struct UART0TME_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0TME_W<'a> {
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
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - 31:10\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 9 - 9:9\\] UART1 test mode enable"]
    #[inline(always)]
    pub fn uart1tme(&self) -> UART1TME_R {
        UART1TME_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] UART0 test mode enable"]
    #[inline(always)]
    pub fn uart0tme(&self) -> UART0TME_R {
        UART0TME_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] UART1 test mode enable"]
    #[inline(always)]
    pub fn uart1tme(&mut self) -> UART1TME_W {
        UART1TME_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] UART0 test mode enable"]
    #[inline(always)]
    pub fn uart0tme(&mut self) -> UART0TME_W {
        UART0TME_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
}
