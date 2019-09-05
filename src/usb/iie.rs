#[doc = "Reader of register IIE"]
pub type R = crate::R<u32, super::IIE>;
#[doc = "Writer for register IIE"]
pub type W = crate::W<u32, super::IIE>;
#[doc = "Register IIE `reset()`'s with value 0"]
impl crate::ResetValue for super::IIE {
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `INEP5IE`"]
pub type INEP5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP5IE`"]
pub struct INEP5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP5IE_W<'a> {
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
#[doc = "Reader of field `INEP4IE`"]
pub type INEP4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP4IE`"]
pub struct INEP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP4IE_W<'a> {
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
#[doc = "Reader of field `INEP3IE`"]
pub type INEP3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP3IE`"]
pub struct INEP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP3IE_W<'a> {
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
#[doc = "Reader of field `INEP2IE`"]
pub type INEP2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP2IE`"]
pub struct INEP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP2IE_W<'a> {
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
#[doc = "Reader of field `INEP1IE`"]
pub type INEP1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP1IE`"]
pub struct INEP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP1IE_W<'a> {
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
#[doc = "Reader of field `EP0IE`"]
pub type EP0IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0IE`"]
pub struct EP0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0IE_W<'a> {
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
    #[doc = "Bits 6:7 - 7:6\\] Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\] Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep5ie(&self) -> INEP5IE_R {
        INEP5IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep4ie(&self) -> INEP4IE_R {
        INEP4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep3ie(&self) -> INEP3IE_R {
        INEP3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep2ie(&self) -> INEP2IE_R {
        INEP2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep1ie(&self) -> INEP1IE_R {
        INEP1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn ep0ie(&self) -> EP0IE_R {
        EP0IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep5ie(&mut self) -> INEP5IE_W {
        INEP5IE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep4ie(&mut self) -> INEP4IE_W {
        INEP4IE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep3ie(&mut self) -> INEP3IE_W {
        INEP3IE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep2ie(&mut self) -> INEP2IE_W {
        INEP2IE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep1ie(&mut self) -> INEP1IE_W {
        INEP1IE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn ep0ie(&mut self) -> EP0IE_W {
        EP0IE_W { w: self }
    }
}
