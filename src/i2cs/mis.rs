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
#[doc = "Reader of field `Reserved29`"]
pub type RESERVED29_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved29`"]
pub struct RESERVED29_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `STOPMIS`"]
pub type STOPMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPMIS`"]
pub struct STOPMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPMIS_W<'a> {
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
#[doc = "Reader of field `STARTMIS`"]
pub type STARTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTMIS`"]
pub struct STARTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTMIS_W<'a> {
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
#[doc = "Reader of field `DATAMIS`"]
pub type DATAMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAMIS`"]
pub struct DATAMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAMIS_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\] Stop condition masked interrupt status 1: An unmasked STOP condition interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn stopmis(&self) -> STOPMIS_R {
        STOPMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Start condition masked interrupt status 1: An unmasked START condition interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn startmis(&self) -> STARTMIS_R {
        STARTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Data masked interrupt status 1: An unmasked data received or data requested interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn datamis(&self) -> DATAMIS_R {
        DATAMIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Stop condition masked interrupt status 1: An unmasked STOP condition interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn stopmis(&mut self) -> STOPMIS_W {
        STOPMIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Start condition masked interrupt status 1: An unmasked START condition interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn startmis(&mut self) -> STARTMIS_W {
        STARTMIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Data masked interrupt status 1: An unmasked data received or data requested interrupt is pending. 0: An interrupt has not occurred or is masked. This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn datamis(&mut self) -> DATAMIS_W {
        DATAMIS_W { w: self }
    }
}
