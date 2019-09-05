#[doc = "Reader of register PTEST0"]
pub type R = crate::R<u32, super::PTEST0>;
#[doc = "Writer for register PTEST0"]
pub type W = crate::W<u32, super::PTEST0>;
#[doc = "Register PTEST0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PTEST0 {
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
#[doc = "Reader of field `PRE_PD`"]
pub type PRE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRE_PD`"]
pub struct PRE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CHP_PD`"]
pub type CHP_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHP_PD`"]
pub struct CHP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC_PD`"]
pub type ADC_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_PD`"]
pub struct ADC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PD_W<'a> {
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
#[doc = "Reader of field `DAC_PD`"]
pub type DAC_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAC_PD`"]
pub struct DAC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_PD_W<'a> {
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
#[doc = "Reader of field `LNA_PD`"]
pub type LNA_PD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNA_PD`"]
pub struct LNA_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXMIX_PD`"]
pub type TXMIX_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMIX_PD`"]
pub struct TXMIX_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIX_PD_W<'a> {
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
#[doc = "Reader of field `AAF_PD`"]
pub type AAF_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AAF_PD`"]
pub struct AAF_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AAF_PD_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Prescaler power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pre_pd(&self) -> PRE_PD_R {
        PRE_PD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Charge pump power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn chp_pd(&self) -> CHP_PD_R {
        CHP_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] ADC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn adc_pd(&self) -> ADC_PD_R {
        ADC_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] DAC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn dac_pd(&self) -> DAC_PD_R {
        DAC_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\] Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lna_pd(&self) -> LNA_PD_R {
        LNA_PD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\] Transmit mixer power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn txmix_pd(&self) -> TXMIX_PD_R {
        TXMIX_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn aaf_pd(&self) -> AAF_PD_R {
        AAF_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Prescaler power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pre_pd(&mut self) -> PRE_PD_W {
        PRE_PD_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Charge pump power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn chp_pd(&mut self) -> CHP_PD_W {
        CHP_PD_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] ADC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn adc_pd(&mut self) -> ADC_PD_W {
        ADC_PD_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] DAC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn dac_pd(&mut self) -> DAC_PD_W {
        DAC_PD_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lna_pd(&mut self) -> LNA_PD_W {
        LNA_PD_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Transmit mixer power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn txmix_pd(&mut self) -> TXMIX_PD_W {
        TXMIX_PD_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn aaf_pd(&mut self) -> AAF_PD_W {
        AAF_PD_W { w: self }
    }
}
