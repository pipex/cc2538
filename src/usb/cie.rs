#[doc = "Reader of register CIE"]
pub type R = crate::R<u32, super::CIE>;
#[doc = "Writer for register CIE"]
pub type W = crate::W<u32, super::CIE>;
#[doc = "Register CIE `reset()`'s with value 0"]
impl crate::ResetValue for super::CIE {
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
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SOFIE`"]
pub type SOFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIE`"]
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
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
#[doc = "Reader of field `RSTIE`"]
pub type RSTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIE`"]
pub struct RSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIE_W<'a> {
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
#[doc = "Reader of field `RESUMEIE`"]
pub type RESUMEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUMEIE`"]
pub struct RESUMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIE_W<'a> {
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
#[doc = "Reader of field `SUSPENDIE`"]
pub type SUSPENDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPENDIE`"]
pub struct SUSPENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPENDIE_W<'a> {
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
    #[doc = "Bits 4:7 - 7:4\\] Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn resumeie(&self) -> RESUMEIE_R {
        RESUMEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn suspendie(&self) -> SUSPENDIE_R {
        SUSPENDIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W {
        RSTIE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn resumeie(&mut self) -> RESUMEIE_W {
        RESUMEIE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn suspendie(&mut self) -> SUSPENDIE_W {
        SUSPENDIE_W { w: self }
    }
}
