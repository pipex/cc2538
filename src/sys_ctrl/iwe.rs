#[doc = "Reader of register IWE"]
pub type R = crate::R<u32, super::IWE>;
#[doc = "Writer for register IWE"]
pub type W = crate::W<u32, super::IWE>;
#[doc = "Register IWE `reset()`'s with value 0"]
impl crate::ResetValue for super::IWE {
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
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `SM_TIMER_IWE`"]
pub type SM_TIMER_IWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM_TIMER_IWE`"]
pub struct SM_TIMER_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_TIMER_IWE_W<'a> {
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
#[doc = "Reader of field `USB_IWE`"]
pub type USB_IWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_IWE`"]
pub struct USB_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_IWE_W<'a> {
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
#[doc = "Reader of field `PORT_D_IWE`"]
pub type PORT_D_IWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT_D_IWE`"]
pub struct PORT_D_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_D_IWE_W<'a> {
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
#[doc = "Reader of field `PORT_C_IWE`"]
pub type PORT_C_IWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT_C_IWE`"]
pub struct PORT_C_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_C_IWE_W<'a> {
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
#[doc = "Reader of field `PORT_B_IWE`"]
pub type PORT_B_IWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT_B_IWE`"]
pub struct PORT_B_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_B_IWE_W<'a> {
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
#[doc = "Reader of field `PORT_A_IWE`"]
pub type PORT_A_IWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT_A_IWE`"]
pub struct PORT_A_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_A_IWE_W<'a> {
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
    #[doc = "Bits 6:31 - 31:6\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bit 5 - 5:5\\] 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
    #[inline(always)]
    pub fn sm_timer_iwe(&self) -> SM_TIMER_IWE_R {
        SM_TIMER_IWE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
    #[inline(always)]
    pub fn usb_iwe(&self) -> USB_IWE_R {
        USB_IWE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
    #[inline(always)]
    pub fn port_d_iwe(&self) -> PORT_D_IWE_R {
        PORT_D_IWE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
    #[inline(always)]
    pub fn port_c_iwe(&self) -> PORT_C_IWE_R {
        PORT_C_IWE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
    #[inline(always)]
    pub fn port_b_iwe(&self) -> PORT_B_IWE_R {
        PORT_B_IWE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
    #[inline(always)]
    pub fn port_a_iwe(&self) -> PORT_A_IWE_R {
        PORT_A_IWE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
    #[inline(always)]
    pub fn sm_timer_iwe(&mut self) -> SM_TIMER_IWE_W {
        SM_TIMER_IWE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
    #[inline(always)]
    pub fn usb_iwe(&mut self) -> USB_IWE_W {
        USB_IWE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
    #[inline(always)]
    pub fn port_d_iwe(&mut self) -> PORT_D_IWE_W {
        PORT_D_IWE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
    #[inline(always)]
    pub fn port_c_iwe(&mut self) -> PORT_C_IWE_W {
        PORT_C_IWE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
    #[inline(always)]
    pub fn port_b_iwe(&mut self) -> PORT_B_IWE_W {
        PORT_B_IWE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
    #[inline(always)]
    pub fn port_a_iwe(&mut self) -> PORT_A_IWE_W {
        PORT_A_IWE_W { w: self }
    }
}
