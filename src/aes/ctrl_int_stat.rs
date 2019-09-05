#[doc = "Reader of register CTRL_INT_STAT"]
pub type R = crate::R<u32, super::CTRL_INT_STAT>;
#[doc = "Writer for register CTRL_INT_STAT"]
pub type W = crate::W<u32, super::CTRL_INT_STAT>;
#[doc = "Register CTRL_INT_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_INT_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_BUS_ERR`"]
pub type DMA_BUS_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_BUS_ERR`"]
pub struct DMA_BUS_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BUS_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `KEY_ST_WR_ERR`"]
pub type KEY_ST_WR_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY_ST_WR_ERR`"]
pub struct KEY_ST_WR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_ST_WR_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `KEY_ST_RD_ERR`"]
pub type KEY_ST_RD_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY_ST_RD_ERR`"]
pub struct KEY_ST_RD_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_ST_RD_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `Reserved27`"]
pub type RESERVED27_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved27`"]
pub struct RESERVED27_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 2)) | (((value as u32) & 0x07ff_ffff) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMA_IN_DONE`"]
pub type DMA_IN_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_IN_DONE`"]
pub struct DMA_IN_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DONE_W<'a> {
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
#[doc = "Reader of field `RESULT_AV`"]
pub type RESULT_AV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESULT_AV`"]
pub struct RESULT_AV_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_AV_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation."]
    #[inline(always)]
    pub fn dma_bus_err(&self) -> DMA_BUS_ERR_R {
        DMA_BUS_ERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\] This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected."]
    #[inline(always)]
    pub fn key_st_wr_err(&self) -> KEY_ST_WR_ERR_R {
        KEY_ST_WR_ERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\] This bit is set when a read error is detected during the read of a key from the key store, while copying it to the AES core. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a key location is selected in the key store that is not available."]
    #[inline(always)]
    pub fn key_st_rd_err(&self) -> KEY_ST_RD_ERR_R {
        KEY_ST_RD_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 2:28 - 28:2\\] Bits should be ignored"]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 2) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\] This read only bit returns the actual DMA data in done (irq_data_in_done) interrupt status of the DMA data in done interrupt output pin (irq_data_in_done)."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] This read only bit returns the actual result available (irq_result_av) interrupt status of the result available interrupt output pin (irq_result_av)."]
    #[inline(always)]
    pub fn result_av(&self) -> RESULT_AV_R {
        RESULT_AV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\] This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation."]
    #[inline(always)]
    pub fn dma_bus_err(&mut self) -> DMA_BUS_ERR_W {
        DMA_BUS_ERR_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected."]
    #[inline(always)]
    pub fn key_st_wr_err(&mut self) -> KEY_ST_WR_ERR_W {
        KEY_ST_WR_ERR_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] This bit is set when a read error is detected during the read of a key from the key store, while copying it to the AES core. The value of this register is held until it is cleared through the CTRL_INT_CLR register. Note: This error is asserted if a key location is selected in the key store that is not available."]
    #[inline(always)]
    pub fn key_st_rd_err(&mut self) -> KEY_ST_RD_ERR_W {
        KEY_ST_RD_ERR_W { w: self }
    }
    #[doc = "Bits 2:28 - 28:2\\] Bits should be ignored"]
    #[inline(always)]
    pub fn reserved27(&mut self) -> RESERVED27_W {
        RESERVED27_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] This read only bit returns the actual DMA data in done (irq_data_in_done) interrupt status of the DMA data in done interrupt output pin (irq_data_in_done)."]
    #[inline(always)]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W {
        DMA_IN_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] This read only bit returns the actual result available (irq_result_av) interrupt status of the result available interrupt output pin (irq_result_av)."]
    #[inline(always)]
    pub fn result_av(&mut self) -> RESULT_AV_W {
        RESULT_AV_W { w: self }
    }
}
