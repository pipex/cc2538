#[doc = "Reader of register F0"]
pub type R = crate::R<u32, super::F0>;
#[doc = "Writer for register F0"]
pub type W = crate::W<u32, super::F0>;
#[doc = "Register F0 `reset()`'s with value 0"]
impl crate::ResetValue for super::F0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `USBF0`"]
pub type USBF0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBF0`"]
pub struct USBF0_W<'a> {
    w: &'a mut W,
}
impl<'a> USBF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\] Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
    #[inline(always)]
    pub fn usbf0(&self) -> USBF0_R {
        USBF0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
    #[inline(always)]
    pub fn usbf0(&mut self) -> USBF0_W {
        USBF0_W { w: self }
    }
}
