#[doc = "Reader of register MTIRQM"]
pub type R = crate::R<u32, super::MTIRQM>;
#[doc = "Writer for register MTIRQM"]
pub type W = crate::W<u32, super::MTIRQM>;
#[doc = "Register MTIRQM `reset()`'s with value 0"]
impl crate::ResetValue for super::MTIRQM {
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
#[doc = "Reader of field `MACTIMER_OVF_COMPARE2M`"]
pub type MACTIMER_OVF_COMPARE2M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MACTIMER_OVF_COMPARE2M`"]
pub struct MACTIMER_OVF_COMPARE2M_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_COMPARE2M_W<'a> {
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
#[doc = "Reader of field `MACTIMER_OVF_COMPARE1M`"]
pub type MACTIMER_OVF_COMPARE1M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MACTIMER_OVF_COMPARE1M`"]
pub struct MACTIMER_OVF_COMPARE1M_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_COMPARE1M_W<'a> {
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
#[doc = "Reader of field `MACTIMER_OVF_PERM`"]
pub type MACTIMER_OVF_PERM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MACTIMER_OVF_PERM`"]
pub struct MACTIMER_OVF_PERM_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_PERM_W<'a> {
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
#[doc = "Reader of field `MACTIMER_COMPARE2M`"]
pub type MACTIMER_COMPARE2M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MACTIMER_COMPARE2M`"]
pub struct MACTIMER_COMPARE2M_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_COMPARE2M_W<'a> {
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
#[doc = "Reader of field `MACTIMER_COMPARE1M`"]
pub type MACTIMER_COMPARE1M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MACTIMER_COMPARE1M`"]
pub struct MACTIMER_COMPARE1M_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_COMPARE1M_W<'a> {
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
#[doc = "Reader of field `MACTIMER_PERM`"]
pub type MACTIMER_PERM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MACTIMER_PERM`"]
pub struct MACTIMER_PERM_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_PERM_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\] Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2m(&self) -> MACTIMER_OVF_COMPARE2M_R {
        MACTIMER_OVF_COMPARE2M_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1m(&self) -> MACTIMER_OVF_COMPARE1M_R {
        MACTIMER_OVF_COMPARE1M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_perm(&self) -> MACTIMER_OVF_PERM_R {
        MACTIMER_OVF_PERM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare2m(&self) -> MACTIMER_COMPARE2M_R {
        MACTIMER_COMPARE2M_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare1m(&self) -> MACTIMER_COMPARE1M_R {
        MACTIMER_COMPARE1M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_perm(&self) -> MACTIMER_PERM_R {
        MACTIMER_PERM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2m(&mut self) -> MACTIMER_OVF_COMPARE2M_W {
        MACTIMER_OVF_COMPARE2M_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1m(&mut self) -> MACTIMER_OVF_COMPARE1M_W {
        MACTIMER_OVF_COMPARE1M_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_perm(&mut self) -> MACTIMER_OVF_PERM_W {
        MACTIMER_OVF_PERM_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare2m(&mut self) -> MACTIMER_COMPARE2M_W {
        MACTIMER_COMPARE2M_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare1m(&mut self) -> MACTIMER_COMPARE1M_W {
        MACTIMER_COMPARE1M_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_perm(&mut self) -> MACTIMER_PERM_W {
        MACTIMER_PERM_W { w: self }
    }
}
