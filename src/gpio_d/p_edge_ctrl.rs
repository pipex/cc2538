#[doc = "Reader of register P_EDGE_CTRL"]
pub type R = crate::R<u32, super::P_EDGE_CTRL>;
#[doc = "Writer for register P_EDGE_CTRL"]
pub type W = crate::W<u32, super::P_EDGE_CTRL>;
#[doc = "Register P_EDGE_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::P_EDGE_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDIRC7`"]
pub type PDIRC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIRC7`"]
pub struct PDIRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC7_W<'a> {
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
#[doc = "Reader of field `PDIRC6`"]
pub type PDIRC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIRC6`"]
pub struct PDIRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC6_W<'a> {
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
#[doc = "Reader of field `PDIRC5`"]
pub type PDIRC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIRC5`"]
pub struct PDIRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC5_W<'a> {
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
#[doc = "Reader of field `PDIRC4`"]
pub type PDIRC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIRC4`"]
pub struct PDIRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC4_W<'a> {
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
#[doc = "Reader of field `PDIRC3`"]
pub type PDIRC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIRC3`"]
pub struct PDIRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC3_W<'a> {
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
#[doc = "Reader of field `PDIRC2`"]
pub type PDIRC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIRC2`"]
pub struct PDIRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC2_W<'a> {
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
#[doc = "Reader of field `PDIRC1`"]
pub type PDIRC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIRC1`"]
pub struct PDIRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC1_W<'a> {
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
#[doc = "Reader of field `PDIRC0`"]
pub type PDIRC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDIRC0`"]
pub struct PDIRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIRC0_W<'a> {
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
#[doc = "Reader of field `PCIRC7`"]
pub type PCIRC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIRC7`"]
pub struct PCIRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC7_W<'a> {
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
#[doc = "Reader of field `PCIRC6`"]
pub type PCIRC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIRC6`"]
pub struct PCIRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC6_W<'a> {
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
#[doc = "Reader of field `PCIRC5`"]
pub type PCIRC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIRC5`"]
pub struct PCIRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC5_W<'a> {
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
#[doc = "Reader of field `PCIRC4`"]
pub type PCIRC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIRC4`"]
pub struct PCIRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC4_W<'a> {
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
#[doc = "Reader of field `PCIRC3`"]
pub type PCIRC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIRC3`"]
pub struct PCIRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC3_W<'a> {
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
#[doc = "Reader of field `PCIRC2`"]
pub type PCIRC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIRC2`"]
pub struct PCIRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC2_W<'a> {
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
#[doc = "Reader of field `PCIRC1`"]
pub type PCIRC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIRC1`"]
pub struct PCIRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC1_W<'a> {
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
#[doc = "Reader of field `PCIRC0`"]
pub type PCIRC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCIRC0`"]
pub struct PCIRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCIRC0_W<'a> {
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
#[doc = "Reader of field `PBIRC7`"]
pub type PBIRC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIRC7`"]
pub struct PBIRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC7_W<'a> {
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
#[doc = "Reader of field `PBIRC6`"]
pub type PBIRC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIRC6`"]
pub struct PBIRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC6_W<'a> {
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
#[doc = "Reader of field `PBIRC5`"]
pub type PBIRC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIRC5`"]
pub struct PBIRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC5_W<'a> {
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
#[doc = "Reader of field `PBIRC4`"]
pub type PBIRC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIRC4`"]
pub struct PBIRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC4_W<'a> {
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
#[doc = "Reader of field `PBIRC3`"]
pub type PBIRC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIRC3`"]
pub struct PBIRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC3_W<'a> {
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
#[doc = "Reader of field `PBIRC2`"]
pub type PBIRC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIRC2`"]
pub struct PBIRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC2_W<'a> {
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
#[doc = "Reader of field `PBIRC1`"]
pub type PBIRC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIRC1`"]
pub struct PBIRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC1_W<'a> {
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
#[doc = "Reader of field `PBIRC0`"]
pub type PBIRC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBIRC0`"]
pub struct PBIRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIRC0_W<'a> {
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
#[doc = "Reader of field `PAIRC7`"]
pub type PAIRC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRC7`"]
pub struct PAIRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC7_W<'a> {
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
#[doc = "Reader of field `PAIRC6`"]
pub type PAIRC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRC6`"]
pub struct PAIRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC6_W<'a> {
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
#[doc = "Reader of field `PAIRC5`"]
pub type PAIRC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRC5`"]
pub struct PAIRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC5_W<'a> {
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
#[doc = "Reader of field `PAIRC4`"]
pub type PAIRC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRC4`"]
pub struct PAIRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC4_W<'a> {
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
#[doc = "Reader of field `PAIRC3`"]
pub type PAIRC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRC3`"]
pub struct PAIRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC3_W<'a> {
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
#[doc = "Reader of field `PAIRC2`"]
pub type PAIRC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRC2`"]
pub struct PAIRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC2_W<'a> {
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
#[doc = "Reader of field `PAIRC1`"]
pub type PAIRC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRC1`"]
pub struct PAIRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC1_W<'a> {
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
#[doc = "Reader of field `PAIRC0`"]
pub type PAIRC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRC0`"]
pub struct PAIRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRC0_W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc7(&self) -> PDIRC7_R {
        PDIRC7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\] Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc6(&self) -> PDIRC6_R {
        PDIRC6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 29:29\\] Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc5(&self) -> PDIRC5_R {
        PDIRC5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 28:28\\] Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc4(&self) -> PDIRC4_R {
        PDIRC4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 27:27\\] Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc3(&self) -> PDIRC3_R {
        PDIRC3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 26:26\\] Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc2(&self) -> PDIRC2_R {
        PDIRC2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 25:25\\] Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc1(&self) -> PDIRC1_R {
        PDIRC1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\] Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc0(&self) -> PDIRC0_R {
        PDIRC0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\] Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc7(&self) -> PCIRC7_R {
        PCIRC7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 22:22\\] Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc6(&self) -> PCIRC6_R {
        PCIRC6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 21:21\\] Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc5(&self) -> PCIRC5_R {
        PCIRC5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 20:20\\] Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc4(&self) -> PCIRC4_R {
        PCIRC4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 19:19\\] Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc3(&self) -> PCIRC3_R {
        PCIRC3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 18:18\\] Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc2(&self) -> PCIRC2_R {
        PCIRC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 17:17\\] Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc1(&self) -> PCIRC1_R {
        PCIRC1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\] Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc0(&self) -> PCIRC0_R {
        PCIRC0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 15:15\\] Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc7(&self) -> PBIRC7_R {
        PBIRC7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 14:14\\] Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc6(&self) -> PBIRC6_R {
        PBIRC6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\] Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc5(&self) -> PBIRC5_R {
        PBIRC5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\] Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc4(&self) -> PBIRC4_R {
        PBIRC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\] Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc3(&self) -> PBIRC3_R {
        PBIRC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc2(&self) -> PBIRC2_R {
        PBIRC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc1(&self) -> PBIRC1_R {
        PBIRC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc0(&self) -> PBIRC0_R {
        PBIRC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc7(&self) -> PAIRC7_R {
        PAIRC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc6(&self) -> PAIRC6_R {
        PAIRC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc5(&self) -> PAIRC5_R {
        PAIRC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc4(&self) -> PAIRC4_R {
        PAIRC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc3(&self) -> PAIRC3_R {
        PAIRC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc2(&self) -> PAIRC2_R {
        PAIRC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc1(&self) -> PAIRC1_R {
        PAIRC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc0(&self) -> PAIRC0_R {
        PAIRC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\] Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc7(&mut self) -> PDIRC7_W {
        PDIRC7_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc6(&mut self) -> PDIRC6_W {
        PDIRC6_W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc5(&mut self) -> PDIRC5_W {
        PDIRC5_W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc4(&mut self) -> PDIRC4_W {
        PDIRC4_W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc3(&mut self) -> PDIRC3_W {
        PDIRC3_W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc2(&mut self) -> PDIRC2_W {
        PDIRC2_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc1(&mut self) -> PDIRC1_W {
        PDIRC1_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc0(&mut self) -> PDIRC0_W {
        PDIRC0_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc7(&mut self) -> PCIRC7_W {
        PCIRC7_W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc6(&mut self) -> PCIRC6_W {
        PCIRC6_W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc5(&mut self) -> PCIRC5_W {
        PCIRC5_W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc4(&mut self) -> PCIRC4_W {
        PCIRC4_W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc3(&mut self) -> PCIRC3_W {
        PCIRC3_W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc2(&mut self) -> PCIRC2_W {
        PCIRC2_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc1(&mut self) -> PCIRC1_W {
        PCIRC1_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc0(&mut self) -> PCIRC0_W {
        PCIRC0_W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc7(&mut self) -> PBIRC7_W {
        PBIRC7_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc6(&mut self) -> PBIRC6_W {
        PBIRC6_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc5(&mut self) -> PBIRC5_W {
        PBIRC5_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc4(&mut self) -> PBIRC4_W {
        PBIRC4_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc3(&mut self) -> PBIRC3_W {
        PBIRC3_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc2(&mut self) -> PBIRC2_W {
        PBIRC2_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc1(&mut self) -> PBIRC1_W {
        PBIRC1_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc0(&mut self) -> PBIRC0_W {
        PBIRC0_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc7(&mut self) -> PAIRC7_W {
        PAIRC7_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc6(&mut self) -> PAIRC6_W {
        PAIRC6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc5(&mut self) -> PAIRC5_W {
        PAIRC5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc4(&mut self) -> PAIRC4_W {
        PAIRC4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc3(&mut self) -> PAIRC3_W {
        PAIRC3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc2(&mut self) -> PAIRC2_W {
        PAIRC2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc1(&mut self) -> PAIRC1_W {
        PAIRC1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc0(&mut self) -> PAIRC0_W {
        PAIRC0_W { w: self }
    }
}
