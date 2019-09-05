#[doc = "Reader of register DCGCSSI"]
pub type R = crate::R<u32, super::DCGCSSI>;
#[doc = "Writer for register DCGCSSI"]
pub type W = crate::W<u32, super::DCGCSSI>;
#[doc = "Register DCGCSSI `reset()`'s with value 0"]
impl crate::ResetValue for super::DCGCSSI {
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
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `SSI1`"]
pub type SSI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI1`"]
pub struct SSI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI1_W<'a> {
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
#[doc = "Reader of field `SSI0`"]
pub type SSI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI0`"]
pub struct SSI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI0_W<'a> {
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
    #[doc = "Bits 2:31 - 31:2\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\] 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
    #[inline(always)]
    pub fn ssi1(&self) -> SSI1_R {
        SSI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
    #[inline(always)]
    pub fn ssi0(&self) -> SSI0_R {
        SSI0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Clock for SSI1 is gated. 1: Clock for SSI1 is enabled."]
    #[inline(always)]
    pub fn ssi1(&mut self) -> SSI1_W {
        SSI1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Clock for SSI0 is gated. 1: Clock for SSI0 is enabled."]
    #[inline(always)]
    pub fn ssi0(&mut self) -> SSI0_W {
        SSI0_W { w: self }
    }
}
