#[doc = "Reader of register POW"]
pub type R = crate::R<u32, super::POW>;
#[doc = "Writer for register POW"]
pub type W = crate::W<u32, super::POW>;
#[doc = "Register POW `reset()`'s with value 0"]
impl crate::ResetValue for super::POW {
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
#[doc = "Reader of field `ISOWAITSOF`"]
pub type ISOWAITSOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOWAITSOF`"]
pub struct ISOWAITSOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOWAITSOF_W<'a> {
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
#[doc = "Reader of field `Reserved7`"]
pub type RESERVED7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
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
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEND`"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
#[doc = "Reader of field `SUSPENDEN`"]
pub type SUSPENDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPENDEN`"]
pub struct SUSPENDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPENDEN_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\] For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
    #[inline(always)]
    pub fn isowaitsof(&self) -> ISOWAITSOF_R {
        ISOWAITSOF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\] Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] Indicates that reset signaling is present on the bus"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Indicates entry into suspend mode Suspend mode must be enabled by setting USB_POW.SUSPENDEN Software clears this bit by reading the USB_CIF register or by asserting USB_POW.RESUME"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Enables detection of and entry into suspend mode."]
    #[inline(always)]
    pub fn suspenden(&self) -> SUSPENDEN_R {
        SUSPENDEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] For isochronous mode IN endpoints: When set, the USB controller will wait for an SOF token from the time USB_CSIL.INPKTRDY is set before sending the packet. If an IN token is received before an SOF token, then a zero length data packet will be sent."]
    #[inline(always)]
    pub fn isowaitsof(&mut self) -> ISOWAITSOF_W {
        ISOWAITSOF_W { w: self }
    }
    #[doc = "Bits 4:6 - 6:4\\] Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Indicates that reset signaling is present on the bus"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Drives resume signaling for remote wakeup According to the USB Specification, the resume signal must be held active for at least 1 ms and no more than 15 ms. It is recommended to keep this bit set for approximately 10 ms."]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Indicates entry into suspend mode Suspend mode must be enabled by setting USB_POW.SUSPENDEN Software clears this bit by reading the USB_CIF register or by asserting USB_POW.RESUME"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enables detection of and entry into suspend mode."]
    #[inline(always)]
    pub fn suspenden(&mut self) -> SUSPENDEN_W {
        SUSPENDEN_W { w: self }
    }
}
