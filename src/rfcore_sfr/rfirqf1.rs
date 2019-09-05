#[doc = "Reader of register RFIRQF1"]
pub type R = crate::R<u32, super::RFIRQF1>;
#[doc = "Writer for register RFIRQF1"]
pub type W = crate::W<u32, super::RFIRQF1>;
#[doc = "Register RFIRQF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RFIRQF1 {
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
#[doc = "Reader of field `CSP_WAIT`"]
pub type CSP_WAIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSP_WAIT`"]
pub struct CSP_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSP_WAIT_W<'a> {
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
#[doc = "Reader of field `CSP_STOP`"]
pub type CSP_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSP_STOP`"]
pub struct CSP_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSP_STOP_W<'a> {
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
#[doc = "Reader of field `CSP_MANINT`"]
pub type CSP_MANINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSP_MANINT`"]
pub struct CSP_MANINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSP_MANINT_W<'a> {
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
#[doc = "Reader of field `RFIDLE`"]
pub type RFIDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFIDLE`"]
pub struct RFIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXDONE`"]
pub type TXDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDONE`"]
pub struct TXDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDONE_W<'a> {
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
#[doc = "Reader of field `TXACKDONE`"]
pub type TXACKDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXACKDONE`"]
pub struct TXACKDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACKDONE_W<'a> {
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
    #[doc = "Bit 5 - 5:5\\] Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_wait(&self) -> CSP_WAIT_R {
        CSP_WAIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_stop(&self) -> CSP_STOP_R {
        CSP_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_manint(&self) -> CSP_MANINT_R {
        CSP_MANINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rfidle(&self) -> RFIDLE_R {
        RFIDLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txdone(&self) -> TXDONE_R {
        TXDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txackdone(&self) -> TXACKDONE_R {
        TXACKDONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_wait(&mut self) -> CSP_WAIT_W {
        CSP_WAIT_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_stop(&mut self) -> CSP_STOP_W {
        CSP_STOP_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_manint(&mut self) -> CSP_MANINT_W {
        CSP_MANINT_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rfidle(&mut self) -> RFIDLE_W {
        RFIDLE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txdone(&mut self) -> TXDONE_W {
        TXDONE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txackdone(&mut self) -> TXACKDONE_W {
        TXACKDONE_W { w: self }
    }
}
