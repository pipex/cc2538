#[doc = "Reader of register CTRL_INT_EN"]
pub type R = crate::R<u32, super::CTRL_INT_EN>;
#[doc = "Writer for register CTRL_INT_EN"]
pub type W = crate::W<u32, super::CTRL_INT_EN>;
#[doc = "Register CTRL_INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_INT_EN {
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
    #[doc = "Bits 2:31 - 31:2\\] Bits should be written with 0s and ignored on read."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\] If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&self) -> RESULT_AV_R {
        RESULT_AV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\] Bits should be written with 0s and ignored on read."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W {
        DMA_IN_DONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&mut self) -> RESULT_AV_W {
        RESULT_AV_W { w: self }
    }
}
