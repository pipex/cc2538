#[doc = "Reader of register CSIH"]
pub type R = crate::R<u32, super::CSIH>;
#[doc = "Writer for register CSIH"]
pub type W = crate::W<u32, super::CSIH>;
#[doc = "Register CSIH `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIH {
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
#[doc = "Reader of field `AUTISET`"]
pub type AUTISET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTISET`"]
pub struct AUTISET_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTISET_W<'a> {
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
#[doc = "Reader of field `ISO`"]
pub type ISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISO`"]
pub struct ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_W<'a> {
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
#[doc = "Reader of field `Reserved5`"]
pub type RESERVED5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
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
#[doc = "Reader of field `Reserved4`"]
pub type RESERVED4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
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
#[doc = "Reader of field `FORCEDATATOG`"]
pub type FORCEDATATOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEDATATOG`"]
pub struct FORCEDATATOG_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEDATATOG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `INDBLBUF`"]
pub type INDBLBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDBLBUF`"]
pub struct INDBLBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> INDBLBUF_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
    #[inline(always)]
    pub fn autiset(&self) -> AUTISET_R {
        AUTISET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
    #[inline(always)]
    pub fn forcedatatog(&self) -> FORCEDATATOG_R {
        FORCEDATATOG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    pub fn indblbuf(&self) -> INDBLBUF_R {
        INDBLBUF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] If set by software, the USB_CSIL.INPKTRDY bit is automatically set when a data packet of maximum size (specified by USBMAXI) is loaded into the IN endpoint FIFO. If a packet of less than the maximum packet size is loaded, then USB_CSIL.INPKTRDY will have to be set manually."]
    #[inline(always)]
    pub fn autiset(&mut self) -> AUTISET_W {
        AUTISET_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Selects IN endpoint type: 0: Bulk/interrupt 1: Isochronous"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W {
        ISO_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Software sets this bit to force the IN endpoint's data toggle to switch after each data packet is sent regardless of whether an ACK was received. This can be used by interrupt IN endpoints which are used to communicate rate feedback for isochronous endpoints."]
    #[inline(always)]
    pub fn forcedatatog(&mut self) -> FORCEDATATOG_W {
        FORCEDATATOG_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] IN endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    pub fn indblbuf(&mut self) -> INDBLBUF_W {
        INDBLBUF_W { w: self }
    }
}
