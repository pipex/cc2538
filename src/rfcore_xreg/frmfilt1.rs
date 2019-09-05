#[doc = "Reader of register FRMFILT1"]
pub type R = crate::R<u32, super::FRMFILT1>;
#[doc = "Writer for register FRMFILT1"]
pub type W = crate::W<u32, super::FRMFILT1>;
#[doc = "Register FRMFILT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FRMFILT1 {
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
#[doc = "Reader of field `ACCEPT_FT_4TO7_RESERVED`"]
pub type ACCEPT_FT_4TO7_RESERVED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCEPT_FT_4TO7_RESERVED`"]
pub struct ACCEPT_FT_4TO7_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_4TO7_RESERVED_W<'a> {
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
#[doc = "Reader of field `ACCEPT_FT_3_MAC_CMD`"]
pub type ACCEPT_FT_3_MAC_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCEPT_FT_3_MAC_CMD`"]
pub struct ACCEPT_FT_3_MAC_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_3_MAC_CMD_W<'a> {
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
#[doc = "Reader of field `ACCEPT_FT_2_ACK`"]
pub type ACCEPT_FT_2_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCEPT_FT_2_ACK`"]
pub struct ACCEPT_FT_2_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_2_ACK_W<'a> {
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
#[doc = "Reader of field `ACCEPT_FT_1_DATA`"]
pub type ACCEPT_FT_1_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCEPT_FT_1_DATA`"]
pub struct ACCEPT_FT_1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_1_DATA_W<'a> {
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
#[doc = "Reader of field `ACCEPT_FT_0_BEACON`"]
pub type ACCEPT_FT_0_BEACON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCEPT_FT_0_BEACON`"]
pub struct ACCEPT_FT_0_BEACON_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEPT_FT_0_BEACON_W<'a> {
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
#[doc = "Reader of field `MODIFY_FT_FILTER`"]
pub type MODIFY_FT_FILTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODIFY_FT_FILTER`"]
pub struct MODIFY_FT_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MODIFY_FT_FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `FRM_RESERVED`"]
pub type FRM_RESERVED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRM_RESERVED`"]
pub struct FRM_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_RESERVED_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Defines whether reserved frames are accepted or not. Reserved frames have frame type = 100, 101, 110, or 111. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_4to7_reserved(&self) -> ACCEPT_FT_4TO7_RESERVED_R {
        ACCEPT_FT_4TO7_RESERVED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_3_mac_cmd(&self) -> ACCEPT_FT_3_MAC_CMD_R {
        ACCEPT_FT_3_MAC_CMD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_2_ack(&self) -> ACCEPT_FT_2_ACK_R {
        ACCEPT_FT_2_ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_1_data(&self) -> ACCEPT_FT_1_DATA_R {
        ACCEPT_FT_1_DATA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_0_beacon(&self) -> ACCEPT_FT_0_BEACON_R {
        ACCEPT_FT_0_BEACON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\] These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
    #[inline(always)]
    pub fn modify_ft_filter(&self) -> MODIFY_FT_FILTER_R {
        MODIFY_FT_FILTER_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] Reserved. Always write 0."]
    #[inline(always)]
    pub fn frm_reserved(&self) -> FRM_RESERVED_R {
        FRM_RESERVED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Defines whether reserved frames are accepted or not. Reserved frames have frame type = 100, 101, 110, or 111. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_4to7_reserved(&mut self) -> ACCEPT_FT_4TO7_RESERVED_W {
        ACCEPT_FT_4TO7_RESERVED_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_3_mac_cmd(&mut self) -> ACCEPT_FT_3_MAC_CMD_W {
        ACCEPT_FT_3_MAC_CMD_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_2_ack(&mut self) -> ACCEPT_FT_2_ACK_W {
        ACCEPT_FT_2_ACK_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_1_data(&mut self) -> ACCEPT_FT_1_DATA_W {
        ACCEPT_FT_1_DATA_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_0_beacon(&mut self) -> ACCEPT_FT_0_BEACON_W {
        ACCEPT_FT_0_BEACON_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
    #[inline(always)]
    pub fn modify_ft_filter(&mut self) -> MODIFY_FT_FILTER_W {
        MODIFY_FT_FILTER_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Reserved. Always write 0."]
    #[inline(always)]
    pub fn frm_reserved(&mut self) -> FRM_RESERVED_W {
        FRM_RESERVED_W { w: self }
    }
}
