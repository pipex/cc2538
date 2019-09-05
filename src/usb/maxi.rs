#[doc = "Reader of register MAXI"]
pub type R = crate::R<u32, super::MAXI>;
#[doc = "Writer for register MAXI"]
pub type W = crate::W<u32, super::MAXI>;
#[doc = "Register MAXI `reset()`'s with value 0"]
impl crate::ResetValue for super::MAXI {
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
#[doc = "Reader of field `USBMAXI`"]
pub type USBMAXI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBMAXI`"]
pub struct USBMAXI_W<'a> {
    w: &'a mut W,
}
impl<'a> USBMAXI_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxi(&self) -> USBMAXI_R {
        USBMAXI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Maximum packet size, in units of 8 bytes, for the selected IN endpoint The value of this register should match the wMaxPacketSize field in the standard endpoint descriptor for the endpoint. The value must not exceed the available memory."]
    #[inline(always)]
    pub fn usbmaxi(&mut self) -> USBMAXI_W {
        USBMAXI_W { w: self }
    }
}
