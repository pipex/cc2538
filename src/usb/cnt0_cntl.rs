#[doc = "Reader of register CNT0_CNTL"]
pub type R = crate::R<u32, super::CNT0_CNTL>;
#[doc = "Writer for register CNT0_CNTL"]
pub type W = crate::W<u32, super::CNT0_CNTL>;
#[doc = "Register CNT0_CNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CNT0_CNTL {
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
#[doc = "Reader of field `FIFOCNT_or_FIFOCNTL`"]
pub type FIFOCNT_OR_FIFOCNTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOCNT_or_FIFOCNTL`"]
pub struct FIFOCNT_OR_FIFOCNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCNT_OR_FIFOCNTL_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] USB_CS0.FIFOCNT (USBINDEX = 0) \\[RO\\]: Number of bytes received in the packet in the endpoint 0 FIFO Valid only when USB_CS0.OUTPKTRDY is set USB_CSIL.FIFOCNTL (USBINDEX = 1 to 5) \\[RW\\]: Bits 7:0 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
    #[inline(always)]
    pub fn fifocnt_or_fifocntl(&self) -> FIFOCNT_OR_FIFOCNTL_R {
        FIFOCNT_OR_FIFOCNTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] USB_CS0.FIFOCNT (USBINDEX = 0) \\[RO\\]: Number of bytes received in the packet in the endpoint 0 FIFO Valid only when USB_CS0.OUTPKTRDY is set USB_CSIL.FIFOCNTL (USBINDEX = 1 to 5) \\[RW\\]: Bits 7:0 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
    #[inline(always)]
    pub fn fifocnt_or_fifocntl(&mut self) -> FIFOCNT_OR_FIFOCNTL_W {
        FIFOCNT_OR_FIFOCNTL_W { w: self }
    }
}
