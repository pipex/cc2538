#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
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
#[doc = "Reader of field `FBR`"]
pub type FBR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBR`"]
pub struct FBR_W<'a> {
    w: &'a mut W,
}
impl<'a> FBR_W<'a> {
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
#[doc = "Reader of field `TREQ`"]
pub type TREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TREQ`"]
pub struct TREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TREQ_W<'a> {
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
#[doc = "Reader of field `RREQ`"]
pub type RREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RREQ`"]
pub struct RREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RREQ_W<'a> {
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
    #[doc = "Bit 2 - 2:2\\] First byte received 1: The first byte following the slave's own address has been received. 0: The first byte has not been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the I2CSDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    pub fn fbr(&self) -> FBR_R {
        FBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Transmit request 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the I2CSDR register. 0: No outstanding transmit request."]
    #[inline(always)]
    pub fn treq(&self) -> TREQ_R {
        TREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Receive request 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the I2CSDR register. 0: No outstanding receive data"]
    #[inline(always)]
    pub fn rreq(&self) -> RREQ_R {
        RREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] First byte received 1: The first byte following the slave's own address has been received. 0: The first byte has not been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the I2CSDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    pub fn fbr(&mut self) -> FBR_W {
        FBR_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Transmit request 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the I2CSDR register. 0: No outstanding transmit request."]
    #[inline(always)]
    pub fn treq(&mut self) -> TREQ_W {
        TREQ_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Receive request 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the I2CSDR register. 0: No outstanding receive data"]
    #[inline(always)]
    pub fn rreq(&mut self) -> RREQ_W {
        RREQ_W { w: self }
    }
}
