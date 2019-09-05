#[doc = "Reader of register DMAC_CH1_CTRL"]
pub type R = crate::R<u32, super::DMAC_CH1_CTRL>;
#[doc = "Writer for register DMAC_CH1_CTRL"]
pub type W = crate::W<u32, super::DMAC_CH1_CTRL>;
#[doc = "Register DMAC_CH1_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAC_CH1_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved30`"]
pub type RESERVED30_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved30`"]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `PRIO`"]
pub type PRIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIO`"]
pub struct PRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\] Should be written with 0s and ignored on read"]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\] Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\] Should be written with 0s and ignored on read"]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W {
        PRIO_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
