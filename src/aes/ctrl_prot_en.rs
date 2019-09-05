#[doc = "Reader of register CTRL_PROT_EN"]
pub type R = crate::R<u32, super::CTRL_PROT_EN>;
#[doc = "Writer for register CTRL_PROT_EN"]
pub type W = crate::W<u32, super::CTRL_PROT_EN>;
#[doc = "Register CTRL_PROT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_PROT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved31`"]
pub type RESERVED31_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved31`"]
pub struct RESERVED31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `PROT_EN`"]
pub type PROT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT_EN`"]
pub struct PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_EN_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\] Bits should be written with 0s and ignored on read."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\] If this bit is cleared to 0, m_h_prot\\[1\\] on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\] signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
    #[inline(always)]
    pub fn prot_en(&self) -> PROT_EN_R {
        PROT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\] Bits should be written with 0s and ignored on read."]
    #[inline(always)]
    pub fn reserved31(&mut self) -> RESERVED31_W {
        RESERVED31_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] If this bit is cleared to 0, m_h_prot\\[1\\] on the AHB mater interface always remains 0. If this bit is set to one, the m_h_prot\\[1\\] signal on the master AHB bus is asserted to 1 if an AHB read operation is performed, using DMA, with the key store module as destination."]
    #[inline(always)]
    pub fn prot_en(&mut self) -> PROT_EN_W {
        PROT_EN_W { w: self }
    }
}
