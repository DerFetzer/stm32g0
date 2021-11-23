///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///LPTIM input 1 selection The IN1SEL bits control the LPTIM Input 1 multiplexer, which connects LPTIM Input 1 to one of the available inputs. For connection details refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IN1SEL_A {
    ///0: lptim_in1_mux0
    B_0X0 = 0,
    ///1: lptim_in1_mux1
    B_0X1 = 1,
    ///2: lptim_in1_mux2
    B_0X2 = 2,
    ///3: lptim_in1_mux3
    B_0X3 = 3,
}
impl From<IN1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IN1SEL_A) -> Self {
        variant as _
    }
}
///Field `IN1SEL` reader - LPTIM input 1 selection The IN1SEL bits control the LPTIM Input 1 multiplexer, which connects LPTIM Input 1 to one of the available inputs. For connection details refer to .
pub struct IN1SEL_R(crate::FieldReader<u8, IN1SEL_A>);
impl IN1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        IN1SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IN1SEL_A {
        match self.bits {
            0 => IN1SEL_A::B_0X0,
            1 => IN1SEL_A::B_0X1,
            2 => IN1SEL_A::B_0X2,
            3 => IN1SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == IN1SEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == IN1SEL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == IN1SEL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == IN1SEL_A::B_0X3
    }
}
impl core::ops::Deref for IN1SEL_R {
    type Target = crate::FieldReader<u8, IN1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IN1SEL` writer - LPTIM input 1 selection The IN1SEL bits control the LPTIM Input 1 multiplexer, which connects LPTIM Input 1 to one of the available inputs. For connection details refer to .
pub struct IN1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IN1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///lptim_in1_mux0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IN1SEL_A::B_0X0)
    }
    ///lptim_in1_mux1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IN1SEL_A::B_0X1)
    }
    ///lptim_in1_mux2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(IN1SEL_A::B_0X2)
    }
    ///lptim_in1_mux3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(IN1SEL_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///LPTIM input 2 selection The IN2SEL bits control the LPTIM Input 2 multiplexer, which connect LPTIM Input 2 to one of the available inputs. For connection details refer to . Note: If the LPTIM does not support encoder mode feature, these bits are reserved. Please refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IN2SEL_A {
    ///0: lptim_in2_mux0
    B_0X0 = 0,
    ///1: lptim_in2_mux1
    B_0X1 = 1,
    ///2: lptim_in2_mux2
    B_0X2 = 2,
    ///3: lptim_in2_mux3
    B_0X3 = 3,
}
impl From<IN2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IN2SEL_A) -> Self {
        variant as _
    }
}
///Field `IN2SEL` reader - LPTIM input 2 selection The IN2SEL bits control the LPTIM Input 2 multiplexer, which connect LPTIM Input 2 to one of the available inputs. For connection details refer to . Note: If the LPTIM does not support encoder mode feature, these bits are reserved. Please refer to .
pub struct IN2SEL_R(crate::FieldReader<u8, IN2SEL_A>);
impl IN2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        IN2SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IN2SEL_A {
        match self.bits {
            0 => IN2SEL_A::B_0X0,
            1 => IN2SEL_A::B_0X1,
            2 => IN2SEL_A::B_0X2,
            3 => IN2SEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == IN2SEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == IN2SEL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == IN2SEL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == IN2SEL_A::B_0X3
    }
}
impl core::ops::Deref for IN2SEL_R {
    type Target = crate::FieldReader<u8, IN2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IN2SEL` writer - LPTIM input 2 selection The IN2SEL bits control the LPTIM Input 2 multiplexer, which connect LPTIM Input 2 to one of the available inputs. For connection details refer to . Note: If the LPTIM does not support encoder mode feature, these bits are reserved. Please refer to .
pub struct IN2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IN2SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///lptim_in2_mux0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IN2SEL_A::B_0X0)
    }
    ///lptim_in2_mux1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IN2SEL_A::B_0X1)
    }
    ///lptim_in2_mux2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(IN2SEL_A::B_0X2)
    }
    ///lptim_in2_mux3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(IN2SEL_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    ///Bits 0:1 - LPTIM input 1 selection The IN1SEL bits control the LPTIM Input 1 multiplexer, which connects LPTIM Input 1 to one of the available inputs. For connection details refer to .
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 4:5 - LPTIM input 2 selection The IN2SEL bits control the LPTIM Input 2 multiplexer, which connect LPTIM Input 2 to one of the available inputs. For connection details refer to . Note: If the LPTIM does not support encoder mode feature, these bits are reserved. Please refer to .
    #[inline(always)]
    pub fn in2sel(&self) -> IN2SEL_R {
        IN2SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - LPTIM input 1 selection The IN1SEL bits control the LPTIM Input 1 multiplexer, which connects LPTIM Input 1 to one of the available inputs. For connection details refer to .
    #[inline(always)]
    pub fn in1sel(&mut self) -> IN1SEL_W {
        IN1SEL_W { w: self }
    }
    ///Bits 4:5 - LPTIM input 2 selection The IN2SEL bits control the LPTIM Input 2 multiplexer, which connect LPTIM Input 2 to one of the available inputs. For connection details refer to . Note: If the LPTIM does not support encoder mode feature, these bits are reserved. Please refer to .
    #[inline(always)]
    pub fn in2sel(&mut self) -> IN2SEL_W {
        IN2SEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPTIM configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
