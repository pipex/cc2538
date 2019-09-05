#[doc = "Reader of register USBCTRL"]
pub type R = crate::R<u32, super::USBCTRL>;
#[doc = "Writer for register USBCTRL"]
pub type W = crate::W<u32, super::USBCTRL>;
#[doc = "Register USBCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USB_STB`"]
pub type USB_STB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_STB`"]
pub struct USB_STB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_STB_W<'a> {
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
    #[doc = "Bit 0 - 0:0\\] USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
    #[inline(always)]
    pub fn usb_stb(&self) -> USB_STB_R {
        USB_STB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\] USB PHY stand-by override bit When this bit is cleared to 0 (default state) the USB module cannot change the stand-by mode of the PHY (USB pads) and the PHY is forced out of stand-by mode. This bit must be 1 as well as the stand-by control from the USB controller, before the mode of the PHY is stand-by."]
    #[inline(always)]
    pub fn usb_stb(&mut self) -> USB_STB_W {
        USB_STB_W { w: self }
    }
}
