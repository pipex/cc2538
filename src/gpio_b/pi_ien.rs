#[doc = "Reader of register PI_IEN"]
pub type R = crate::R<u32, super::PI_IEN>;
#[doc = "Writer for register PI_IEN"]
pub type W = crate::W<u32, super::PI_IEN>;
#[doc = "Register PI_IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::PI_IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDIEN7`"]
pub type PDIEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIEN7`"]
pub struct PDIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `PDIEN6`"]
pub type PDIEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIEN6`"]
pub struct PDIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PDIEN5`"]
pub type PDIEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIEN5`"]
pub struct PDIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `PDIEN4`"]
pub type PDIEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIEN4`"]
pub struct PDIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PDIEN3`"]
pub type PDIEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIEN3`"]
pub struct PDIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PDIEN2`"]
pub type PDIEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIEN2`"]
pub struct PDIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PDIEN1`"]
pub type PDIEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIEN1`"]
pub struct PDIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PDIEN0`"]
pub type PDIEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIEN0`"]
pub struct PDIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIEN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `PCIEN7`"]
pub type PCIEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIEN7`"]
pub struct PCIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `PCIEN6`"]
pub type PCIEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIEN6`"]
pub struct PCIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PCIEN5`"]
pub type PCIEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIEN5`"]
pub struct PCIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PCIEN4`"]
pub type PCIEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIEN4`"]
pub struct PCIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PCIEN3`"]
pub type PCIEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIEN3`"]
pub struct PCIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PCIEN2`"]
pub type PCIEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIEN2`"]
pub struct PCIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PCIEN1`"]
pub type PCIEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIEN1`"]
pub struct PCIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PCIEN0`"]
pub type PCIEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIEN0`"]
pub struct PCIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIEN0_W<'a> {
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
#[doc = "Reader of field `PBIEN7`"]
pub type PBIEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIEN7`"]
pub struct PBIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PBIEN6`"]
pub type PBIEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIEN6`"]
pub struct PBIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PBIEN5`"]
pub type PBIEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIEN5`"]
pub struct PBIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PBIEN4`"]
pub type PBIEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIEN4`"]
pub struct PBIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PBIEN3`"]
pub type PBIEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIEN3`"]
pub struct PBIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN3_W<'a> {
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
#[doc = "Reader of field `PBIEN2`"]
pub type PBIEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIEN2`"]
pub struct PBIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN2_W<'a> {
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
#[doc = "Reader of field `PBIEN1`"]
pub type PBIEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIEN1`"]
pub struct PBIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN1_W<'a> {
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
#[doc = "Reader of field `PBIEN0`"]
pub type PBIEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIEN0`"]
pub struct PBIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIEN0_W<'a> {
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
#[doc = "Reader of field `PAIEN7`"]
pub type PAIEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIEN7`"]
pub struct PAIEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PAIEN6`"]
pub type PAIEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIEN6`"]
pub struct PAIEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PAIEN5`"]
pub type PAIEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIEN5`"]
pub struct PAIEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PAIEN4`"]
pub type PAIEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIEN4`"]
pub struct PAIEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN4_W<'a> {
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
#[doc = "Reader of field `PAIEN3`"]
pub type PAIEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIEN3`"]
pub struct PAIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN3_W<'a> {
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
#[doc = "Reader of field `PAIEN2`"]
pub type PAIEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIEN2`"]
pub struct PAIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN2_W<'a> {
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
#[doc = "Reader of field `PAIEN1`"]
pub type PAIEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIEN1`"]
pub struct PAIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN1_W<'a> {
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
#[doc = "Reader of field `PAIEN0`"]
pub type PAIEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIEN0`"]
pub struct PAIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIEN0_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien7(&self) -> PDIEN7_R {
        PDIEN7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\] Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien6(&self) -> PDIEN6_R {
        PDIEN6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\] Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien5(&self) -> PDIEN5_R {
        PDIEN5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\] Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien4(&self) -> PDIEN4_R {
        PDIEN4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\] Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien3(&self) -> PDIEN3_R {
        PDIEN3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\] Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien2(&self) -> PDIEN2_R {
        PDIEN2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\] Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien1(&self) -> PDIEN1_R {
        PDIEN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\] Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien0(&self) -> PDIEN0_R {
        PDIEN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\] Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien7(&self) -> PCIEN7_R {
        PCIEN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\] Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien6(&self) -> PCIEN6_R {
        PCIEN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\] Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien5(&self) -> PCIEN5_R {
        PCIEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\] Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien4(&self) -> PCIEN4_R {
        PCIEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\] Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien3(&self) -> PCIEN3_R {
        PCIEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\] Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien2(&self) -> PCIEN2_R {
        PCIEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\] Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien1(&self) -> PCIEN1_R {
        PCIEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\] Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien0(&self) -> PCIEN0_R {
        PCIEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\] Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien7(&self) -> PBIEN7_R {
        PBIEN7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\] Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien6(&self) -> PBIEN6_R {
        PBIEN6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\] Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien5(&self) -> PBIEN5_R {
        PBIEN5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\] Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien4(&self) -> PBIEN4_R {
        PBIEN4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\] Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien3(&self) -> PBIEN3_R {
        PBIEN3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien2(&self) -> PBIEN2_R {
        PBIEN2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien1(&self) -> PBIEN1_R {
        PBIEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien0(&self) -> PBIEN0_R {
        PBIEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien7(&self) -> PAIEN7_R {
        PAIEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien6(&self) -> PAIEN6_R {
        PAIEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien5(&self) -> PAIEN5_R {
        PAIEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien4(&self) -> PAIEN4_R {
        PAIEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien3(&self) -> PAIEN3_R {
        PAIEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien2(&self) -> PAIEN2_R {
        PAIEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien1(&self) -> PAIEN1_R {
        PAIEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien0(&self) -> PAIEN0_R {
        PAIEN0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\] Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien7(&mut self) -> PDIEN7_W {
        PDIEN7_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien6(&mut self) -> PDIEN6_W {
        PDIEN6_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien5(&mut self) -> PDIEN5_W {
        PDIEN5_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien4(&mut self) -> PDIEN4_W {
        PDIEN4_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien3(&mut self) -> PDIEN3_W {
        PDIEN3_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien2(&mut self) -> PDIEN2_W {
        PDIEN2_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien1(&mut self) -> PDIEN1_W {
        PDIEN1_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien0(&mut self) -> PDIEN0_W {
        PDIEN0_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien7(&mut self) -> PCIEN7_W {
        PCIEN7_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien6(&mut self) -> PCIEN6_W {
        PCIEN6_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien5(&mut self) -> PCIEN5_W {
        PCIEN5_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien4(&mut self) -> PCIEN4_W {
        PCIEN4_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien3(&mut self) -> PCIEN3_W {
        PCIEN3_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien2(&mut self) -> PCIEN2_W {
        PCIEN2_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien1(&mut self) -> PCIEN1_W {
        PCIEN1_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien0(&mut self) -> PCIEN0_W {
        PCIEN0_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien7(&mut self) -> PBIEN7_W {
        PBIEN7_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien6(&mut self) -> PBIEN6_W {
        PBIEN6_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien5(&mut self) -> PBIEN5_W {
        PBIEN5_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien4(&mut self) -> PBIEN4_W {
        PBIEN4_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien3(&mut self) -> PBIEN3_W {
        PBIEN3_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien2(&mut self) -> PBIEN2_W {
        PBIEN2_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien1(&mut self) -> PBIEN1_W {
        PBIEN1_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien0(&mut self) -> PBIEN0_W {
        PBIEN0_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien7(&mut self) -> PAIEN7_W {
        PAIEN7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien6(&mut self) -> PAIEN6_W {
        PAIEN6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien5(&mut self) -> PAIEN5_W {
        PAIEN5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien4(&mut self) -> PAIEN4_W {
        PAIEN4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien3(&mut self) -> PAIEN3_W {
        PAIEN3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien2(&mut self) -> PAIEN2_W {
        PAIEN2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien1(&mut self) -> PAIEN1_W {
        PAIEN1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien0(&mut self) -> PAIEN0_W {
        PAIEN0_W { w: self }
    }
}
