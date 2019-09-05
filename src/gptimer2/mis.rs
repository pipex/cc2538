#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Writer for register MIS"]
pub type W = crate::W<u32, super::MIS>;
#[doc = "Register MIS `reset()`'s with value 0"]
impl crate::ResetValue for super::MIS {
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
#[doc = "Reader of field `TBMMIS`"]
pub type TBMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMMIS`"]
pub struct TBMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMMIS_W<'a> {
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
#[doc = "Reader of field `CBEMIS`"]
pub type CBEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBEMIS`"]
pub struct CBEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEMIS_W<'a> {
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
#[doc = "Reader of field `CBMMIS`"]
pub type CBMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMMIS`"]
pub struct CBMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMMIS_W<'a> {
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
#[doc = "Reader of field `TBTOMIS`"]
pub type TBTOMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTOMIS`"]
pub struct TBTOMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOMIS_W<'a> {
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
#[doc = "Reader of field `TAMRIS`"]
pub type TAMRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMRIS`"]
pub struct TAMRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMRIS_W<'a> {
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
#[doc = "Reader of field `CAEMIS`"]
pub type CAEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAEMIS`"]
pub struct CAEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEMIS_W<'a> {
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
#[doc = "Reader of field `CAMMIS`"]
pub type CAMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMMIS`"]
pub struct CAMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMMIS_W<'a> {
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
#[doc = "Reader of field `TATOMIS`"]
pub type TATOMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATOMIS`"]
pub struct TATOMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOMIS_W<'a> {
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
    #[doc = "Bit 11 - 11:11\\] GPTM Timer B match masked interrupt"]
    #[inline(always)]
    pub fn tbmmis(&self) -> TBMMIS_R {
        TBMMIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] GPTM Timer B capture event masked interrupt"]
    #[inline(always)]
    pub fn cbemis(&self) -> CBEMIS_R {
        CBEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B capture match masked interrupt"]
    #[inline(always)]
    pub fn cbmmis(&self) -> CBMMIS_R {
        CBMMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B time-out masked interrupt"]
    #[inline(always)]
    pub fn tbtomis(&self) -> TBTOMIS_R {
        TBTOMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer A match raw interrupt"]
    #[inline(always)]
    pub fn tamris(&self) -> TAMRIS_R {
        TAMRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer A capture event raw interrupt"]
    #[inline(always)]
    pub fn caemis(&self) -> CAEMIS_R {
        CAEMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] GPTM Timer A capture match raw interrupt"]
    #[inline(always)]
    pub fn cammis(&self) -> CAMMIS_R {
        CAMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] GPTM Timer A time-out raw interrupt"]
    #[inline(always)]
    pub fn tatomis(&self) -> TATOMIS_R {
        TATOMIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] GPTM Timer B match masked interrupt"]
    #[inline(always)]
    pub fn tbmmis(&mut self) -> TBMMIS_W {
        TBMMIS_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] GPTM Timer B capture event masked interrupt"]
    #[inline(always)]
    pub fn cbemis(&mut self) -> CBEMIS_W {
        CBEMIS_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B capture match masked interrupt"]
    #[inline(always)]
    pub fn cbmmis(&mut self) -> CBMMIS_W {
        CBMMIS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B time-out masked interrupt"]
    #[inline(always)]
    pub fn tbtomis(&mut self) -> TBTOMIS_W {
        TBTOMIS_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer A match raw interrupt"]
    #[inline(always)]
    pub fn tamris(&mut self) -> TAMRIS_W {
        TAMRIS_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer A capture event raw interrupt"]
    #[inline(always)]
    pub fn caemis(&mut self) -> CAEMIS_W {
        CAEMIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] GPTM Timer A capture match raw interrupt"]
    #[inline(always)]
    pub fn cammis(&mut self) -> CAMMIS_W {
        CAMMIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] GPTM Timer A time-out raw interrupt"]
    #[inline(always)]
    pub fn tatomis(&mut self) -> TATOMIS_W {
        TATOMIS_W { w: self }
    }
}
