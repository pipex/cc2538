#[doc = "Reader of register OPTIONS"]
pub type R = crate::R<u32, super::OPTIONS>;
#[doc = "Writer for register OPTIONS"]
pub type W = crate::W<u32, super::OPTIONS>;
#[doc = "Register OPTIONS `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTIONS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIRST_LNME_FIFO_DEPTH`"]
pub type FIRST_LNME_FIFO_DEPTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIRST_LNME_FIFO_DEPTH`"]
pub struct FIRST_LNME_FIFO_DEPTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FIRST_LNME_FIFO_DEPTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `FIRST_LNME_NR_OF_PES`"]
pub type FIRST_LNME_NR_OF_PES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIRST_LNME_NR_OF_PES`"]
pub struct FIRST_LNME_NR_OF_PES_W<'a> {
    w: &'a mut W,
}
impl<'a> FIRST_LNME_NR_OF_PES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `Reserved2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `MMM3A`"]
pub type MMM3A_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MMM3A`"]
pub struct MMM3A_W<'a> {
    w: &'a mut W,
}
impl<'a> MMM3A_W<'a> {
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
#[doc = "Reader of field `INT_MASKING`"]
pub type INT_MASKING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_MASKING`"]
pub struct INT_MASKING_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_MASKING_W<'a> {
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
#[doc = "Reader of field `PROTECTION_OPTION`"]
pub type PROTECTION_OPTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROTECTION_OPTION`"]
pub struct PROTECTION_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTECTION_OPTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `PROGRAM_RAM`"]
pub type PROGRAM_RAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROGRAM_RAM`"]
pub struct PROGRAM_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGRAM_RAM_W<'a> {
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
#[doc = "Reader of field `SEQUENCER_CONFIGURATION`"]
pub type SEQUENCER_CONFIGURATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEQUENCER_CONFIGURATION`"]
pub struct SEQUENCER_CONFIGURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQUENCER_CONFIGURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `LNME_CONFIGURATION`"]
pub type LNME_CONFIGURATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNME_CONFIGURATION`"]
pub struct LNME_CONFIGURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> LNME_CONFIGURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `PKCP_CONFIGURATION`"]
pub type PKCP_CONFIGURATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PKCP_CONFIGURATION`"]
pub struct PKCP_CONFIGURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> PKCP_CONFIGURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\] Number of words in the first LNME's FIFO RAM Should be ignored if LNME configuration is 0. The contents of this field indicate the actual depth as selected by the LNME FIFO RAM size strap input, fifo_size_sel. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn first_lnme_fifo_depth(&self) -> FIRST_LNME_FIFO_DEPTH_R {
        FIRST_LNME_FIFO_DEPTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\] Ignore on read"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\] Number of processing elements in the pipeline of the first LNME Should be ignored if LNME configuration is 0. Note: Reset value is undefined."]
    #[inline(always)]
    pub fn first_lnme_nr_of_pes(&self) -> FIRST_LNME_NR_OF_PES_R {
        FIRST_LNME_NR_OF_PES_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\] Ignore on read"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - 12:12\\] Reserved for a future functional extension to the LNME Always 0b"]
    #[inline(always)]
    pub fn mmm3a(&self) -> MMM3A_R {
        MMM3A_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\] Value 0b indicates that the main interrupt output (bit \\[1\\] of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, value 1b indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn int_masking(&self) -> INT_MASKING_R {
        INT_MASKING_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\] Value 0 indicates no additional protection against side channel attacks, value 1 indicates the SCAP option, value 3 indicates the PROT option; other values are reserved. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn protection_option(&self) -> PROTECTION_OPTION_R {
        PROTECTION_OPTION_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - 7:7\\] Value 1b indicates sequencer program storage in RAM, value 0b in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn program_ram(&self) -> PROGRAM_RAM_R {
        PROGRAM_RAM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\] Value 1 indicates a standard sequencer; other values are reserved."]
    #[inline(always)]
    pub fn sequencer_configuration(&self) -> SEQUENCER_CONFIGURATION_R {
        SEQUENCER_CONFIGURATION_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - 4:2\\] Value 0 indicates NO LNME, value 1 indicates one standard LNME (with alpha = 32, beta = 8); other values reserved. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn lnme_configuration(&self) -> LNME_CONFIGURATION_R {
        LNME_CONFIGURATION_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] Value 1 indicates a PKCP with a 16x16 multiplier, value 2 indicates a PKCP with a 32x32 multiplier, other values reserved. Note: Reset value is undefined."]
    #[inline(always)]
    pub fn pkcp_configuration(&self) -> PKCP_CONFIGURATION_R {
        PKCP_CONFIGURATION_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\] Number of words in the first LNME's FIFO RAM Should be ignored if LNME configuration is 0. The contents of this field indicate the actual depth as selected by the LNME FIFO RAM size strap input, fifo_size_sel. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn first_lnme_fifo_depth(&mut self) -> FIRST_LNME_FIFO_DEPTH_W {
        FIRST_LNME_FIFO_DEPTH_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Ignore on read"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\] Number of processing elements in the pipeline of the first LNME Should be ignored if LNME configuration is 0. Note: Reset value is undefined."]
    #[inline(always)]
    pub fn first_lnme_nr_of_pes(&mut self) -> FIRST_LNME_NR_OF_PES_W {
        FIRST_LNME_NR_OF_PES_W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\] Ignore on read"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Reserved for a future functional extension to the LNME Always 0b"]
    #[inline(always)]
    pub fn mmm3a(&mut self) -> MMM3A_W {
        MMM3A_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Value 0b indicates that the main interrupt output (bit \\[1\\] of the interrupts output bus) is the direct complement of the run bit in the PKA_CONTROL register, value 1b indicates that interrupt masking logic is present for this output. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn int_masking(&mut self) -> INT_MASKING_W {
        INT_MASKING_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Value 0 indicates no additional protection against side channel attacks, value 1 indicates the SCAP option, value 3 indicates the PROT option; other values are reserved. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn protection_option(&mut self) -> PROTECTION_OPTION_W {
        PROTECTION_OPTION_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Value 1b indicates sequencer program storage in RAM, value 0b in ROM. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn program_ram(&mut self) -> PROGRAM_RAM_W {
        PROGRAM_RAM_W { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\] Value 1 indicates a standard sequencer; other values are reserved."]
    #[inline(always)]
    pub fn sequencer_configuration(&mut self) -> SEQUENCER_CONFIGURATION_W {
        SEQUENCER_CONFIGURATION_W { w: self }
    }
    #[doc = "Bits 2:4 - 4:2\\] Value 0 indicates NO LNME, value 1 indicates one standard LNME (with alpha = 32, beta = 8); other values reserved. Note: Reset value is undefined"]
    #[inline(always)]
    pub fn lnme_configuration(&mut self) -> LNME_CONFIGURATION_W {
        LNME_CONFIGURATION_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Value 1 indicates a PKCP with a 16x16 multiplier, value 2 indicates a PKCP with a 32x32 multiplier, other values reserved. Note: Reset value is undefined."]
    #[inline(always)]
    pub fn pkcp_configuration(&mut self) -> PKCP_CONFIGURATION_W {
        PKCP_CONFIGURATION_W { w: self }
    }
}
