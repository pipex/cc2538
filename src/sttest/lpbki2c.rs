#[doc = "Reader of register LPBKI2C"]
pub type R = crate::R<u32, super::LPBKI2C>;
#[doc = "Writer for register LPBKI2C"]
pub type W = crate::W<u32, super::LPBKI2C>;
#[doc = "Register LPBKI2C `reset()`'s with value 0"]
impl crate::ResetValue for super::LPBKI2C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPBKI2C`"]
pub type LPBKI2C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPBKI2C`"]
pub struct LPBKI2C_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBKI2C_W<'a> {
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
    #[doc = "Bit 0 - 0:0\\] I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&self) -> LPBKI2C_R {
        LPBKI2C_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\] I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&mut self) -> LPBKI2C_W {
        LPBKI2C_W { w: self }
    }
}
