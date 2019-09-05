#[doc = "Reader of register CS0_CSIL"]
pub type R = crate::R<u32, super::CS0_CSIL>;
#[doc = "Writer for register CS0_CSIL"]
pub type W = crate::W<u32, super::CS0_CSIL>;
#[doc = "Register CS0_CSIL `reset()`'s with value 0"]
impl crate::ResetValue for super::CS0_CSIL {
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
#[doc = "Reader of field `CLRSETUPEND_or_Reserved8`"]
pub type CLRSETUPEND_OR_RESERVED8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRSETUPEND_or_Reserved8`"]
pub struct CLRSETUPEND_OR_RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSETUPEND_OR_RESERVED8_W<'a> {
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
#[doc = "Reader of field `CLROUTPKTRDY_or_CLRDATATOG`"]
pub type CLROUTPKTRDY_OR_CLRDATATOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLROUTPKTRDY_or_CLRDATATOG`"]
pub struct CLROUTPKTRDY_OR_CLRDATATOG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLROUTPKTRDY_OR_CLRDATATOG_W<'a> {
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
#[doc = "Reader of field `SENDSTALL_or_SENTSTALL`"]
pub type SENDSTALL_OR_SENTSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENDSTALL_or_SENTSTALL`"]
pub struct SENDSTALL_OR_SENTSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDSTALL_OR_SENTSTALL_W<'a> {
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
#[doc = "Reader of field `SETUPEND_or_SENDSTALL`"]
pub type SETUPEND_OR_SENDSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUPEND_or_SENDSTALL`"]
pub struct SETUPEND_OR_SENDSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUPEND_OR_SENDSTALL_W<'a> {
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
#[doc = "Reader of field `DATAEND_or_FLUSHPACKET`"]
pub type DATAEND_OR_FLUSHPACKET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAEND_or_FLUSHPACKET`"]
pub struct DATAEND_OR_FLUSHPACKET_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAEND_OR_FLUSHPACKET_W<'a> {
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
#[doc = "Reader of field `SENTSTALL_or_UNDERRUN`"]
pub type SENTSTALL_OR_UNDERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENTSTALL_or_UNDERRUN`"]
pub struct SENTSTALL_OR_UNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENTSTALL_OR_UNDERRUN_W<'a> {
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
#[doc = "Reader of field `INPKTRDY_or_PKTPRESENT`"]
pub type INPKTRDY_OR_PKTPRESENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INPKTRDY_or_PKTPRESENT`"]
pub struct INPKTRDY_OR_PKTPRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> INPKTRDY_OR_PKTPRESENT_W<'a> {
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
#[doc = "Reader of field `OUTPKTRDY_or_INPKTRDY`"]
pub type OUTPKTRDY_OR_INPKTRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTPKTRDY_or_INPKTRDY`"]
pub struct OUTPKTRDY_OR_INPKTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPKTRDY_OR_INPKTRDY_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
    #[inline(always)]
    pub fn clrsetupend_or_reserved8(&self) -> CLRSETUPEND_OR_RESERVED8_R {
        CLRSETUPEND_OR_RESERVED8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clroutpktrdy_or_clrdatatog(&self) -> CLROUTPKTRDY_OR_CLRDATATOG_R {
        CLROUTPKTRDY_OR_CLRDATATOG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
    #[inline(always)]
    pub fn sendstall_or_sentstall(&self) -> SENDSTALL_OR_SENTSTALL_R {
        SENDSTALL_OR_SENTSTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] USB_CS0.SETUPEND \\[RO\\]: This bit is set when a control transaction ends before the USB_CS0.DATAEND bit has been set. An interrupt is generated and the FIFO flushed at this time. Software clears this bit by setting USB_CS0.CLRSETUPEND. CSIL.SENDSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn setupend_or_sendstall(&self) -> SETUPEND_OR_SENDSTALL_R {
        SETUPEND_OR_SENDSTALL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn dataend_or_flushpacket(&self) -> DATAEND_OR_FLUSHPACKET_R {
        DATAEND_OR_FLUSHPACKET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall_or_underrun(&self) -> SENTSTALL_OR_UNDERRUN_R {
        SENTSTALL_OR_UNDERRUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
    #[inline(always)]
    pub fn inpktrdy_or_pktpresent(&self) -> INPKTRDY_OR_PKTPRESENT_R {
        INPKTRDY_OR_PKTPRESENT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] USB_CS0.OUTPKTRDY \\[RO\\]: Endpoint 0 data packet received An interrupt request (EP0) is generated if the interrupt is enabled. Software must read the endpoint 0 FIFO empty, and clear this bit by setting USB_CS0.CLROUTPKTRDY USB_CSIL.INPKTRDY \\[RW\\]: IN endpoint {1-5} packet transfer pending Software sets this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. An interrupt is generated (if enabled) when the bit is cleared. When using double-buffering, the bit is cleared immediately if the other FIFO is empty."]
    #[inline(always)]
    pub fn outpktrdy_or_inpktrdy(&self) -> OUTPKTRDY_OR_INPKTRDY_R {
        OUTPKTRDY_OR_INPKTRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] USB_CS0.CLRSETUPEND \\[RW\\]: Software sets this bit to clear the USB_CS0.SETUPEND bit. It is cleared automatically. USB_CSIL.Reserved \\[RO\\]: Reserved"]
    #[inline(always)]
    pub fn clrsetupend_or_reserved8(&mut self) -> CLRSETUPEND_OR_RESERVED8_W {
        CLRSETUPEND_OR_RESERVED8_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] USB_CS0.CLROUTPKTRDY \\[RW\\]: Software sets this bit to clear the USB_CS0.OUTPKTRDY bit. It is cleared automatically. USB_CSIL.CLRDATATOG \\[RW\\]: Software sets this bit to reset the IN endpoint data toggle to 0."]
    #[inline(always)]
    pub fn clroutpktrdy_or_clrdatatog(&mut self) -> CLROUTPKTRDY_OR_CLRDATATOG_W {
        CLROUTPKTRDY_OR_CLRDATATOG_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] USB_CS0.SENDSTALL \\[RW\\]: Software sets this bit to terminate the current transaction with a STALL handshake. The bit is cleared automatically when the STALL handshake has been transmitted. USB_CSIL.SENTSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: This bit is set when a STALL handshake is transmitted. The FIFO is flushed and the USB_CSIL.INPKTRDY bit cleared. Software should clear this bit."]
    #[inline(always)]
    pub fn sendstall_or_sentstall(&mut self) -> SENDSTALL_OR_SENTSTALL_W {
        SENDSTALL_OR_SENTSTALL_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] USB_CS0.SETUPEND \\[RO\\]: This bit is set when a control transaction ends before the USB_CS0.DATAEND bit has been set. An interrupt is generated and the FIFO flushed at this time. Software clears this bit by setting USB_CS0.CLRSETUPEND. CSIL.SENDSTALL \\[RW\\]: For bulk/interrupt mode IN endpoints: Software sets this bit to issue a STALL handshake. Software clears this bit to terminate the stall condition."]
    #[inline(always)]
    pub fn setupend_or_sendstall(&mut self) -> SETUPEND_OR_SENDSTALL_W {
        SETUPEND_OR_SENDSTALL_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] USB_CS0.DATAEND \\[RW\\]: This bit is used to signal the end of the data stage, and must be set: 1. When the last data packet is loaded and USB_CS0.INPKTRDY is set. 2. When the last data packet is unloaded and USB_CS0.CLROUTPKTRDY is set. 3. When USB_CS0.INPKTRDY is set to send a zero-length packet. The USB controller clears this bit automatically. USB_CSIL.FLUSHPACKET \\[RW\\]: Software sets this bit to flush the next packet to be transmitted from the IN endpoint FIFO. The FIFO pointer is reset and the USB_CSIL.INPKTRDY bit is cleared. Note: If the FIFO contains two packets, USB_CSIL.FLUSHPACKET will need to be set twice to completely clear the FIFO."]
    #[inline(always)]
    pub fn dataend_or_flushpacket(&mut self) -> DATAEND_OR_FLUSHPACKET_W {
        DATAEND_OR_FLUSHPACKET_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] USB_CS0.SENTSTALL \\[RW\\]: This bit is set when a STALL handshake is sent. An interrupt is generated is generated when this bit is set. Software must clear this bit. USB_CSIL.UNDERRUN \\[RW\\]: In isochronous mode, this bit is set when a zero length data packet is sent after receiving an IN token with USB_CSIL.INPKTRDY not set. In bulk/interrupt mode, this bit is set when a NAK is returned in response to an IN token. Software should clear this bit."]
    #[inline(always)]
    pub fn sentstall_or_underrun(&mut self) -> SENTSTALL_OR_UNDERRUN_W {
        SENTSTALL_OR_UNDERRUN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] USB_CS0. INPKTRDY \\[RW\\]: Software sets this bit after loading a data packet into the endpoint 0 FIFO. It is cleared automatically when the data packet has been transmitted. An interrupt is generated when the bit is cleared. USB_CSIL.PKTPRESENT \\[RO\\]: This bit is set when there is at least one packet in the IN endpoint FIFO."]
    #[inline(always)]
    pub fn inpktrdy_or_pktpresent(&mut self) -> INPKTRDY_OR_PKTPRESENT_W {
        INPKTRDY_OR_PKTPRESENT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] USB_CS0.OUTPKTRDY \\[RO\\]: Endpoint 0 data packet received An interrupt request (EP0) is generated if the interrupt is enabled. Software must read the endpoint 0 FIFO empty, and clear this bit by setting USB_CS0.CLROUTPKTRDY USB_CSIL.INPKTRDY \\[RW\\]: IN endpoint {1-5} packet transfer pending Software sets this bit after loading a data packet into the FIFO. It is cleared automatically when a data packet has been transmitted. An interrupt is generated (if enabled) when the bit is cleared. When using double-buffering, the bit is cleared immediately if the other FIFO is empty."]
    #[inline(always)]
    pub fn outpktrdy_or_inpktrdy(&mut self) -> OUTPKTRDY_OR_INPKTRDY_W {
        OUTPKTRDY_OR_INPKTRDY_W { w: self }
    }
}
