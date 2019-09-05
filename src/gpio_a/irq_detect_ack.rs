#[doc = "Reader of register IRQ_DETECT_ACK"]
pub type R = crate::R<u32, super::IRQ_DETECT_ACK>;
#[doc = "Writer for register IRQ_DETECT_ACK"]
pub type W = crate::W<u32, super::IRQ_DETECT_ACK>;
#[doc = "Register IRQ_DETECT_ACK `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_DETECT_ACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDIACK7`"]
pub type PDIACK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIACK7`"]
pub struct PDIACK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK7_W<'a> {
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
#[doc = "Reader of field `PDIACK6`"]
pub type PDIACK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIACK6`"]
pub struct PDIACK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK6_W<'a> {
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
#[doc = "Reader of field `PDIACK5`"]
pub type PDIACK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIACK5`"]
pub struct PDIACK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK5_W<'a> {
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
#[doc = "Reader of field `PDIACK4`"]
pub type PDIACK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIACK4`"]
pub struct PDIACK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK4_W<'a> {
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
#[doc = "Reader of field `PDIACK3`"]
pub type PDIACK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIACK3`"]
pub struct PDIACK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK3_W<'a> {
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
#[doc = "Reader of field `PDIACK2`"]
pub type PDIACK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIACK2`"]
pub struct PDIACK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK2_W<'a> {
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
#[doc = "Reader of field `PDIACK1`"]
pub type PDIACK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIACK1`"]
pub struct PDIACK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK1_W<'a> {
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
#[doc = "Reader of field `PDIACK0`"]
pub type PDIACK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIACK0`"]
pub struct PDIACK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIACK0_W<'a> {
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
#[doc = "Reader of field `PCIACK7`"]
pub type PCIACK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIACK7`"]
pub struct PCIACK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK7_W<'a> {
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
#[doc = "Reader of field `PCIACK6`"]
pub type PCIACK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIACK6`"]
pub struct PCIACK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK6_W<'a> {
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
#[doc = "Reader of field `PCIACK5`"]
pub type PCIACK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIACK5`"]
pub struct PCIACK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK5_W<'a> {
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
#[doc = "Reader of field `PCIACK4`"]
pub type PCIACK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIACK4`"]
pub struct PCIACK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK4_W<'a> {
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
#[doc = "Reader of field `PCIACK3`"]
pub type PCIACK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIACK3`"]
pub struct PCIACK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK3_W<'a> {
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
#[doc = "Reader of field `PCIACK2`"]
pub type PCIACK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIACK2`"]
pub struct PCIACK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK2_W<'a> {
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
#[doc = "Reader of field `PCIACK1`"]
pub type PCIACK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIACK1`"]
pub struct PCIACK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK1_W<'a> {
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
#[doc = "Reader of field `PCIACK0`"]
pub type PCIACK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIACK0`"]
pub struct PCIACK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIACK0_W<'a> {
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
#[doc = "Reader of field `PBIACK7`"]
pub type PBIACK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIACK7`"]
pub struct PBIACK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK7_W<'a> {
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
#[doc = "Reader of field `PBIACK6`"]
pub type PBIACK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIACK6`"]
pub struct PBIACK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK6_W<'a> {
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
#[doc = "Reader of field `PBIACK5`"]
pub type PBIACK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIACK5`"]
pub struct PBIACK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK5_W<'a> {
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
#[doc = "Reader of field `PBIACK4`"]
pub type PBIACK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIACK4`"]
pub struct PBIACK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK4_W<'a> {
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
#[doc = "Reader of field `PBIACK3`"]
pub type PBIACK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIACK3`"]
pub struct PBIACK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK3_W<'a> {
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
#[doc = "Reader of field `PBIACK2`"]
pub type PBIACK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIACK2`"]
pub struct PBIACK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK2_W<'a> {
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
#[doc = "Reader of field `PBIACK1`"]
pub type PBIACK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIACK1`"]
pub struct PBIACK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK1_W<'a> {
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
#[doc = "Reader of field `PBIACK0`"]
pub type PBIACK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIACK0`"]
pub struct PBIACK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIACK0_W<'a> {
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
#[doc = "Reader of field `PAIACK7`"]
pub type PAIACK7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIACK7`"]
pub struct PAIACK7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK7_W<'a> {
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
#[doc = "Reader of field `PAIACK6`"]
pub type PAIACK6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIACK6`"]
pub struct PAIACK6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK6_W<'a> {
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
#[doc = "Reader of field `PAIACK5`"]
pub type PAIACK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIACK5`"]
pub struct PAIACK5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK5_W<'a> {
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
#[doc = "Reader of field `PAIACK4`"]
pub type PAIACK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIACK4`"]
pub struct PAIACK4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK4_W<'a> {
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
#[doc = "Reader of field `PAIACK3`"]
pub type PAIACK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIACK3`"]
pub struct PAIACK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK3_W<'a> {
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
#[doc = "Reader of field `PAIACK2`"]
pub type PAIACK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIACK2`"]
pub struct PAIACK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK2_W<'a> {
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
#[doc = "Reader of field `PAIACK1`"]
pub type PAIACK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIACK1`"]
pub struct PAIACK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK1_W<'a> {
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
#[doc = "Reader of field `PAIACK0`"]
pub type PAIACK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIACK0`"]
pub struct PAIACK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIACK0_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack7(&self) -> PDIACK7_R {
        PDIACK7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\] Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack6(&self) -> PDIACK6_R {
        PDIACK6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\] Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack5(&self) -> PDIACK5_R {
        PDIACK5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\] Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack4(&self) -> PDIACK4_R {
        PDIACK4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\] Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack3(&self) -> PDIACK3_R {
        PDIACK3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\] Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack2(&self) -> PDIACK2_R {
        PDIACK2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\] Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
    #[inline(always)]
    pub fn pdiack1(&self) -> PDIACK1_R {
        PDIACK1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\] Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack0(&self) -> PDIACK0_R {
        PDIACK0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\] Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack7(&self) -> PCIACK7_R {
        PCIACK7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\] Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack6(&self) -> PCIACK6_R {
        PCIACK6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\] Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack5(&self) -> PCIACK5_R {
        PCIACK5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\] Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack4(&self) -> PCIACK4_R {
        PCIACK4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\] Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack3(&self) -> PCIACK3_R {
        PCIACK3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\] Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack2(&self) -> PCIACK2_R {
        PCIACK2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\] Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack1(&self) -> PCIACK1_R {
        PCIACK1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\] Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack0(&self) -> PCIACK0_R {
        PCIACK0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\] Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack7(&self) -> PBIACK7_R {
        PBIACK7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\] Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack6(&self) -> PBIACK6_R {
        PBIACK6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\] Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack5(&self) -> PBIACK5_R {
        PBIACK5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\] Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack4(&self) -> PBIACK4_R {
        PBIACK4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\] Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack3(&self) -> PBIACK3_R {
        PBIACK3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack2(&self) -> PBIACK2_R {
        PBIACK2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack1(&self) -> PBIACK1_R {
        PBIACK1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack0(&self) -> PBIACK0_R {
        PBIACK0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack7(&self) -> PAIACK7_R {
        PAIACK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack6(&self) -> PAIACK6_R {
        PAIACK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack5(&self) -> PAIACK5_R {
        PAIACK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack4(&self) -> PAIACK4_R {
        PAIACK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack3(&self) -> PAIACK3_R {
        PAIACK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack2(&self) -> PAIACK2_R {
        PAIACK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack1(&self) -> PAIACK1_R {
        PAIACK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack0(&self) -> PAIACK0_R {
        PAIACK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\] Port D bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack7(&mut self) -> PDIACK7_W {
        PDIACK7_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Port D bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack6(&mut self) -> PDIACK6_W {
        PDIACK6_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Port D bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack5(&mut self) -> PDIACK5_W {
        PDIACK5_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Port D bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack4(&mut self) -> PDIACK4_W {
        PDIACK4_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Port D bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack3(&mut self) -> PDIACK3_W {
        PDIACK3_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Port D bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack2(&mut self) -> PDIACK2_W {
        PDIACK2_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Port D bit 1 masked interrupt status: 1: Detected0: Not detected"]
    #[inline(always)]
    pub fn pdiack1(&mut self) -> PDIACK1_W {
        PDIACK1_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Port D bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pdiack0(&mut self) -> PDIACK0_W {
        PDIACK0_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Port C bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack7(&mut self) -> PCIACK7_W {
        PCIACK7_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Port C bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack6(&mut self) -> PCIACK6_W {
        PCIACK6_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Port C bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack5(&mut self) -> PCIACK5_W {
        PCIACK5_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Port C bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack4(&mut self) -> PCIACK4_W {
        PCIACK4_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Port C bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack3(&mut self) -> PCIACK3_W {
        PCIACK3_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Port C bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack2(&mut self) -> PCIACK2_W {
        PCIACK2_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Port C bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack1(&mut self) -> PCIACK1_W {
        PCIACK1_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Port C bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pciack0(&mut self) -> PCIACK0_W {
        PCIACK0_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Port B bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack7(&mut self) -> PBIACK7_W {
        PBIACK7_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Port B bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack6(&mut self) -> PBIACK6_W {
        PBIACK6_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Port B bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack5(&mut self) -> PBIACK5_W {
        PBIACK5_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Port B bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack4(&mut self) -> PBIACK4_W {
        PBIACK4_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Port B bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack3(&mut self) -> PBIACK3_W {
        PBIACK3_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Port B bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack2(&mut self) -> PBIACK2_W {
        PBIACK2_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Port B bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack1(&mut self) -> PBIACK1_W {
        PBIACK1_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Port B bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn pbiack0(&mut self) -> PBIACK0_W {
        PBIACK0_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Port A bit 7 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack7(&mut self) -> PAIACK7_W {
        PAIACK7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Port A bit 6 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack6(&mut self) -> PAIACK6_W {
        PAIACK6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Port A bit 5 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack5(&mut self) -> PAIACK5_W {
        PAIACK5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Port A bit 4 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack4(&mut self) -> PAIACK4_W {
        PAIACK4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Port A bit 3 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack3(&mut self) -> PAIACK3_W {
        PAIACK3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Port A bit 2 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack2(&mut self) -> PAIACK2_W {
        PAIACK2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Port A bit 1 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack1(&mut self) -> PAIACK1_W {
        PAIACK1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Port A bit 0 masked interrupt status: 1: Detected 0: Not detected"]
    #[inline(always)]
    pub fn paiack0(&mut self) -> PAIACK0_W {
        PAIACK0_W { w: self }
    }
}
