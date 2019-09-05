#[doc = "Reader of register FRMCTRL0"]
pub type R = crate::R<u32, super::FRMCTRL0>;
#[doc = "Writer for register FRMCTRL0"]
pub type W = crate::W<u32, super::FRMCTRL0>;
#[doc = "Register FRMCTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FRMCTRL0 {
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
#[doc = "Reader of field `APPEND_DATA_MODE`"]
pub type APPEND_DATA_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APPEND_DATA_MODE`"]
pub struct APPEND_DATA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> APPEND_DATA_MODE_W<'a> {
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
#[doc = "Reader of field `AUTOCRC`"]
pub type AUTOCRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCRC`"]
pub struct AUTOCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCRC_W<'a> {
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
#[doc = "Reader of field `AUTOACK`"]
pub type AUTOACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOACK`"]
pub struct AUTOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOACK_W<'a> {
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
#[doc = "Reader of field `ENERGY_SCAN`"]
pub type ENERGY_SCAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENERGY_SCAN`"]
pub struct ENERGY_SCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENERGY_SCAN_W<'a> {
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
#[doc = "Reader of field `RX_MODE`"]
pub type RX_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_MODE`"]
pub struct RX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TX_MODE`"]
pub type TX_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_MODE`"]
pub struct TX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MODE_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
    #[inline(always)]
    pub fn append_data_mode(&self) -> APPEND_DATA_MODE_R {
        APPEND_DATA_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
    #[inline(always)]
    pub fn autocrc(&self) -> AUTOCRC_R {
        AUTOCRC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
    #[inline(always)]
    pub fn autoack(&self) -> AUTOACK_R {
        AUTOACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
    #[inline(always)]
    pub fn energy_scan(&self) -> ENERGY_SCAN_R {
        ENERGY_SCAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\] Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
    #[inline(always)]
    pub fn rx_mode(&self) -> RX_MODE_R {
        RX_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
    #[inline(always)]
    pub fn tx_mode(&self) -> TX_MODE_R {
        TX_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
    #[inline(always)]
    pub fn append_data_mode(&mut self) -> APPEND_DATA_MODE_W {
        APPEND_DATA_MODE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
    #[inline(always)]
    pub fn autocrc(&mut self) -> AUTOCRC_W {
        AUTOCRC_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
    #[inline(always)]
    pub fn autoack(&mut self) -> AUTOACK_W {
        AUTOACK_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
    #[inline(always)]
    pub fn energy_scan(&mut self) -> ENERGY_SCAN_W {
        ENERGY_SCAN_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
    #[inline(always)]
    pub fn rx_mode(&mut self) -> RX_MODE_W {
        RX_MODE_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TX_MODE_W {
        TX_MODE_W { w: self }
    }
}
