#[doc = "Reader of register RCGCSEC"]
pub type R = crate::R<u32, super::RCGCSEC>;
#[doc = "Writer for register RCGCSEC"]
pub type W = crate::W<u32, super::RCGCSEC>;
#[doc = "Register RCGCSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::RCGCSEC {
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
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
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
#[doc = "Reader of field `PKA`"]
pub type PKA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA`"]
pub struct PKA_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_W<'a> {
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
    #[doc = "Bit 1 - 1:1\\] 0: Clock for AES is gated. 1: Clock for AES is enabled."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Clock for AES is gated. 1: Clock for AES is enabled."]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Clock for PKA is gated. 1: Clock for PKA is enabled."]
    #[inline(always)]
    pub fn pka(&mut self) -> PKA_W {
        PKA_W { w: self }
    }
}
