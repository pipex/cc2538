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
#[doc = "Reader of field `Reserved11`"]
pub type RESERVED11_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved11`"]
pub struct RESERVED11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | (((value as u32) & 0x07ff) << 21);
        self.w
    }
}
#[doc = "Reader of field `DMACHANS`"]
pub type DMACHANS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMACHANS`"]
pub struct DMACHANS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACHANS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `Reserved8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `Reserved3`"]
pub type RESERVED3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Reserved3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `MASTEN`"]
pub type MASTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTEN`"]
pub struct MASTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTEN_W<'a> {
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
    #[doc = "Bits 21:31 - 31:21\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\] Available uDMA channels minus 1 This field contains a value equal to the number of uDMA channels the uDMA controller is configured to use, minus one. The value of 0x1F corresponds to 32 uDMA channels."]
    #[inline(always)]
    pub fn dmachans(&self) -> DMACHANS_R {
        DMACHANS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\] Control state machine status This field shows the current status of the control state-machine. Status can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source end pointer 0x3: Reading destination end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA-0xF: Undefined"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 1:3 - 3:1\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - 0:0\\] Master enable status 0: The uDMA controller is disabled. 1: The uDMA controller is enabled."]
    #[inline(always)]
    pub fn masten(&self) -> MASTEN_R {
        MASTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 21:31 - 31:21\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved11(&mut self) -> RESERVED11_W {
        RESERVED11_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\] Available uDMA channels minus 1 This field contains a value equal to the number of uDMA channels the uDMA controller is configured to use, minus one. The value of 0x1F corresponds to 32 uDMA channels."]
    #[inline(always)]
    pub fn dmachans(&mut self) -> DMACHANS_W {
        DMACHANS_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Control state machine status This field shows the current status of the control state-machine. Status can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source end pointer 0x3: Reading destination end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA-0xF: Undefined"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bits 1:3 - 3:1\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Master enable status 0: The uDMA controller is disabled. 1: The uDMA controller is enabled."]
    #[inline(always)]
    pub fn masten(&mut self) -> MASTEN_W {
        MASTEN_W { w: self }
    }
}
