#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Writer for register RIS"]
pub type W = crate::W<u32, super::RIS>;
#[doc = "Register RIS `reset()`'s with value 0"]
impl crate::ResetValue for super::RIS {
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
#[doc = "Reader of field `STOPRIS`"]
pub type STOPRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPRIS`"]
pub struct STOPRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPRIS_W<'a> {
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
#[doc = "Reader of field `STARTRIS`"]
pub type STARTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTRIS`"]
pub struct STARTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTRIS_W<'a> {
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
#[doc = "Reader of field `DATARIS`"]
pub type DATARIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATARIS`"]
pub struct DATARIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARIS_W<'a> {
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
    #[doc = "Bit 2 - 2:2\\] Stop condition raw interrupt status 1: A STOP condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn stopris(&self) -> STOPRIS_R {
        STOPRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Start condition raw interrupt status 1: A START condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn startris(&self) -> STARTRIS_R {
        STARTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Data raw interrupt status 1: A data received or data requested interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn dataris(&self) -> DATARIS_R {
        DATARIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Stop condition raw interrupt status 1: A STOP condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STOPIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn stopris(&mut self) -> STOPRIS_W {
        STOPRIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Start condition raw interrupt status 1: A START condition interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the STARTIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn startris(&mut self) -> STARTRIS_W {
        STARTRIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Data raw interrupt status 1: A data received or data requested interrupt is pending. 0: No interrupt This bit is cleared by writing 1 to the DATAIC bit in the I2CSICR register."]
    #[inline(always)]
    pub fn dataris(&mut self) -> DATARIS_W {
        DATARIS_W { w: self }
    }
}
