#[doc = "Reader of register OAR"]
pub type R = crate::R<u32, super::OAR>;
#[doc = "Writer for register OAR"]
pub type W = crate::W<u32, super::OAR>;
#[doc = "Register OAR `reset()`'s with value 0"]
impl crate::ResetValue for super::OAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved25`"]
pub type RESERVED25_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved25`"]
pub struct RESERVED25_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Reader of field `OAR`"]
pub type OAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OAR`"]
pub struct OAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - 31:7\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bits 0:6 - 6:0\\] I2C slave own address This field specifies bits A6 through A0 of the slave address."]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\] I2C slave own address This field specifies bits A6 through A0 of the slave address."]
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W {
        OAR_W { w: self }
    }
}
