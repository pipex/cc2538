#[doc = "Reader of register OBSSEL2"]
pub type R = crate::R<u32, super::OBSSEL2>;
#[doc = "Writer for register OBSSEL2"]
pub type W = crate::W<u32, super::OBSSEL2>;
#[doc = "Register OBSSEL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::OBSSEL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 7:7\\] Observation output 2 enable control for PC2 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC2."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6 - 6:0\\] n - obs_sigs\\[n\\] output on output 2: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - 7:7\\] Observation output 2 enable control for PC2 0: Observation output disabled 1: Observation output enabled Note: If enabled, this overwrites the standard GPIO behavior of PC2."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\] n - obs_sigs\\[n\\] output on output 2: 0: rfc_obs_sig0 1: rfc_obs_sig1 2: rfc_obs_sig2 Others: Reserved"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
