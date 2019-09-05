#[doc = "Reader of register MDMTEST1"]
pub type R = crate::R<u32, super::MDMTEST1>;
#[doc = "Writer for register MDMTEST1"]
pub type W = crate::W<u32, super::MDMTEST1>;
#[doc = "Register MDMTEST1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMTEST1 {
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `USEMIRROR_IF`"]
pub type USEMIRROR_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USEMIRROR_IF`"]
pub struct USEMIRROR_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> USEMIRROR_IF_W<'a> {
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
#[doc = "Reader of field `MOD_IF`"]
pub type MOD_IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MOD_IF`"]
pub struct MOD_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_IF_W<'a> {
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
#[doc = "Reader of field `RAMP_AMP`"]
pub type RAMP_AMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMP_AMP`"]
pub struct RAMP_AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMP_AMP_W<'a> {
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
#[doc = "Reader of field `RFC_SNIFF_EN`"]
pub type RFC_SNIFF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC_SNIFF_EN`"]
pub struct RFC_SNIFF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_SNIFF_EN_W<'a> {
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
#[doc = "Reader of field `LOOPBACK_EN`"]
pub type LOOPBACK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBACK_EN`"]
pub struct LOOPBACK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_EN_W<'a> {
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
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\] 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
    #[inline(always)]
    pub fn usemirror_if(&self) -> USEMIRROR_IF_R {
        USEMIRROR_IF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
    #[inline(always)]
    pub fn mod_if(&self) -> MOD_IF_R {
        MOD_IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
    #[inline(always)]
    pub fn ramp_amp(&self) -> RAMP_AMP_R {
        RAMP_AMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
    #[inline(always)]
    pub fn rfc_sniff_en(&self) -> RFC_SNIFF_EN_R {
        RFC_SNIFF_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
    #[inline(always)]
    pub fn loopback_en(&self) -> LOOPBACK_EN_R {
        LOOPBACK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Use the normal IF frequency (MDMTEST0.TX_TONE) for automatic IF compensation of channel frequency on TX. 1: Use mirror IF frequency for automatic compensation of channel frequency on TX."]
    #[inline(always)]
    pub fn usemirror_if(&mut self) -> USEMIRROR_IF_W {
        USEMIRROR_IF_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Modulation is performed at an IF set by MDMTEST0.TX_TONE. The tone set by MDMTEST0.TX_TONE is superimposed on the data. 1: Modulate a tone set by MDMTEST0.TX_TONE. A tone is transmitted with frequency set by MDMTEST0.TX_TONE."]
    #[inline(always)]
    pub fn mod_if(&mut self) -> MOD_IF_W {
        MOD_IF_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 1: Enable ramping of DAC output amplitude during startup and finish. 0: Disable ramping of DAC output amplitude."]
    #[inline(always)]
    pub fn ramp_amp(&mut self) -> RAMP_AMP_W {
        RAMP_AMP_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Packet sniffer module disabled 1: Packet sniffer module enabled. The received and transmitted data can be observed on GPIO pins."]
    #[inline(always)]
    pub fn rfc_sniff_en(&mut self) -> RFC_SNIFF_EN_W {
        RFC_SNIFF_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enables loopback of modulated data into the receiver chain 0: An STXCAL instruction calibrates for TX. Use STXON to continue to active TX. 1: An STXCAL instruction enables the loopback mode."]
    #[inline(always)]
    pub fn loopback_en(&mut self) -> LOOPBACK_EN_W {
        LOOPBACK_EN_W { w: self }
    }
}
