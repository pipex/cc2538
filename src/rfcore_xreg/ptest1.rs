#[doc = "Reader of register PTEST1"]
pub type R = crate::R<u32, super::PTEST1>;
#[doc = "Writer for register PTEST1"]
pub type W = crate::W<u32, super::PTEST1>;
#[doc = "Register PTEST1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PTEST1 {
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
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PD_OVERRIDE`"]
pub type PD_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_OVERRIDE`"]
pub struct PD_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `PA_PD`"]
pub type PA_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA_PD`"]
pub struct PA_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PD_W<'a> {
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
#[doc = "Reader of field `VCO_PD`"]
pub type VCO_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCO_PD`"]
pub struct VCO_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_PD_W<'a> {
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
#[doc = "Reader of field `LODIV_PD`"]
pub type LODIV_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LODIV_PD`"]
pub struct LODIV_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_PD_W<'a> {
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
    #[doc = "Bits 4:7 - 7:4\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\] depenancy."]
    #[inline(always)]
    pub fn pd_override(&self) -> PD_OVERRIDE_R {
        PD_OVERRIDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pa_pd(&self) -> PA_PD_R {
        PA_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn vco_pd(&self) -> VCO_PD_R {
        VCO_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] LO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lodiv_pd(&self) -> LODIV_PD_R {
        LODIV_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\] depenancy."]
    #[inline(always)]
    pub fn pd_override(&mut self) -> PD_OVERRIDE_W {
        PD_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pa_pd(&mut self) -> PA_PD_W {
        PA_PD_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn vco_pd(&mut self) -> VCO_PD_W {
        VCO_PD_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] LO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lodiv_pd(&mut self) -> LODIV_PD_W {
        LODIV_PD_W { w: self }
    }
}
