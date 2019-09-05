#[doc = "Reader of register OIE"]
pub type R = crate::R<u32, super::OIE>;
#[doc = "Writer for register OIE"]
pub type W = crate::W<u32, super::OIE>;
#[doc = "Register OIE `reset()`'s with value 0"]
impl crate::ResetValue for super::OIE {
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
#[doc = "Reader of field `reserved8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reserved8`"]
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
#[doc = "Reader of field `OUTEP5IE`"]
pub type OUTEP5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP5IE`"]
pub struct OUTEP5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP5IE_W<'a> {
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
#[doc = "Reader of field `OUTEP4IE`"]
pub type OUTEP4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP4IE`"]
pub struct OUTEP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP4IE_W<'a> {
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
#[doc = "Reader of field `OUTEP3IE`"]
pub type OUTEP3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP3IE`"]
pub struct OUTEP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP3IE_W<'a> {
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
#[doc = "Reader of field `OUTEP2IE`"]
pub type OUTEP2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP2IE`"]
pub struct OUTEP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP2IE_W<'a> {
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
#[doc = "Reader of field `OUTEP1IE`"]
pub type OUTEP1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP1IE`"]
pub struct OUTEP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP1IE_W<'a> {
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
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
    #[doc = "Bit 5 - 5:5\\] Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&self) -> OUTEP5IE_R {
        OUTEP5IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&self) -> OUTEP4IE_R {
        OUTEP4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&self) -> OUTEP3IE_R {
        OUTEP3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&self) -> OUTEP2IE_R {
        OUTEP2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&self) -> OUTEP1IE_R {
        OUTEP1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits & 0x01) != 0)
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
    #[doc = "Bit 5 - 5:5\\] Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&mut self) -> OUTEP5IE_W {
        OUTEP5IE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&mut self) -> OUTEP4IE_W {
        OUTEP4IE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&mut self) -> OUTEP3IE_W {
        OUTEP3IE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&mut self) -> OUTEP2IE_W {
        OUTEP2IE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&mut self) -> OUTEP1IE_W {
        OUTEP1IE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
}
