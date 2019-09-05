#[doc = "Reader of register USB_IRQ_ACK"]
pub type R = crate::R<u32, super::USB_IRQ_ACK>;
#[doc = "Writer for register USB_IRQ_ACK"]
pub type W = crate::W<u32, super::USB_IRQ_ACK>;
#[doc = "Register USB_IRQ_ACK `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_IRQ_ACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved31`"]
pub type RESERVED31_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved31`"]
pub struct RESERVED31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `USBACK`"]
pub type USBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBACK`"]
pub struct USBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> USBACK_W<'a> {
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
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\] USB masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn usback(&self) -> USBACK_R {
        USBACK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn reserved31(&mut self) -> RESERVED31_W {
        RESERVED31_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] USB masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn usback(&mut self) -> USBACK_W {
        USBACK_W { w: self }
    }
}
