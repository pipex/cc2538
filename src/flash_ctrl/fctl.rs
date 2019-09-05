#[doc = "Reader of register FCTL"]
pub type R = crate::R<u32, super::FCTL>;
#[doc = "Writer for register FCTL"]
pub type W = crate::W<u32, super::FCTL>;
#[doc = "Register FCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FCTL {
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
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `UPPER_PAGE_ACCESS`"]
pub type UPPER_PAGE_ACCESS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPPER_PAGE_ACCESS`"]
pub struct UPPER_PAGE_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPER_PAGE_ACCESS_W<'a> {
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
#[doc = "Reader of field `SEL_INFO_PAGE`"]
pub type SEL_INFO_PAGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL_INFO_PAGE`"]
pub struct SEL_INFO_PAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_INFO_PAGE_W<'a> {
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULL`"]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
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
#[doc = "Reader of field `ABORT`"]
pub type ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABORT`"]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
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
#[doc = "Reader of field `Reserved5`"]
pub type RESERVED5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
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
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERASE`"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
    #[doc = "Bits 10:31 - 31:10\\] Unused"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 9 - 9:9\\] Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&self) -> UPPER_PAGE_ACCESS_R {
        UPPER_PAGE_ACCESS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&self) -> SEL_INFO_PAGE_R {
        SEL_INFO_PAGE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\] Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\] Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\] Unused"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Lock bit for lock bit page 0: Neither write nor erase not allowed 1: Both write and erase allowed"]
    #[inline(always)]
    pub fn upper_page_access(&mut self) -> UPPER_PAGE_ACCESS_W {
        UPPER_PAGE_ACCESS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Flash erase or write operation on APB bus must assert this when accessing the information page"]
    #[inline(always)]
    pub fn sel_info_page(&mut self) -> SEL_INFO_PAGE_W {
        SEL_INFO_PAGE_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Set when the WRITE or ERASE bit is set; that is, when the flash controller is busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Write buffer full The CPU can write to FWDATA when this bit is 0 and WRITE is 1. This bit is cleared when BUSY is cleared."]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Abort status This bit is set to 1 when a write sequence or page erase is aborted. An operation is aborted when the accessed page is locked. Cleared when a write or page erase is started. If a write operation times out (because the FWDATA register is not written fast enough), the ABORT bit is not set even if the page is locked. If a page erase and a write operation are started simultaneously, the ABORT bit reflects the status of the last write operation. For example, if the page is locked and the write times out, the ABORT bit is not set because only the write operation times out."]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Cache Mode Disabling the cache increases the power consumption and reduces performance. Prefetching improves performance at the expense of a potential increase in power consumption. Real-time mode provides predictable flash read access time, the execution time is equal to cache disabled mode, but the power consumption is lower. 00: Cache disabled 01: Cache enabled 10: Cache enabled, with prefetch 11: Real-time mode Note: The read value always represents the current cache mode. Writing a new cache mode starts a cache mode change request that does not take effect until the controller is ready. Writes to this register are ignored if there is a current cache change request in progress."]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Write bit Start a write sequence by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.ERASE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Erase bit Start an erase operation by setting this bit to 1. Cleared by hardware when the operation completes. Writes to this bit are ignored when FCTL.BUSY is 1. If FCTL.WRITE is set simultaneously with this bit, the erase operation is started first, then the write is started."]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
}
