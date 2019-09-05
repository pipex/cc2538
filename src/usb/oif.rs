#[doc = "Reader of register OIF"]
pub type R = crate::R<u32, super::OIF>;
#[doc = "Writer for register OIF"]
pub type W = crate::W<u32, super::OIF>;
#[doc = "Register OIF `reset()`'s with value 0"]
impl crate::ResetValue for super::OIF {
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
#[doc = "Reader of field `OUTEP5IF`"]
pub type OUTEP5IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP5IF`"]
pub struct OUTEP5IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP5IF_W<'a> {
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
#[doc = "Reader of field `OUTEP4IF`"]
pub type OUTEP4IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP4IF`"]
pub struct OUTEP4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP4IF_W<'a> {
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
#[doc = "Reader of field `OUTEP3IF`"]
pub type OUTEP3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP3IF`"]
pub struct OUTEP3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP3IF_W<'a> {
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
#[doc = "Reader of field `OUTEP2IF`"]
pub type OUTEP2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP2IF`"]
pub struct OUTEP2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP2IF_W<'a> {
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
#[doc = "Reader of field `OUTEP1IF`"]
pub type OUTEP1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTEP1IF`"]
pub struct OUTEP1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTEP1IF_W<'a> {
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
    #[doc = "Bit 5 - 5:5\\] Interrupt flag for OUT endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep5if(&self) -> OUTEP5IF_R {
        OUTEP5IF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt flag for OUT endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep4if(&self) -> OUTEP4IF_R {
        OUTEP4IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt flag for OUT endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep3if(&self) -> OUTEP3IF_R {
        OUTEP3IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt flag for OUT endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep2if(&self) -> OUTEP2IF_R {
        OUTEP2IF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt flag for OUT endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep1if(&self) -> OUTEP1IF_R {
        OUTEP1IF_R::new(((self.bits >> 1) & 0x01) != 0)
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
    #[doc = "Bit 5 - 5:5\\] Interrupt flag for OUT endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep5if(&mut self) -> OUTEP5IF_W {
        OUTEP5IF_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt flag for OUT endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep4if(&mut self) -> OUTEP4IF_W {
        OUTEP4IF_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt flag for OUT endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep3if(&mut self) -> OUTEP3IF_W {
        OUTEP3IF_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt flag for OUT endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep2if(&mut self) -> OUTEP2IF_W {
        OUTEP2IF_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt flag for OUT endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn outep1if(&mut self) -> OUTEP1IF_W {
        OUTEP1IF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
}
