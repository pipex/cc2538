#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Writer for register IMR"]
pub type W = crate::W<u32, super::IMR>;
#[doc = "Register IMR `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR {
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
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `TBMIM`"]
pub type TBMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMIM`"]
pub struct TBMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CBEIM`"]
pub type CBEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBEIM`"]
pub struct CBEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEIM_W<'a> {
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
#[doc = "Reader of field `CBMIM`"]
pub type CBMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMIM`"]
pub struct CBMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMIM_W<'a> {
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
#[doc = "Reader of field `TBTOIM`"]
pub type TBTOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTOIM`"]
pub struct TBTOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `TAMIM`"]
pub type TAMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMIM`"]
pub struct TAMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CAEIM`"]
pub type CAEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAEIM`"]
pub struct CAEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEIM_W<'a> {
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
#[doc = "Reader of field `CAMIM`"]
pub type CAMIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMIM`"]
pub struct CAMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMIM_W<'a> {
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
#[doc = "Reader of field `TATOIM`"]
pub type TATOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATOIM`"]
pub struct TATOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOIM_W<'a> {
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
    #[doc = "Bits 12:31 - 31:12\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbmim(&self) -> TBMIM_R {
        TBMIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbeim(&self) -> CBEIM_R {
        CBEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbmim(&self) -> CBMIM_R {
        CBMIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbtoim(&self) -> TBTOIM_R {
        TBTOIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tamim(&self) -> TAMIM_R {
        TAMIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn caeim(&self) -> CAEIM_R {
        CAEIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn camim(&self) -> CAMIM_R {
        CAMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tatoim(&self) -> TATOIM_R {
        TATOIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbmim(&mut self) -> TBMIM_W {
        TBMIM_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbeim(&mut self) -> CBEIM_W {
        CBEIM_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbmim(&mut self) -> CBMIM_W {
        CBMIM_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbtoim(&mut self) -> TBTOIM_W {
        TBTOIM_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tamim(&mut self) -> TAMIM_W {
        TAMIM_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn caeim(&mut self) -> CAEIM_W {
        CAEIM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn camim(&mut self) -> CAMIM_W {
        CAMIM_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tatoim(&mut self) -> TATOIM_W {
        TATOIM_W { w: self }
    }
}
