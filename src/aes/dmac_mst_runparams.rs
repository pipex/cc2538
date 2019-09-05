#[doc = "Reader of register DMAC_MST_RUNPARAMS"]
pub type R = crate::R<u32, super::DMAC_MST_RUNPARAMS>;
#[doc = "Writer for register DMAC_MST_RUNPARAMS"]
pub type W = crate::W<u32, super::DMAC_MST_RUNPARAMS>;
#[doc = "Register DMAC_MST_RUNPARAMS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAC_MST_RUNPARAMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AHB_MST1_BURST_SIZE`"]
pub type AHB_MST1_BURST_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHB_MST1_BURST_SIZE`"]
pub struct AHB_MST1_BURST_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_BURST_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `AHB_MST1_IDLE_EN`"]
pub type AHB_MST1_IDLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_MST1_IDLE_EN`"]
pub struct AHB_MST1_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_IDLE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `AHB_MST1_INCR_EN`"]
pub type AHB_MST1_INCR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_MST1_INCR_EN`"]
pub struct AHB_MST1_INCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_INCR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `AHB_MST1_LOCK_EN`"]
pub type AHB_MST1_LOCK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_MST1_LOCK_EN`"]
pub struct AHB_MST1_LOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_LOCK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `AHB_MST1_BIGEND`"]
pub type AHB_MST1_BIGEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_MST1_BIGEND`"]
pub struct AHB_MST1_BIGEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_BIGEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\] Should be written with 0s and ignored on read"]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\] Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&self) -> AHB_MST1_BURST_SIZE_R {
        AHB_MST1_BURST_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\] Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&self) -> AHB_MST1_IDLE_EN_R {
        AHB_MST1_IDLE_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&self) -> AHB_MST1_INCR_EN_R {
        AHB_MST1_INCR_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&self) -> AHB_MST1_LOCK_EN_R {
        AHB_MST1_LOCK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] Endianess for the AHB master 0: Little endian 1: Big endian"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&self) -> AHB_MST1_BIGEND_R {
        AHB_MST1_BIGEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\] Should be written with 0s and ignored on read"]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Should be written with 0s and ignored on read"]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Maximum burst size that can be performed on the AHB bus 0010b = 4 bytes (default) 0011b = 8 bytes 0100b = 16 bytes 0101b = 32 bytes 0110b = 64 bytes Others = Reserved"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&mut self) -> AHB_MST1_BURST_SIZE_W {
        AHB_MST1_BURST_SIZE_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Idle insertion between consecutive burst transfers on AHB 0: No Idle insertion 1: Idle insertion"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&mut self) -> AHB_MST1_IDLE_EN_W {
        AHB_MST1_IDLE_EN_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Burst length type of AHB transfer 0: Unspecified length burst transfers 1: Fixed length burst or single transfers"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&mut self) -> AHB_MST1_INCR_EN_W {
        AHB_MST1_INCR_EN_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Locked transform on AHB 0: Transfers are not locked 1: Transfers are locked"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&mut self) -> AHB_MST1_LOCK_EN_W {
        AHB_MST1_LOCK_EN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Endianess for the AHB master 0: Little endian 1: Big endian"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&mut self) -> AHB_MST1_BIGEND_W {
        AHB_MST1_BIGEND_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Should be written with 0s and ignored on read"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
}
