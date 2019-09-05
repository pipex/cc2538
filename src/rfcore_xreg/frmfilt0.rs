#[doc = "Reader of register FRMFILT0"]
pub type R = crate::R<u32, super::FRMFILT0>;
#[doc = "Writer for register FRMFILT0"]
pub type W = crate::W<u32, super::FRMFILT0>;
#[doc = "Register FRMFILT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FRMFILT0 {
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
#[doc = "Reader of field `FCF_RESERVED_MASK`"]
pub type FCF_RESERVED_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCF_RESERVED_MASK`"]
pub struct FCF_RESERVED_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FCF_RESERVED_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MAX_FRAME_VERSION`"]
pub type MAX_FRAME_VERSION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_FRAME_VERSION`"]
pub struct MAX_FRAME_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_FRAME_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PAN_COORDINATOR`"]
pub type PAN_COORDINATOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAN_COORDINATOR`"]
pub struct PAN_COORDINATOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAN_COORDINATOR_W<'a> {
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
#[doc = "Reader of field `FRAME_FILTER_EN`"]
pub type FRAME_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAME_FILTER_EN`"]
pub struct FRAME_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_FILTER_EN_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\] Used for filtering on the reserved part of the frame control field (FCF) FCF_RESERVED_MASK\\[2:0\\] is ANDed with FCF\\[9:7\\]. If the result is nonzero and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    pub fn fcf_reserved_mask(&self) -> FCF_RESERVED_MASK_R {
        FCF_RESERVED_MASK_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\] Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\] (the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\] and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    pub fn max_frame_version(&self) -> MAX_FRAME_VERSION_R {
        MAX_FRAME_VERSION_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\] Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
    #[inline(always)]
    pub fn pan_coordinator(&self) -> PAN_COORDINATOR_R {
        PAN_COORDINATOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\] and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\] and SRCMATCH\\[2:0\\] are don't care.) 1: Frame filtering on."]
    #[inline(always)]
    pub fn frame_filter_en(&self) -> FRAME_FILTER_EN_R {
        FRAME_FILTER_EN_R::new((self.bits & 0x01) != 0)
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
    #[doc = "Bits 4:6 - 6:4\\] Used for filtering on the reserved part of the frame control field (FCF) FCF_RESERVED_MASK\\[2:0\\] is ANDed with FCF\\[9:7\\]. If the result is nonzero and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    pub fn fcf_reserved_mask(&mut self) -> FCF_RESERVED_MASK_W {
        FCF_RESERVED_MASK_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\] (the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\] and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    pub fn max_frame_version(&mut self) -> MAX_FRAME_VERSION_W {
        MAX_FRAME_VERSION_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
    #[inline(always)]
    pub fn pan_coordinator(&mut self) -> PAN_COORDINATOR_W {
        PAN_COORDINATOR_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\] and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\] and SRCMATCH\\[2:0\\] are don't care.) 1: Frame filtering on."]
    #[inline(always)]
    pub fn frame_filter_en(&mut self) -> FRAME_FILTER_EN_W {
        FRAME_FILTER_EN_W { w: self }
    }
}
