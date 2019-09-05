#[doc = "Reader of register DMAC_STATUS"]
pub type R = crate::R<u32, super::DMAC_STATUS>;
#[doc = "Writer for register DMAC_STATUS"]
pub type W = crate::W<u32, super::DMAC_STATUS>;
#[doc = "Register DMAC_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAC_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved14`"]
pub type RESERVED14_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved14`"]
pub struct RESERVED14_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 18)) | (((value as u32) & 0x3fff) << 18);
        self.w
    }
}
#[doc = "Reader of field `PORT_ERR`"]
pub type PORT_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT_ERR`"]
pub struct PORT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_ERR_W<'a> {
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
#[doc = "Reader of field `Reserved15`"]
pub type RESERVED15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved15`"]
pub struct RESERVED15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 2)) | (((value as u32) & 0x7fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `CH1_ACT`"]
pub type CH1_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_ACT`"]
pub struct CH1_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_ACT_W<'a> {
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
#[doc = "Reader of field `CH0_ACT`"]
pub type CH0_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_ACT`"]
pub struct CH0_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_ACT_W<'a> {
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
    #[doc = "Bits 18:31 - 31:18\\] Bits should be ignored on read."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\] Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub fn port_err(&self) -> PORT_ERR_R {
        PORT_ERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 2:16 - 16:2\\] Bits should be ignored on read."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bit 1 - 1:1\\] A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch1_act(&self) -> CH1_ACT_R {
        CH1_ACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch0_act(&self) -> CH0_ACT_R {
        CH0_ACT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 18:31 - 31:18\\] Bits should be ignored on read."]
    #[inline(always)]
    pub fn reserved14(&mut self) -> RESERVED14_W {
        RESERVED14_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub fn port_err(&mut self) -> PORT_ERR_W {
        PORT_ERR_W { w: self }
    }
    #[doc = "Bits 2:16 - 16:2\\] Bits should be ignored on read."]
    #[inline(always)]
    pub fn reserved15(&mut self) -> RESERVED15_W {
        RESERVED15_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] A value of 1 indicates that channel 1 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch1_act(&mut self) -> CH1_ACT_W {
        CH1_ACT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] A value of 1 indicates that channel 0 is active (DMA transfer on-going)."]
    #[inline(always)]
    pub fn ch0_act(&mut self) -> CH0_ACT_W {
        CH0_ACT_W { w: self }
    }
}
