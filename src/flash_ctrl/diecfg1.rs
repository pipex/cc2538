#[doc = "Reader of register DIECFG1"]
pub type R = crate::R<u32, super::DIECFG1>;
#[doc = "Writer for register DIECFG1"]
pub type W = crate::W<u32, super::DIECFG1>;
#[doc = "Register DIECFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DIECFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `I2C_EN`"]
pub type I2C_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_EN`"]
pub struct I2C_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `Reserved24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `UART1_EN`"]
pub type UART1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1_EN`"]
pub struct UART1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `UART0_EN`"]
pub type UART0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0_EN`"]
pub struct UART0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SSI1_EN`"]
pub type SSI1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI1_EN`"]
pub struct SSI1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SSI0_EN`"]
pub type SSI0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI0_EN`"]
pub struct SSI0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `Reserved8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `GPTM3_EN`"]
pub type GPTM3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM3_EN`"]
pub struct GPTM3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM3_EN_W<'a> {
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
#[doc = "Reader of field `GPTM2_EN`"]
pub type GPTM2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM2_EN`"]
pub struct GPTM2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM2_EN_W<'a> {
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
#[doc = "Reader of field `GPTM1_EN`"]
pub type GPTM1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM1_EN`"]
pub struct GPTM1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM1_EN_W<'a> {
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
#[doc = "Reader of field `GPTM0_EN`"]
pub type GPTM0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM0_EN`"]
pub struct GPTM0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM0_EN_W<'a> {
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
    #[doc = "Bits 25:31 - 31:25\\] Unused"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\] 1: I2C is enabled. 0: I2C is permanently disabled."]
    #[inline(always)]
    pub fn i2c_en(&self) -> I2C_EN_R {
        I2C_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 18:23 - 23:18\\] Unused"]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\] 1: UART1 is enabled. 0: UART1 is permanently disabled."]
    #[inline(always)]
    pub fn uart1_en(&self) -> UART1_EN_R {
        UART1_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\] 1: UART0 is enabled. 0: UART0 is permanently disabled."]
    #[inline(always)]
    pub fn uart0_en(&self) -> UART0_EN_R {
        UART0_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\] Unused"]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\] 1: SSI1 is enabled. 0: SSI1 is permanently disabled."]
    #[inline(always)]
    pub fn ssi1_en(&self) -> SSI1_EN_R {
        SSI1_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] 1: SSI0 is enabled. 0: SSI0 is permanently disabled."]
    #[inline(always)]
    pub fn ssi0_en(&self) -> SSI0_EN_R {
        SSI0_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\] Unused"]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - 3:3\\] 1: GPTM3 is enabled. 0: GPTM3 is permanently disabled."]
    #[inline(always)]
    pub fn gptm3_en(&self) -> GPTM3_EN_R {
        GPTM3_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] 1: GPTM2 is enabled. 0: GPTM2 is permanently disabled."]
    #[inline(always)]
    pub fn gptm2_en(&self) -> GPTM2_EN_R {
        GPTM2_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] 1: GPTM1 is enabled. 0: GPTM1 is permanently disabled."]
    #[inline(always)]
    pub fn gptm1_en(&self) -> GPTM1_EN_R {
        GPTM1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] 1: GPTM0 is enabled. 0: GPTM0 is permanently disabled."]
    #[inline(always)]
    pub fn gptm0_en(&self) -> GPTM0_EN_R {
        GPTM0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\] Unused"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] 1: I2C is enabled. 0: I2C is permanently disabled."]
    #[inline(always)]
    pub fn i2c_en(&mut self) -> I2C_EN_W {
        I2C_EN_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\] Unused"]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] 1: UART1 is enabled. 0: UART1 is permanently disabled."]
    #[inline(always)]
    pub fn uart1_en(&mut self) -> UART1_EN_W {
        UART1_EN_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] 1: UART0 is enabled. 0: UART0 is permanently disabled."]
    #[inline(always)]
    pub fn uart0_en(&mut self) -> UART0_EN_W {
        UART0_EN_W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\] Unused"]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] 1: SSI1 is enabled. 0: SSI1 is permanently disabled."]
    #[inline(always)]
    pub fn ssi1_en(&mut self) -> SSI1_EN_W {
        SSI1_EN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] 1: SSI0 is enabled. 0: SSI0 is permanently disabled."]
    #[inline(always)]
    pub fn ssi0_en(&mut self) -> SSI0_EN_W {
        SSI0_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Unused"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 1: GPTM3 is enabled. 0: GPTM3 is permanently disabled."]
    #[inline(always)]
    pub fn gptm3_en(&mut self) -> GPTM3_EN_W {
        GPTM3_EN_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 1: GPTM2 is enabled. 0: GPTM2 is permanently disabled."]
    #[inline(always)]
    pub fn gptm2_en(&mut self) -> GPTM2_EN_W {
        GPTM2_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 1: GPTM1 is enabled. 0: GPTM1 is permanently disabled."]
    #[inline(always)]
    pub fn gptm1_en(&mut self) -> GPTM1_EN_W {
        GPTM1_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 1: GPTM0 is enabled. 0: GPTM0 is permanently disabled."]
    #[inline(always)]
    pub fn gptm0_en(&mut self) -> GPTM0_EN_W {
        GPTM0_EN_W { w: self }
    }
}
