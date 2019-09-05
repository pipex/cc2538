#[doc = "Reader of register MDMTEST0"]
pub type R = crate::R<u32, super::MDMTEST0>;
#[doc = "Writer for register MDMTEST0"]
pub type W = crate::W<u32, super::MDMTEST0>;
#[doc = "Register MDMTEST0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMTEST0 {
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
#[doc = "Reader of field `TX_TONE`"]
pub type TX_TONE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_TONE`"]
pub struct TX_TONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DC_WIN_SIZE`"]
pub type DC_WIN_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DC_WIN_SIZE`"]
pub struct DC_WIN_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_WIN_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DC_BLOCK_MODE`"]
pub type DC_BLOCK_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DC_BLOCK_MODE`"]
pub struct DC_BLOCK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_BLOCK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 4:7 - 7:4\\] Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
    #[inline(always)]
    pub fn tx_tone(&self) -> TX_TONE_R {
        TX_TONE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\] Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
    #[inline(always)]
    pub fn dc_win_size(&self) -> DC_WIN_SIZE_R {
        DC_WIN_SIZE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
    #[inline(always)]
    pub fn dc_block_mode(&self) -> DC_BLOCK_MODE_R {
        DC_BLOCK_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Enables the possibility to transmit a baseband tone by picking samples from the sine tables with a controllable phase step between the samples. The step size is controlled by TX_TONE. If MDMTEST1.MOD_IF is 0, the tone is superpositioned on the modulated data, effectively giving modulation with an IF. If MDMTEST1.MOD_IF is 1, only the tone is transmitted. 0000: -6 MHz 0001: -4 MHz 0010: -3 MHz 0011: -2 MHz 0100: -1 MHz 0101: -500 kHz 0110: -4 kHz 0111: 0 1000: 4 kHz 1001: 500 kHz 1010: 1 MHz 1011: 2 MHz 1100: 3 MHz 1101: 4 MHz 1110: 6 MHz Others: Reserved"]
    #[inline(always)]
    pub fn tx_tone(&mut self) -> TX_TONE_W {
        TX_TONE_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Controls the numbers of samples to be accumulated between each dump of the accumulate-and-dump filter used in DC removal 00: 32 samples 01: 64 samples 10: 128 samples 11: 256 samples"]
    #[inline(always)]
    pub fn dc_win_size(&mut self) -> DC_WIN_SIZE_W {
        DC_WIN_SIZE_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Selects the mode of operation 00: The input signal to the DC blocker is passed to the output without any attempt to remove DC. 01: Enable DC cancellation. Normal operation 10: Freeze estimates of DC when sync is found. Resume estimating DC when searching for the next frame. 11: Reserved"]
    #[inline(always)]
    pub fn dc_block_mode(&mut self) -> DC_BLOCK_MODE_W {
        DC_BLOCK_MODE_W { w: self }
    }
}
