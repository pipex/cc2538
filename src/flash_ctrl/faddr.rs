#[doc = "Reader of register FADDR"]
pub type R = crate::R<u32, super::FADDR>;
#[doc = "Writer for register FADDR"]
pub type W = crate::W<u32, super::FADDR>;
#[doc = "Register FADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FADDR {
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
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Reader of field `FADDR`"]
pub type FADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FADDR`"]
pub struct FADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - 31:17\\] Unused. These bits always reflect 0 on read back"]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:16 - 16:0\\] Bit number \\[16:9\\] selects one of 256 pages for page erase. Bit number \\[8:7\\] selects one of the 4 row in a given page Bit number \\[6:1\\] selects one of the 64-bit wide locations in a give row. Bit number \\[0\\] will select upper/lower 32-bits in a given 64-bit location - 64Kbytes --> Bits \\[16:14\\] will always be 0. - 128Kbytes --> Bits \\[16:15\\] will always be 0. - 256Kbytes --> Bit \\[16\\] will always be 0. - 384/512Kbytes --> All bits written and valid. Writes to this register will be ignored when any of FCTL.WRITE and FCTL.ERASE is set. FADDR should be written with byte addressable location of the Flash to be programmed. Read back value always reflects a 32-bit aligned address. When the register is read back, the value that was written to FADDR gets right shift by 2 to indicate 32-bit aligned address. In other words lower 2 bits are discarded while reading back the register. Out of range address results in roll over. There is no status signal generated by flash controller to indicate this. Firmware is responsible to managing the addresses correctly."]
    #[inline(always)]
    pub fn faddr(&self) -> FADDR_R {
        FADDR_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\] Unused. These bits always reflect 0 on read back"]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bits 0:16 - 16:0\\] Bit number \\[16:9\\] selects one of 256 pages for page erase. Bit number \\[8:7\\] selects one of the 4 row in a given page Bit number \\[6:1\\] selects one of the 64-bit wide locations in a give row. Bit number \\[0\\] will select upper/lower 32-bits in a given 64-bit location - 64Kbytes --> Bits \\[16:14\\] will always be 0. - 128Kbytes --> Bits \\[16:15\\] will always be 0. - 256Kbytes --> Bit \\[16\\] will always be 0. - 384/512Kbytes --> All bits written and valid. Writes to this register will be ignored when any of FCTL.WRITE and FCTL.ERASE is set. FADDR should be written with byte addressable location of the Flash to be programmed. Read back value always reflects a 32-bit aligned address. When the register is read back, the value that was written to FADDR gets right shift by 2 to indicate 32-bit aligned address. In other words lower 2 bits are discarded while reading back the register. Out of range address results in roll over. There is no status signal generated by flash controller to indicate this. Firmware is responsible to managing the addresses correctly."]
    #[inline(always)]
    pub fn faddr(&mut self) -> FADDR_W {
        FADDR_W { w: self }
    }
}