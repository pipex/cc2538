#[doc = "Reader of register IIF"]
pub type R = crate::R<u32, super::IIF>;
#[doc = "Writer for register IIF"]
pub type W = crate::W<u32, super::IIF>;
#[doc = "Register IIF `reset()`'s with value 0"]
impl crate::ResetValue for super::IIF {
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
#[doc = "Reader of field `INEP5IF`"]
pub type INEP5IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP5IF`"]
pub struct INEP5IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP5IF_W<'a> {
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
#[doc = "Reader of field `INEP4IF`"]
pub type INEP4IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP4IF`"]
pub struct INEP4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP4IF_W<'a> {
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
#[doc = "Reader of field `INEP3IF`"]
pub type INEP3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP3IF`"]
pub struct INEP3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP3IF_W<'a> {
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
#[doc = "Reader of field `INEP2IF`"]
pub type INEP2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP2IF`"]
pub struct INEP2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP2IF_W<'a> {
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
#[doc = "Reader of field `INEP1IF`"]
pub type INEP1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INEP1IF`"]
pub struct INEP1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INEP1IF_W<'a> {
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
#[doc = "Reader of field `EP0IF`"]
pub type EP0IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0IF`"]
pub struct EP0IF_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0IF_W<'a> {
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
    #[doc = "Bit 5 - 5:5\\] Interrupt flag for IN endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep5if(&self) -> INEP5IF_R {
        INEP5IF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt flag for IN endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep4if(&self) -> INEP4IF_R {
        INEP4IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt flag for IN endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep3if(&self) -> INEP3IF_R {
        INEP3IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt flag for IN endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep2if(&self) -> INEP2IF_R {
        INEP2IF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt flag for IN endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep1if(&self) -> INEP1IF_R {
        INEP1IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Interrupt flag for endpoint 0 Cleared by hardware when read"]
    #[inline(always)]
    pub fn ep0if(&self) -> EP0IF_R {
        EP0IF_R::new((self.bits & 0x01) != 0)
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
    #[doc = "Bit 5 - 5:5\\] Interrupt flag for IN endpoint 5 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep5if(&mut self) -> INEP5IF_W {
        INEP5IF_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt flag for IN endpoint 4 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep4if(&mut self) -> INEP4IF_W {
        INEP4IF_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt flag for IN endpoint 3 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep3if(&mut self) -> INEP3IF_W {
        INEP3IF_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt flag for IN endpoint 2 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep2if(&mut self) -> INEP2IF_W {
        INEP2IF_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt flag for IN endpoint 1 Cleared by hardware when read"]
    #[inline(always)]
    pub fn inep1if(&mut self) -> INEP1IF_W {
        INEP1IF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Interrupt flag for endpoint 0 Cleared by hardware when read"]
    #[inline(always)]
    pub fn ep0if(&mut self) -> EP0IF_W {
        EP0IF_W { w: self }
    }
}
