#[doc = "Reader of register CNTH"]
pub type R = crate::R<u32, super::CNTH>;
#[doc = "Writer for register CNTH"]
pub type W = crate::W<u32, super::CNTH>;
#[doc = "Register CNTH `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTH {
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
#[doc = "Reader of field `Reserved8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `FIFOCNTH`"]
pub type FIFOCNTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFOCNTH`"]
pub struct FIFOCNTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCNTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 3:7 - 7:3\\] Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\] Bits 10:8 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
    #[inline(always)]
    pub fn fifocnth(&self) -> FIFOCNTH_R {
        FIFOCNTH_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Bits 10:8 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
    #[inline(always)]
    pub fn fifocnth(&mut self) -> FIFOCNTH_W {
        FIFOCNTH_W { w: self }
    }
}
