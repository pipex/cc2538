#[doc = "Reader of register RFERRF"]
pub type R = crate::R<u32, super::RFERRF>;
#[doc = "Writer for register RFERRF"]
pub type W = crate::W<u32, super::RFERRF>;
#[doc = "Register RFERRF `reset()`'s with value 0"]
impl crate::ResetValue for super::RFERRF {
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
#[doc = "Reader of field `STROBEERR`"]
pub type STROBEERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBEERR`"]
pub struct STROBEERR_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBEERR_W<'a> {
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
#[doc = "Reader of field `TXUNDERF`"]
pub type TXUNDERF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUNDERF`"]
pub struct TXUNDERF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERF_W<'a> {
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
#[doc = "Reader of field `TXOVERF`"]
pub type TXOVERF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOVERF`"]
pub struct TXOVERF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVERF_W<'a> {
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
#[doc = "Reader of field `RXUNDERF`"]
pub type RXUNDERF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUNDERF`"]
pub struct RXUNDERF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUNDERF_W<'a> {
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
#[doc = "Reader of field `RXOVERF`"]
pub type RXOVERF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVERF`"]
pub struct RXOVERF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERF_W<'a> {
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
#[doc = "Reader of field `RXABO`"]
pub type RXABO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXABO`"]
pub struct RXABO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXABO_W<'a> {
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
#[doc = "Reader of field `NLOCK`"]
pub type NLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NLOCK`"]
pub struct NLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> NLOCK_W<'a> {
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
    #[doc = "Bit 6 - 6:6\\] A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn strobeerr(&self) -> STROBEERR_R {
        STROBEERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txunderf(&self) -> TXUNDERF_R {
        TXUNDERF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txoverf(&self) -> TXOVERF_R {
        TXOVERF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxunderf(&self) -> RXUNDERF_R {
        RXUNDERF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxoverf(&self) -> RXOVERF_R {
        RXOVERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxabo(&self) -> RXABO_R {
        RXABO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn nlock(&self) -> NLOCK_R {
        NLOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn strobeerr(&mut self) -> STROBEERR_W {
        STROBEERR_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txunderf(&mut self) -> TXUNDERF_W {
        TXUNDERF_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txoverf(&mut self) -> TXOVERF_W {
        TXOVERF_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxunderf(&mut self) -> RXUNDERF_W {
        RXUNDERF_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxoverf(&mut self) -> RXOVERF_W {
        RXOVERF_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxabo(&mut self) -> RXABO_W {
        RXABO_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn nlock(&mut self) -> NLOCK_W {
        NLOCK_W { w: self }
    }
}
