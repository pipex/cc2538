#[doc = "Reader of register DMAC_OPTIONS"]
pub type R = crate::R<u32, super::DMAC_OPTIONS>;
#[doc = "Writer for register DMAC_OPTIONS"]
pub type W = crate::W<u32, super::DMAC_OPTIONS>;
#[doc = "Register DMAC_OPTIONS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAC_OPTIONS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved20`"]
pub type RESERVED20_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved20`"]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `NR_OF_CHANNELS`"]
pub type NR_OF_CHANNELS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NR_OF_CHANNELS`"]
pub struct NR_OF_CHANNELS_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_OF_CHANNELS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `Reserved5`"]
pub type RESERVED5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `NR_OF_PORTS`"]
pub type NR_OF_PORTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NR_OF_PORTS`"]
pub struct NR_OF_PORTS_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_OF_PORTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 8:11 - 11:8\\] Number of channels implemented, value in the range 1-8."]
    #[inline(always)]
    pub fn nr_of_channels(&self) -> NR_OF_CHANNELS_R {
        NR_OF_CHANNELS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\] Number of ports implemented, value in range 1-4."]
    #[inline(always)]
    pub fn nr_of_ports(&self) -> NR_OF_PORTS_R {
        NR_OF_PORTS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] Number of channels implemented, value in the range 1-8."]
    #[inline(always)]
    pub fn nr_of_channels(&mut self) -> NR_OF_CHANNELS_W {
        NR_OF_CHANNELS_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Bits should be ignored on read"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Number of ports implemented, value in range 1-4."]
    #[inline(always)]
    pub fn nr_of_ports(&mut self) -> NR_OF_PORTS_W {
        NR_OF_PORTS_W { w: self }
    }
}
