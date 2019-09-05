#[doc = "Reader of register CSOH"]
pub type R = crate::R<u32, super::CSOH>;
#[doc = "Writer for register CSOH"]
pub type W = crate::W<u32, super::CSOH>;
#[doc = "Register CSOH `reset()`'s with value 0"]
impl crate::ResetValue for super::CSOH {
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
#[doc = "Reader of field `AUTOCLEAR`"]
pub type AUTOCLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCLEAR`"]
pub struct AUTOCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCLEAR_W<'a> {
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
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `OUTDBLBUF`"]
pub type OUTDBLBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTDBLBUF`"]
pub struct OUTDBLBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDBLBUF_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
    #[inline(always)]
    pub fn autoclear(&self) -> AUTOCLEAR_R {
        AUTOCLEAR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
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
    #[doc = "Bits 1:3 - 3:1\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] OUT endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    pub fn outdblbuf(&self) -> OUTDBLBUF_R {
        OUTDBLBUF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] If software sets this bit, the USB_CSOL.OUTPKTRDY bit will be automatically cleared when a packet of maximum size (specified by USB_MAXO) has been unloaded from the OUT FIFO. When packets of less than the maximum packet size are unloaded, USB_CSOL.OUTPKTRDY will have to be cleared manually."]
    #[inline(always)]
    pub fn autoclear(&mut self) -> AUTOCLEAR_W {
        AUTOCLEAR_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Selects OUT endpoint type: 0: Bulk/interrupt 1: Isochronous"]
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
    #[doc = "Bits 1:3 - 3:1\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] OUT endpoint FIFO double-buffering enable: 0: Double buffering disabled 1: Double buffering enabled"]
    #[inline(always)]
    pub fn outdblbuf(&mut self) -> OUTDBLBUF_W {
        OUTDBLBUF_W { w: self }
    }
}
