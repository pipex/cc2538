#[doc = "Reader of register STCC"]
pub type R = crate::R<u32, super::STCC>;
#[doc = "Writer for register STCC"]
pub type W = crate::W<u32, super::STCC>;
#[doc = "Register STCC `reset()`'s with value 0"]
impl crate::ResetValue for super::STCC {
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
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `PORT`"]
pub type PORT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORT`"]
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN`"]
pub struct PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 3:5 - 5:3\\] Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\] Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\] Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W { w: self }
    }
}
