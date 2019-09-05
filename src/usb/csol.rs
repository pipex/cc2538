#[doc = "Reader of register CSOL"]
pub type R = crate::R<u32, super::CSOL>;
#[doc = "Writer for register CSOL"]
pub type W = crate::W<u32, super::CSOL>;
#[doc = "Register CSOL `reset()`'s with value 0"]
impl crate::ResetValue for super::CSOL {
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
#[doc = "Reader of field `CLRDATATOG`"]
pub type CLRDATATOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRDATATOG`"]
pub struct CLRDATATOG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRDATATOG_W<'a> {
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
#[doc = "Reader of field `SENTSTALL`"]
pub type SENTSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENTSTALL`"]
pub struct SENTSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENTSTALL_W<'a> {
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
#[doc = "Reader of field `SENDSTALL`"]
pub type SENDSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENDSTALL`"]
pub struct SENDSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDSTALL_W<'a> {
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
#[doc = "Reader of field `FLUSHPACKET`"]
pub type FLUSHPACKET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUSHPACKET`"]
pub struct FLUSHPACKET_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHPACKET_W<'a> {
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
#[doc = "Reader of field `DATAERROR`"]
pub type DATAERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAERROR`"]
pub struct DATAERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAERROR_W<'a> {
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
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERRUN`"]
pub struct OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_W<'a> {
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
#[doc = "Reader of field `FIFOFULL`"]
pub type FIFOFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOFULL`"]
pub struct FIFOFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOFULL_W<'a> {
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
#[doc = "Reader of field `OUTPKTRDY`"]
pub type OUTPKTRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTPKTRDY`"]
pub struct OUTPKTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPKTRDY_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Software sets this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&self) -> CLRDATATOG_R {
        CLRDATATOG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall(&self) -> SENTSTALL_R {
        SENTSTALL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn sendstall(&self) -> SENDSTALL_R {
        SENDSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn flushpacket(&self) -> FLUSHPACKET_R {
        FLUSHPACKET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] For isochronous mode OUT endpoints: This bit is set when USB_CSOL.OUTPKTRDY is set if the data packet has a CRC or bit-stuff error. It is cleared automatically when USB_CSOL.OUTPKTRDY is cleared."]
    #[inline(always)]
    pub fn dataerror(&self) -> DATAERROR_R {
        DATAERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] This bit is set when no more packets can be loaded into the OUT endpoint FIFO."]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OUTPKTRDY_R {
        OUTPKTRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Software sets this bit to reset the endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> CLRDATATOG_W {
        CLRDATATOG_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] This bit is set when a STALL handshake is transmitted. An interrupt is generated when this bit is set. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SENTSTALL_W {
        SENTSTALL_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] For bulk/interrupt mode OUT endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SENDSTALL_W {
        SENDSTALL_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Software sets this bit to flush the next packet to be read from the endpoint OUT FIFO. Note: If the FIFO contains two packets, USB_CSOL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn flushpacket(&mut self) -> FLUSHPACKET_W {
        FLUSHPACKET_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] For isochronous mode OUT endpoints: This bit is set when USB_CSOL.OUTPKTRDY is set if the data packet has a CRC or bit-stuff error. It is cleared automatically when USB_CSOL.OUTPKTRDY is cleared."]
    #[inline(always)]
    pub fn dataerror(&mut self) -> DATAERROR_W {
        DATAERROR_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] For isochronous mode OUT endpoints: This bit is set when an OUT packet cannot be loaded into the OUT endpoint FIFO. Firmware should clear this bit."]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] This bit is set when no more packets can be loaded into the OUT endpoint FIFO."]
    #[inline(always)]
    pub fn fifofull(&mut self) -> FIFOFULL_W {
        FIFOFULL_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] This bit is set when a data packet has been received. Software should clear this bit when the packet has been unloaded from the OUT endpoint FIFO. An interrupt is generated when the bit is set."]
    #[inline(always)]
    pub fn outpktrdy(&mut self) -> OUTPKTRDY_W {
        OUTPKTRDY_W { w: self }
    }
}
