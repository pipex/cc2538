#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
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
#[doc = "Reader of field `WUECINT`"]
pub type WUECINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUECINT`"]
pub struct WUECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUECINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `TBMCINT`"]
pub type TBMCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMCINT`"]
pub struct TBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMCINT_W<'a> {
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
#[doc = "Reader of field `CBECINT`"]
pub type CBECINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBECINT`"]
pub struct CBECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBECINT_W<'a> {
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
#[doc = "Reader of field `CBMCINT`"]
pub type CBMCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMCINT`"]
pub struct CBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMCINT_W<'a> {
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
#[doc = "Reader of field `TBTOCINT`"]
pub type TBTOCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTOCINT`"]
pub struct TBTOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOCINT_W<'a> {
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
#[doc = "Reader of field `TAMCINT`"]
pub type TAMCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMCINT`"]
pub struct TAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMCINT_W<'a> {
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
#[doc = "Reader of field `CAECINT`"]
pub type CAECINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAECINT`"]
pub struct CAECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAECINT_W<'a> {
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
#[doc = "Reader of field `CAMCINT`"]
pub type CAMCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMCINT`"]
pub struct CAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMCINT_W<'a> {
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
#[doc = "Reader of field `TATOCINT`"]
pub type TATOCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATOCINT`"]
pub struct TATOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOCINT_W<'a> {
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
    #[doc = "Bits 17:31 - 31:17\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\] GPTM write update error interrupt clear"]
    #[inline(always)]
    pub fn wuecint(&self) -> WUECINT_R {
        WUECINT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer B match interrupt clear"]
    #[inline(always)]
    pub fn tbmcint(&self) -> TBMCINT_R {
        TBMCINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] GPTM Timer B capture event Interrupt clear"]
    #[inline(always)]
    pub fn cbecint(&self) -> CBECINT_R {
        CBECINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B capture match interrupt clear"]
    #[inline(always)]
    pub fn cbmcint(&self) -> CBMCINT_R {
        CBMCINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B time-out interrupt clear"]
    #[inline(always)]
    pub fn tbtocint(&self) -> TBTOCINT_R {
        TBTOCINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer A match interrupt clear"]
    #[inline(always)]
    pub fn tamcint(&self) -> TAMCINT_R {
        TAMCINT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer A capture event Interrupt clear"]
    #[inline(always)]
    pub fn caecint(&self) -> CAECINT_R {
        CAECINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] GPTM Timer A capture match interrupt clear"]
    #[inline(always)]
    pub fn camcint(&self) -> CAMCINT_R {
        CAMCINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] GPTM Timer A time-out interrupt clear"]
    #[inline(always)]
    pub fn tatocint(&self) -> TATOCINT_R {
        TATOCINT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] GPTM write update error interrupt clear"]
    #[inline(always)]
    pub fn wuecint(&mut self) -> WUECINT_W {
        WUECINT_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer B match interrupt clear"]
    #[inline(always)]
    pub fn tbmcint(&mut self) -> TBMCINT_W {
        TBMCINT_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] GPTM Timer B capture event Interrupt clear"]
    #[inline(always)]
    pub fn cbecint(&mut self) -> CBECINT_W {
        CBECINT_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B capture match interrupt clear"]
    #[inline(always)]
    pub fn cbmcint(&mut self) -> CBMCINT_W {
        CBMCINT_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B time-out interrupt clear"]
    #[inline(always)]
    pub fn tbtocint(&mut self) -> TBTOCINT_W {
        TBTOCINT_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer A match interrupt clear"]
    #[inline(always)]
    pub fn tamcint(&mut self) -> TAMCINT_W {
        TAMCINT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer A capture event Interrupt clear"]
    #[inline(always)]
    pub fn caecint(&mut self) -> CAECINT_W {
        CAECINT_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] GPTM Timer A capture match interrupt clear"]
    #[inline(always)]
    pub fn camcint(&mut self) -> CAMCINT_W {
        CAMCINT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] GPTM Timer A time-out interrupt clear"]
    #[inline(always)]
    pub fn tatocint(&mut self) -> TATOCINT_W {
        TATOCINT_W { w: self }
    }
}
