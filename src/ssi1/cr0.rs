#[doc = "Reader of register CR0"]
pub type R = crate::R<u32, super::CR0>;
#[doc = "Writer for register CR0"]
pub type W = crate::W<u32, super::CR0>;
#[doc = "Register CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCR`"]
pub type SCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCR`"]
pub struct SCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPH`"]
pub type SPH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPH`"]
pub struct SPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SPH_W<'a> {
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
#[doc = "Reader of field `SPO`"]
pub type SPO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPO`"]
pub struct SPO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPO_W<'a> {
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
#[doc = "Reader of field `FRF`"]
pub type FRF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRF`"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DSS`"]
pub type DSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSS`"]
pub struct DSS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\] SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - 7:7\\] SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn sph(&self) -> SPH_R {
        SPH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\] SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\] SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\] Reserved"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W {
        SCR_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn sph(&mut self) -> SPH_W {
        SPH_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn spo(&mut self) -> SPO_W {
        SPO_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
    #[inline(always)]
    pub fn dss(&mut self) -> DSS_W {
        DSS_W { w: self }
    }
}
