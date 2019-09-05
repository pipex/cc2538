#[doc = "Reader of register DIECFG0"]
pub type R = crate::R<u32, super::DIECFG0>;
#[doc = "Writer for register DIECFG0"]
pub type W = crate::W<u32, super::DIECFG0>;
#[doc = "Register DIECFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIECFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHIPID`"]
pub type CHIPID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHIPID`"]
pub struct CHIPID_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIPID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `Reserved16`"]
pub type RESERVED16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Reader of field `CLK_SEL_GATE_EN_N`"]
pub type CLK_SEL_GATE_EN_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_SEL_GATE_EN_N`"]
pub struct CLK_SEL_GATE_EN_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL_GATE_EN_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SRAM_SIZE`"]
pub type SRAM_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRAM_SIZE`"]
pub struct SRAM_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `FLASH_SIZE`"]
pub type FLASH_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_SIZE`"]
pub struct FLASH_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `USB_ENABLE`"]
pub type USB_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB_ENABLE`"]
pub struct USB_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ENABLE_W<'a> {
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
#[doc = "Reader of field `MASS_ERASE_ENABLE`"]
pub type MASS_ERASE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASS_ERASE_ENABLE`"]
pub struct MASS_ERASE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASS_ERASE_ENABLE_W<'a> {
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
#[doc = "Reader of field `LOCK_FWT_N`"]
pub type LOCK_FWT_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_FWT_N`"]
pub struct LOCK_FWT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_FWT_N_W<'a> {
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
#[doc = "Reader of field `LOCK_IP_N`"]
pub type LOCK_IP_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_IP_N`"]
pub struct LOCK_IP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_IP_N_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\] Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn chipid(&self) -> CHIPID_R {
        CHIPID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\] Unused"]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - 10:10\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn clk_sel_gate_en_n(&self) -> CLK_SEL_GATE_EN_N_R {
        CLK_SEL_GATE_EN_N_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - 9:7\\] Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn sram_size(&self) -> SRAM_SIZE_R {
        SRAM_SIZE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\] Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn usb_enable(&self) -> USB_ENABLE_R {
        USB_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn mass_erase_enable(&self) -> MASS_ERASE_ENABLE_R {
        MASS_ERASE_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn lock_fwt_n(&self) -> LOCK_FWT_N_R {
        LOCK_FWT_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn lock_ip_n(&self) -> LOCK_IP_N_R {
        LOCK_IP_N_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn chipid(&mut self) -> CHIPID_W {
        CHIPID_W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\] Unused"]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn clk_sel_gate_en_n(&mut self) -> CLK_SEL_GATE_EN_N_W {
        CLK_SEL_GATE_EN_N_W { w: self }
    }
    #[doc = "Bits 7:9 - 9:7\\] Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn sram_size(&mut self) -> SRAM_SIZE_W {
        SRAM_SIZE_W { w: self }
    }
    #[doc = "Bits 4:6 - 6:4\\] Register copy of configuration bits Three clock cycles after reset is released, this bit field is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn flash_size(&mut self) -> FLASH_SIZE_W {
        FLASH_SIZE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn usb_enable(&mut self) -> USB_ENABLE_W {
        USB_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn mass_erase_enable(&mut self) -> MASS_ERASE_ENABLE_W {
        MASS_ERASE_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn lock_fwt_n(&mut self) -> LOCK_FWT_N_W {
        LOCK_FWT_N_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Register copy of configuration bits Three clock cycles after reset is released, this bit is equal to the field with the same name in the information page."]
    #[inline(always)]
    pub fn lock_ip_n(&mut self) -> LOCK_IP_N_W {
        LOCK_IP_N_W { w: self }
    }
}
