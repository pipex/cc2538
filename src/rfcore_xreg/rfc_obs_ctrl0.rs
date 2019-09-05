#[doc = "Reader of register RFC_OBS_CTRL0"]
pub type R = crate::R<u32, super::RFC_OBS_CTRL0>;
#[doc = "Writer for register RFC_OBS_CTRL0"]
pub type W = crate::W<u32, super::RFC_OBS_CTRL0>;
#[doc = "Register RFC_OBS_CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RFC_OBS_CTRL0 {
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
pub type RESERVED8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
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
#[doc = "Reader of field `RFC_OBS_POL0`"]
pub type RFC_OBS_POL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC_OBS_POL0`"]
pub struct RFC_OBS_POL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_OBS_POL0_W<'a> {
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
#[doc = "Reader of field `RFC_OBS_MUX0`"]
pub type RFC_OBS_MUX0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFC_OBS_MUX0`"]
pub struct RFC_OBS_MUX0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_OBS_MUX0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] The signal chosen by RFC_OBS_MUX0 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol0(&self) -> RFC_OBS_POL0_R {
        RFC_OBS_POL0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - 5:0\\] Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[0\\]. 00 0000: 0 - Constant value 00 0001: 1 - Constant value 00 1000: rfc_sniff_data - Data from packet sniffer. Sample data on rising edges of sniff_clk. 00 1001: rfc_sniff_clk - 250kHz clock for packet sniffer data. 00 1100: rssi_valid - Pin is high when the RSSI value has been updated at least once since RX was started. Cleared when leaving RX. 00 1101: demod_cca - Clear channel assessment. See FSMSTAT1 register for details on how to configure the behavior of this signal. 00 1110: sampled_cca - A sampled version of the CCA bit from demodulator. The value is updated whenever a SSAMPLECCA or STXONCCA strobe is issued. 00 1111: sfd_sync - Pin is high when a SFD has been received or transmitted. Cleared when leaving RX/TX respectively. Not to be confused with the SFD exception. 01 0000: tx_active - Indicates that FFCTRL is in one of the TX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0001: rx_active - Indicates that FFCTRL is in one of the RX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0010: ffctrl_fifo - Pin is high when one or more bytes are in the RXFIFO. Low during RXFIFO overflow. 01 0011: ffctrl_fifop - Pin is high when the number of bytes in the RXFIFO exceeds the programmable threshold or at least one complete frame is in the RXFIFO. Also high during RXFIFO overflow. Not to be confused with the FIFOP exception. 01 0100: packet_done - A complete frame has been received. I.e., the number of bytes set by the frame-length field has been received. 01 0110: rfc_xor_rand_i_q - XOR between I and Q random outputs. Updated at 8 MHz. 01 0111: rfc_rand_q - Random data output from the Q channel of the receiver. Updated at 8 MHz. 01 1000: rfc_rand_i - Random data output from the I channel of the receiver. Updated at 8 MHz 01 1001: lock_status - 1 when PLL is in lock, otherwise 0 10 1000: pa_pd - Power amplifier power-down signal 10 1010: lna_pd - LNA power-down signal Others: Reserved"]
    #[inline(always)]
    pub fn rfc_obs_mux0(&self) -> RFC_OBS_MUX0_R {
        RFC_OBS_MUX0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] The signal chosen by RFC_OBS_MUX0 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol0(&mut self) -> RFC_OBS_POL0_W {
        RFC_OBS_POL0_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[0\\]. 00 0000: 0 - Constant value 00 0001: 1 - Constant value 00 1000: rfc_sniff_data - Data from packet sniffer. Sample data on rising edges of sniff_clk. 00 1001: rfc_sniff_clk - 250kHz clock for packet sniffer data. 00 1100: rssi_valid - Pin is high when the RSSI value has been updated at least once since RX was started. Cleared when leaving RX. 00 1101: demod_cca - Clear channel assessment. See FSMSTAT1 register for details on how to configure the behavior of this signal. 00 1110: sampled_cca - A sampled version of the CCA bit from demodulator. The value is updated whenever a SSAMPLECCA or STXONCCA strobe is issued. 00 1111: sfd_sync - Pin is high when a SFD has been received or transmitted. Cleared when leaving RX/TX respectively. Not to be confused with the SFD exception. 01 0000: tx_active - Indicates that FFCTRL is in one of the TX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0001: rx_active - Indicates that FFCTRL is in one of the RX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0010: ffctrl_fifo - Pin is high when one or more bytes are in the RXFIFO. Low during RXFIFO overflow. 01 0011: ffctrl_fifop - Pin is high when the number of bytes in the RXFIFO exceeds the programmable threshold or at least one complete frame is in the RXFIFO. Also high during RXFIFO overflow. Not to be confused with the FIFOP exception. 01 0100: packet_done - A complete frame has been received. I.e., the number of bytes set by the frame-length field has been received. 01 0110: rfc_xor_rand_i_q - XOR between I and Q random outputs. Updated at 8 MHz. 01 0111: rfc_rand_q - Random data output from the Q channel of the receiver. Updated at 8 MHz. 01 1000: rfc_rand_i - Random data output from the I channel of the receiver. Updated at 8 MHz 01 1001: lock_status - 1 when PLL is in lock, otherwise 0 10 1000: pa_pd - Power amplifier power-down signal 10 1010: lna_pd - LNA power-down signal Others: Reserved"]
    #[inline(always)]
    pub fn rfc_obs_mux0(&mut self) -> RFC_OBS_MUX0_W {
        RFC_OBS_MUX0_W { w: self }
    }
}
