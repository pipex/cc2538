#[doc = "Reader of register CIF"]
pub type R = crate::R<u32, super::CIF>;
#[doc = "Writer for register CIF"]
pub type W = crate::W<u32, super::CIF>;
#[doc = "Register CIF `reset()`'s with value 0"]
impl crate::ResetValue for super::CIF {
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
#[doc = "Reader of field `SOFIF`"]
pub type SOFIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIF`"]
pub struct SOFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIF_W<'a> {
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
#[doc = "Reader of field `RSTIF`"]
pub type RSTIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIF`"]
pub struct RSTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIF_W<'a> {
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
#[doc = "Reader of field `RESUMEIF`"]
pub type RESUMEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUMEIF`"]
pub struct RESUMEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIF_W<'a> {
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
#[doc = "Reader of field `SUSPENDIF`"]
pub type SUSPENDIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPENDIF`"]
pub struct SUSPENDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPENDIF_W<'a> {
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
    #[doc = "Bit 3 - 3:3\\] Start-of-frame interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Reset interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn rstif(&self) -> RSTIF_R {
        RSTIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Resume interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn resumeif(&self) -> RESUMEIF_R {
        RESUMEIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Suspend interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn suspendif(&self) -> SUSPENDIF_R {
        SUSPENDIF_R::new((self.bits & 0x01) != 0)
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
    #[doc = "Bit 3 - 3:3\\] Start-of-frame interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn sofif(&mut self) -> SOFIF_W {
        SOFIF_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Reset interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn rstif(&mut self) -> RSTIF_W {
        RSTIF_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Resume interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn resumeif(&mut self) -> RESUMEIF_W {
        RESUMEIF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Suspend interrupt flag Cleared by hardware when read"]
    #[inline(always)]
    pub fn suspendif(&mut self) -> SUSPENDIF_W {
        SUSPENDIF_W { w: self }
    }
}
