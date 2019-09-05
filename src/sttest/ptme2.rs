#[doc = "Reader of register PTME2"]
pub type R = crate::R<u32, super::PTME2>;
#[doc = "Writer for register PTME2"]
pub type W = crate::W<u32, super::PTME2>;
#[doc = "Register PTME2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PTME2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Reader of field `T3TME`"]
pub type T3TME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T3TME`"]
pub struct T3TME_W<'a> {
    w: &'a mut W,
}
impl<'a> T3TME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MTTME`"]
pub type MTTME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTTME`"]
pub struct MTTME_W<'a> {
    w: &'a mut W,
}
impl<'a> MTTME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `T1TME`"]
pub type T1TME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1TME`"]
pub struct T1TME_W<'a> {
    w: &'a mut W,
}
impl<'a> T1TME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `T0TME`"]
pub type T0TME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T0TME`"]
pub struct T0TME_W<'a> {
    w: &'a mut W,
}
impl<'a> T0TME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | (((value as u32) & 0x7fff) << 1);
        self.w
    }
}
#[doc = "Reader of field `I2C0TME`"]
pub type I2C0TME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0TME`"]
pub struct I2C0TME_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0TME_W<'a> {
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
    #[doc = "Bits 20:31 - 31:20\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bit 19 - 19:19\\] Timer3 test mode enable"]
    #[inline(always)]
    pub fn t3tme(&self) -> T3TME_R {
        T3TME_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\] MacTimer test mode enable"]
    #[inline(always)]
    pub fn mttme(&self) -> MTTME_R {
        MTTME_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\] Timer1 test mode enable"]
    #[inline(always)]
    pub fn t1tme(&self) -> T1TME_R {
        T1TME_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\] Timer0 test mode enable"]
    #[inline(always)]
    pub fn t0tme(&self) -> T0TME_R {
        T0TME_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 1:15 - 15:1\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 0 - 0:0\\] I2C 0 test mode enable"]
    #[inline(always)]
    pub fn i2c0tme(&self) -> I2C0TME_R {
        I2C0TME_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Timer3 test mode enable"]
    #[inline(always)]
    pub fn t3tme(&mut self) -> T3TME_W {
        T3TME_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] MacTimer test mode enable"]
    #[inline(always)]
    pub fn mttme(&mut self) -> MTTME_W {
        MTTME_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Timer1 test mode enable"]
    #[inline(always)]
    pub fn t1tme(&mut self) -> T1TME_W {
        T1TME_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Timer0 test mode enable"]
    #[inline(always)]
    pub fn t0tme(&mut self) -> T0TME_W {
        T0TME_W { w: self }
    }
    #[doc = "Bits 1:15 - 15:1\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] I2C 0 test mode enable"]
    #[inline(always)]
    pub fn i2c0tme(&mut self) -> I2C0TME_W {
        I2C0TME_W { w: self }
    }
}
