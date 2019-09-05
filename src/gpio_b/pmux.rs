#[doc = "Reader of register PMUX"]
pub type R = crate::R<u32, super::PMUX>;
#[doc = "Writer for register PMUX"]
pub type W = crate::W<u32, super::PMUX>;
#[doc = "Register PMUX `reset()`'s with value 0"]
impl crate::ResetValue for super::PMUX {
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
#[doc = "Reader of field `CKOEN`"]
pub type CKOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKOEN`"]
pub struct CKOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `CKOPIN`"]
pub type CKOPIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKOPIN`"]
pub struct CKOPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOPIN_W<'a> {
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
#[doc = "Reader of field `DCEN`"]
pub type DCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCEN`"]
pub struct DCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCEN_W<'a> {
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
#[doc = "Reader of field `Reserved3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `DCPIN`"]
pub type DCPIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCPIN`"]
pub struct DCPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCPIN_W<'a> {
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
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\] Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\] or PB\\[7\\] pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn ckoen(&self) -> CKOEN_R {
        CKOEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\] These are spare registers that are unused in the design."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - 4:4\\] Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\] becomes the 32-kHz clock output. When 1, PB\\[7\\] becomes the 32-kHz clock output."]
    #[inline(always)]
    pub fn ckopin(&self) -> CKOPIN_R {
        CKOPIN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\] or PB\\[0\\] pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\] These are spare registers that are unused in the design."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] Decouple control pin select This control has relevance only when DCEN is set. When 0, PB\\[1\\] becomes the on-die digital regulator status (1 indicates the on-die digital regulator is active); when 1, PB\\[0\\] becomes the on-die digital regulator status. NOTE: PB\\[1\\] and PB\\[0\\] can also be controlled with other override features. In priority order for PB\\[1\\]: When POR/BOD test mode is active, PB\\[1\\] becomes the active low brown-out detected indicator. When DCEN is set and DCPIN is not set, PB\\[1\\] becomes the on-dir digital regulator status. In priority order for PB\\[0\\]: When POR/BOD test mode is active, PB\\[0\\] becomes the power-on-reset indicator. When DCEN and DCPIN are set, PB\\[0\\] becomes the on-die digital regulator status."]
    #[inline(always)]
    pub fn dcpin(&self) -> DCPIN_R {
        DCPIN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Clock out enable When this bit is set, the 32-kHz clock is routed to either PA\\[0\\] or PB\\[7\\] pins. PMUX.CKOPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn ckoen(&mut self) -> CKOEN_W {
        CKOEN_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\] These are spare registers that are unused in the design."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Decouple control pin select This control only has relevance when CKOEN is set. When 0, PA\\[0\\] becomes the 32-kHz clock output. When 1, PB\\[7\\] becomes the 32-kHz clock output."]
    #[inline(always)]
    pub fn ckopin(&mut self) -> CKOPIN_W {
        CKOPIN_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Decouple control enable When this bit is set, the on-die digital regulator status is routed to either PB\\[1\\] or PB\\[0\\] pins. PMUX.DCPIN selects the pin to use. This overrides the current configuration setting for this pin. The pullup or pulldown is disabled and the direction is set to output for this pin."]
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W {
        DCEN_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] These are spare registers that are unused in the design."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Decouple control pin select This control has relevance only when DCEN is set. When 0, PB\\[1\\] becomes the on-die digital regulator status (1 indicates the on-die digital regulator is active); when 1, PB\\[0\\] becomes the on-die digital regulator status. NOTE: PB\\[1\\] and PB\\[0\\] can also be controlled with other override features. In priority order for PB\\[1\\]: When POR/BOD test mode is active, PB\\[1\\] becomes the active low brown-out detected indicator. When DCEN is set and DCPIN is not set, PB\\[1\\] becomes the on-dir digital regulator status. In priority order for PB\\[0\\]: When POR/BOD test mode is active, PB\\[0\\] becomes the power-on-reset indicator. When DCEN and DCPIN are set, PB\\[0\\] becomes the on-die digital regulator status."]
    #[inline(always)]
    pub fn dcpin(&mut self) -> DCPIN_W {
        DCPIN_W { w: self }
    }
}
