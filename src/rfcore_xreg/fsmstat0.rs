#[doc = "Reader of register FSMSTAT0"]
pub type R = crate::R<u32, super::FSMSTAT0>;
#[doc = "Writer for register FSMSTAT0"]
pub type W = crate::W<u32, super::FSMSTAT0>;
#[doc = "Register FSMSTAT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSMSTAT0 {
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
#[doc = "Reader of field `CAL_DONE`"]
pub type CAL_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAL_DONE`"]
pub struct CAL_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_DONE_W<'a> {
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
#[doc = "Reader of field `CAL_RUNNING`"]
pub type CAL_RUNNING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAL_RUNNING`"]
pub struct CAL_RUNNING_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_RUNNING_W<'a> {
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
#[doc = "Reader of field `FSM_FFCTRL_STATE`"]
pub type FSM_FFCTRL_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSM_FFCTRL_STATE`"]
pub struct FSM_FFCTRL_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_FFCTRL_STATE_W<'a> {
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
    #[doc = "Bit 7 - 7:7\\] Frequency synthesis calibration has been performed since the last time the FS was turned on."]
    #[inline(always)]
    pub fn cal_done(&self) -> CAL_DONE_R {
        CAL_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Frequency synthesis calibration status 0: Calibration is complete or not started. 1: Calibration is in progress."]
    #[inline(always)]
    pub fn cal_running(&self) -> CAL_RUNNING_R {
        CAL_RUNNING_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - 5:0\\] Gives the current state of the FIFO and frame control (FFCTRL) finite state-machine."]
    #[inline(always)]
    pub fn fsm_ffctrl_state(&self) -> FSM_FFCTRL_STATE_R {
        FSM_FFCTRL_STATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Frequency synthesis calibration has been performed since the last time the FS was turned on."]
    #[inline(always)]
    pub fn cal_done(&mut self) -> CAL_DONE_W {
        CAL_DONE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Frequency synthesis calibration status 0: Calibration is complete or not started. 1: Calibration is in progress."]
    #[inline(always)]
    pub fn cal_running(&mut self) -> CAL_RUNNING_W {
        CAL_RUNNING_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Gives the current state of the FIFO and frame control (FFCTRL) finite state-machine."]
    #[inline(always)]
    pub fn fsm_ffctrl_state(&mut self) -> FSM_FFCTRL_STATE_W {
        FSM_FFCTRL_STATE_W { w: self }
    }
}
