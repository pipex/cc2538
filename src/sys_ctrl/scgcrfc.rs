#[doc = "Reader of register SCGCRFC"]
pub type R = crate::R<u32, super::SCGCRFC>;
#[doc = "Writer for register SCGCRFC"]
pub type W = crate::W<u32, super::SCGCRFC>;
#[doc = "Register SCGCRFC `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGCRFC {
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
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `RFC0`"]
pub type RFC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC0`"]
pub struct RFC0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC0_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\] 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    pub fn rfc0(&self) -> RFC0_R {
        RFC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    pub fn rfc0(&mut self) -> RFC0_W {
        RFC0_W { w: self }
    }
}
