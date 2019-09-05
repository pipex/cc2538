#[doc = "Reader of register ADCCON3"]
pub type R = crate::R<u32, super::ADCCON3>;
#[doc = "Writer for register ADCCON3"]
pub type W = crate::W<u32, super::ADCCON3>;
#[doc = "Register ADCCON3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCON3 {
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
#[doc = "Reader of field `EREF`"]
pub type EREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EREF`"]
pub struct EREF_W<'a> {
    w: &'a mut W,
}
impl<'a> EREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `EDIV`"]
pub type EDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDIV`"]
pub struct EDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ECH`"]
pub type ECH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECH`"]
pub struct ECH_W<'a> {
    w: &'a mut W,
}
impl<'a> ECH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 6:7 - 7:6\\] Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    pub fn eref(&self) -> EREF_R {
        EREF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\] Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
    #[inline(always)]
    pub fn ediv(&self) -> EDIV_R {
        EDIV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\] Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    pub fn ech(&self) -> ECH_R {
        ECH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    pub fn eref(&mut self) -> EREF_W {
        EREF_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
    #[inline(always)]
    pub fn ediv(&mut self) -> EDIV_W {
        EDIV_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    pub fn ech(&mut self) -> ECH_W {
        ECH_W { w: self }
    }
}
