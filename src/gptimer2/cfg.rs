#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPTMCFG`"]
pub type GPTMCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPTMCFG`"]
pub struct GPTMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTMCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\] GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\] of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn gptmcfg(&self) -> GPTMCFG_R {
        GPTMCFG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\] GPTM configuration The GPTMCFG values are defined as follows: 0x0: 32-bit timer configuration. 0x1: 32-bit real-time clock 0x2: Reserved 0x3: Reserved 0x4: 16-bit timer configuration. The function is controlled by bits \\[1:0\\] of GPTMTAMR and GPTMTBMR. 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn gptmcfg(&mut self) -> GPTMCFG_W {
        GPTMCFG_W { w: self }
    }
}
