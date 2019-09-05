#[doc = "Reader of register LPBKGPT"]
pub type R = crate::R<u32, super::LPBKGPT>;
#[doc = "Writer for register LPBKGPT"]
pub type W = crate::W<u32, super::LPBKGPT>;
#[doc = "Register LPBKGPT `reset()`'s with value 0"]
impl crate::ResetValue for super::LPBKGPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved1`"]
pub type RESERVED1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Reserved1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Reader of field `LPBK32GPT3`"]
pub type LPBK32GPT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPBK32GPT3`"]
pub struct LPBK32GPT3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK32GPT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `LPBK32GPT2`"]
pub type LPBK32GPT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPBK32GPT2`"]
pub struct LPBK32GPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK32GPT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `LPBK32GPT1`"]
pub type LPBK32GPT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPBK32GPT1`"]
pub struct LPBK32GPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK32GPT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `LPBK16GPT3`"]
pub type LPBK16GPT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPBK16GPT3`"]
pub struct LPBK16GPT3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK16GPT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `LPBK16GPT2`"]
pub type LPBK16GPT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPBK16GPT2`"]
pub struct LPBK16GPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK16GPT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LPBK16GPT1`"]
pub type LPBK16GPT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPBK16GPT1`"]
pub struct LPBK16GPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK16GPT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `LPBK16GPT0`"]
pub type LPBK16GPT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPBK16GPT0`"]
pub struct LPBK16GPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK16GPT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31 - 31:22\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:21 - 21:20\\] GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt3(&self) -> LPBK32GPT3_R {
        LPBK32GPT3_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\] GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt2(&self) -> LPBK32GPT2_R {
        LPBK32GPT2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\] GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt1(&self) -> LPBK32GPT1_R {
        LPBK32GPT1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\] GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt3(&self) -> LPBK16GPT3_R {
        LPBK16GPT3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\] GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt2(&self) -> LPBK16GPT2_R {
        LPBK16GPT2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\] GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt1(&self) -> LPBK16GPT1_R {
        LPBK16GPT1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\] GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt0(&self) -> LPBK16GPT0_R {
        LPBK16GPT0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 22:31 - 31:22\\] Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 20:21 - 21:20\\] GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt3(&mut self) -> LPBK32GPT3_W {
        LPBK32GPT3_W { w: self }
    }
    #[doc = "Bits 18:19 - 19:18\\] GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt2(&mut self) -> LPBK32GPT2_W {
        LPBK32GPT2_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\] GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt1(&mut self) -> LPBK32GPT1_W {
        LPBK32GPT1_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt3(&mut self) -> LPBK16GPT3_W {
        LPBK16GPT3_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt2(&mut self) -> LPBK16GPT2_W {
        LPBK16GPT2_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt1(&mut self) -> LPBK16GPT1_W {
        LPBK16GPT1_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt0(&mut self) -> LPBK16GPT0_W {
        LPBK16GPT0_W { w: self }
    }
}
