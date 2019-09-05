#[doc = "Reader of register FRML"]
pub type R = crate::R<u32, super::FRML>;
#[doc = "Writer for register FRML"]
pub type W = crate::W<u32, super::FRML>;
#[doc = "Register FRML `reset()`'s with value 0"]
impl crate::ResetValue for super::FRML {
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
#[doc = "Reader of field `FRAMEL`"]
pub type FRAMEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRAMEL`"]
pub struct FRAMEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEL_W<'a> {
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
    #[doc = "Bits 0:7 - 7:0\\] Bits 7:0 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
    #[inline(always)]
    pub fn framel(&self) -> FRAMEL_R {
        FRAMEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Bits 7:0 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
    #[inline(always)]
    pub fn framel(&mut self) -> FRAMEL_W {
        FRAMEL_W { w: self }
    }
}
