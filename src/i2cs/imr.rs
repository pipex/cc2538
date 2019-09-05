#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Writer for register IMR"]
pub type W = crate::W<u32, super::IMR>;
#[doc = "Register IMR `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved29`"]
pub type RESERVED29_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved29`"]
pub struct RESERVED29_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `STOPIM`"]
pub type STOPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPIM`"]
pub struct STOPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIM_W<'a> {
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
#[doc = "Reader of field `STARTIM`"]
pub type STARTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTIM`"]
pub struct STARTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTIM_W<'a> {
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
#[doc = "Reader of field `DATAIM`"]
pub type DATAIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAIM`"]
pub struct DATAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIM_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\] Stop condition interrupt mask 1: The STOP condition interrupt is sent to the interrupt controller when the STOPRIS bit in the I2CSRIS register is set. 0: The STOPRIS interrupt is supressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn stopim(&self) -> STOPIM_R {
        STOPIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Start condition interrupt mask 1: The START condition interrupt is sent to the interrupt controller when the STARTRIS bit in the I2CSRIS register is set. 0: The STARTRIS interrupt is supressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn startim(&self) -> STARTIM_R {
        STARTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&self) -> DATAIM_R {
        DATAIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Stop condition interrupt mask 1: The STOP condition interrupt is sent to the interrupt controller when the STOPRIS bit in the I2CSRIS register is set. 0: The STOPRIS interrupt is supressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn stopim(&mut self) -> STOPIM_W {
        STOPIM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Start condition interrupt mask 1: The START condition interrupt is sent to the interrupt controller when the STARTRIS bit in the I2CSRIS register is set. 0: The STARTRIS interrupt is supressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn startim(&mut self) -> STARTIM_W {
        STARTIM_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Data interrupt mask 1: The data received or data requested interrupt is sent to the interrupt controller when the DATARIS bit in the I2CSRIS register is set. 0: The DATARIS interrupt is surpressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn dataim(&mut self) -> DATAIM_W {
        DATAIM_W { w: self }
    }
}
