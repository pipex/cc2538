#[doc = "Reader of register TPR"]
pub type R = crate::R<u32, super::TPR>;
#[doc = "Writer for register TPR"]
pub type W = crate::W<u32, super::TPR>;
#[doc = "Register TPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TPR {
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
#[doc = "Reader of field `TPR`"]
pub type TPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TPR`"]
pub struct TPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPR_W<'a> {
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
    #[doc = "Bits 0:6 - 6:0\\] SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2 * (1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    pub fn tpr(&self) -> TPR_R {
        TPR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved25(&mut self) -> RESERVED25_W {
        RESERVED25_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\] SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2 * (1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    pub fn tpr(&mut self) -> TPR_W {
        TPR_W { w: self }
    }
}
