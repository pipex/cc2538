#[doc = "Reader of register F3"]
pub type R = crate::R<u32, super::F3>;
#[doc = "Writer for register F3"]
pub type W = crate::W<u32, super::F3>;
#[doc = "Register F3 `reset()`'s with value 0"]
impl crate::ResetValue for super::F3 {
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
#[doc = "Reader of field `USBF3`"]
pub type USBF3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBF3`"]
pub struct USBF3_W<'a> {
    w: &'a mut W,
}
impl<'a> USBF3_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
    #[inline(always)]
    pub fn usbf3(&self) -> USBF3_R {
        USBF3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Endpoint 3 FIFO register Reading this register unloads one byte from the EP3 OUT FIFO. Writing to this register loads one byte into the EP3 IN FIFO."]
    #[inline(always)]
    pub fn usbf3(&mut self) -> USBF3_W {
        USBF3_W { w: self }
    }
}
