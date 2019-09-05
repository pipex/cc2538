#[doc = "Reader of register MTCSPCFG"]
pub type R = crate::R<u32, super::MTCSPCFG>;
#[doc = "Writer for register MTCSPCFG"]
pub type W = crate::W<u32, super::MTCSPCFG>;
#[doc = "Register MTCSPCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MTCSPCFG {
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
#[doc = "Reader of field `MACTIMER_EVENMT_CFG`"]
pub type MACTIMER_EVENMT_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MACTIMER_EVENMT_CFG`"]
pub struct MACTIMER_EVENMT_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_EVENMT_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `Reserved3`"]
pub type RESERVED3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
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
#[doc = "Reader of field `MACTIMER_EVENT1_CFG`"]
pub type MACTIMER_EVENT1_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MACTIMER_EVENT1_CFG`"]
pub struct MACTIMER_EVENT1_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_EVENT1_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
    #[doc = "Bits 4:6 - 6:4\\] Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_evenmt_cfg(&self) -> MACTIMER_EVENMT_CFG_R {
        MACTIMER_EVENMT_CFG_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - 2:0\\] Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_event1_cfg(&self) -> MACTIMER_EVENT1_CFG_R {
        MACTIMER_EVENT1_CFG_R::new((self.bits & 0x07) as u8)
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
    #[doc = "Bits 4:6 - 6:4\\] Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_evenmt_cfg(&mut self) -> MACTIMER_EVENMT_CFG_W {
        MACTIMER_EVENMT_CFG_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Reserved. Always read 0."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_event1_cfg(&mut self) -> MACTIMER_EVENT1_CFG_W {
        MACTIMER_EVENT1_CFG_W { w: self }
    }
}
