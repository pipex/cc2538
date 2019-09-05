#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
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
#[doc = "Reader of field `PLLLOCKED`"]
pub type PLLLOCKED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLLOCKED`"]
pub struct PLLLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLOCKED_W<'a> {
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
#[doc = "Reader of field `Reserved7`"]
pub type RESERVED7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved7`"]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PLLEN`"]
pub type PLLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLEN`"]
pub struct PLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLEN_W<'a> {
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
#[doc = "Reader of field `USBEN`"]
pub type USBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBEN`"]
pub struct USBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEN_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\] PLL lock status. The PLL is locked when USB_CTRL.PLLLOCKED is 1."]
    #[inline(always)]
    pub fn plllocked(&self) -> PLLLOCKED_R {
        PLLLOCKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - 6:3\\] Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - 2:2\\] Reserved."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] USB enable The USB controller is reset when this bit is cleared"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] PLL lock status. The PLL is locked when USB_CTRL.PLLLOCKED is 1."]
    #[inline(always)]
    pub fn plllocked(&mut self) -> PLLLOCKED_W {
        PLLLOCKED_W { w: self }
    }
    #[doc = "Bits 3:6 - 6:3\\] Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Reserved."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 48 MHz USB PLL enable When this bit is set, the 48 MHz PLL is started. Software must avoid access to other USB registers before the PLL has locked; that is, USB_CTRL.PLLLOCKED is 1. This bit can be set only when USB_CTRL.USBEN is 1. The PLL must be disabled before entering PM1 when suspended, and must be re-enabled when resuming operation."]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] USB enable The USB controller is reset when this bit is cleared"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W {
        USBEN_W { w: self }
    }
}
