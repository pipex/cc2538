#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 15)) | (((value as u32) & 0x0001_ffff) << 15);
        self.w
    }
}
#[doc = "Reader of field `TBPWML`"]
pub type TBPWML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPWML`"]
pub struct TBPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWML_W<'a> {
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
#[doc = "Reader of field `TBOTE`"]
pub type TBOTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBOTE`"]
pub struct TBOTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBOTE_W<'a> {
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
#[doc = "Reader of field `Reserved13`"]
pub type RESERVED13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved13`"]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
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
#[doc = "Reader of field `TBEVENT`"]
pub type TBEVENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBEVENT`"]
pub struct TBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `TBSTALL`"]
pub type TBSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBSTALL`"]
pub struct TBSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSTALL_W<'a> {
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
#[doc = "Reader of field `TBEN`"]
pub type TBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBEN`"]
pub struct TBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEN_W<'a> {
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
#[doc = "Reader of field `Reserved8`"]
pub type RESERVED8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
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
#[doc = "Reader of field `TAPWML`"]
pub type TAPWML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAPWML`"]
pub struct TAPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPWML_W<'a> {
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
#[doc = "Reader of field `TAOTE`"]
pub type TAOTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAOTE`"]
pub struct TAOTE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAOTE_W<'a> {
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
#[doc = "Reader of field `Reserved4`"]
pub type RESERVED4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Reserved4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
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
#[doc = "Reader of field `TAEVENT`"]
pub type TAEVENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAEVENT`"]
pub struct TAEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TASTALL`"]
pub type TASTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TASTALL`"]
pub struct TASTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASTALL_W<'a> {
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
#[doc = "Reader of field `TAEN`"]
pub type TAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAEN`"]
pub struct TAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEN_W<'a> {
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
    #[doc = "Bits 15:31 - 31:15\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 14 - 14:14\\] GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&self) -> TBPWML_R {
        TBPWML_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 13:13\\] GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    pub fn tbote(&self) -> TBOTE_R {
        TBOTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\] GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn tbevent(&self) -> TBEVENT_R {
        TBEVENT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tbstall(&self) -> TBSTALL_R {
        TBSTALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn tben(&self) -> TBEN_R {
        TBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tapwml(&self) -> TAPWML_R {
        TAPWML_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    pub fn taote(&self) -> TAOTE_R {
        TAOTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\] GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn taevent(&self) -> TAEVENT_R {
        TAEVENT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\] GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tastall(&self) -> TASTALL_R {
        TASTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn taen(&self) -> TAEN_R {
        TAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 15:31 - 31:15\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] GPTM Timer B PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tbpwml(&mut self) -> TBPWML_W {
        TBPWML_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] GPTM Timer B output trigger enable 0: The ADC trigger of output Timer B is disabled. 1: The ADC trigger of output Timer B is enabled."]
    #[inline(always)]
    pub fn tbote(&mut self) -> TBOTE_W {
        TBOTE_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\] GPTM Timer B event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn tbevent(&mut self) -> TBEVENT_W {
        TBEVENT_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B stall enable 0: Timer B continues counting while the processor is halted by the debugger. 1: Timer B freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tbstall(&mut self) -> TBSTALL_W {
        TBSTALL_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B enable 0: Timer B is disabled. 1: Timer B is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn tben(&mut self) -> TBEN_W {
        TBEN_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] GPTM Timer A PWM output level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub fn tapwml(&mut self) -> TAPWML_W {
        TAPWML_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] GPTM Timer A output trigger enable 0: The ADC trigger of output Timer A is disabled. 1: The ADC trigger of output Timer A is enabled."]
    #[inline(always)]
    pub fn taote(&mut self) -> TAOTE_W {
        TAOTE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] GPTM Timer A event mode 0x0: Positive edge 0x1: Negative edge 0x2: Reserved 0x3: Both edges"]
    #[inline(always)]
    pub fn taevent(&mut self) -> TAEVENT_W {
        TAEVENT_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] GPTM Timer A stall enable 0: Timer A continues counting while the processor is halted by the debugger. 1: Timer A freezes counting while the processor is halted by the debugger."]
    #[inline(always)]
    pub fn tastall(&mut self) -> TASTALL_W {
        TASTALL_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] GPTM Timer A enable 0: Timer A is disabled. 1: Timer A is enabled and begins counting or the capture logic is enabled based on the GPTMCFG register."]
    #[inline(always)]
    pub fn taen(&mut self) -> TAEN_W {
        TAEN_W { w: self }
    }
}
