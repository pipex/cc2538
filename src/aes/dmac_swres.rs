#[doc = "Reader of register DMAC_SWRES"]
pub type R = crate::R<u32, super::DMAC_SWRES>;
#[doc = "Writer for register DMAC_SWRES"]
pub type W = crate::W<u32, super::DMAC_SWRES>;
#[doc = "Register DMAC_SWRES `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAC_SWRES {
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
#[doc = "Reader of field `SWRES`"]
pub type SWRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRES`"]
pub struct SWRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRES_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\] Bits should be written with a value of 0."]
    #[inline(always)]
    pub fn reserved31(&self) -> RESERVED31_R {
        RESERVED31_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\] Software reset enable 0 = Disabled 1 = Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMAC_STATUS register."]
    #[inline(always)]
    pub fn swres(&self) -> SWRES_R {
        SWRES_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\] Bits should be written with a value of 0."]
    #[inline(always)]
    pub fn reserved31(&mut self) -> RESERVED31_W {
        RESERVED31_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Software reset enable 0 = Disabled 1 = Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMAC_STATUS register."]
    #[inline(always)]
    pub fn swres(&mut self) -> SWRES_W {
        SWRES_W { w: self }
    }
}
