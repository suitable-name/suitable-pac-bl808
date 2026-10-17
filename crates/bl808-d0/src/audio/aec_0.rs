#[doc = "Register `aec_0` reader"]
pub type R = crate::R<Aec0Spec>;
#[doc = "Register `aec_0` writer"]
pub type W = crate::W<Aec0Spec>;
#[doc = "Field `aec_0_atten_mode` reader - "]
pub type Aec0AttenModeR = crate::FieldReader;
#[doc = "Field `aec_0_atten_mode` writer - "]
pub type Aec0AttenModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aec_1_atten_mode` reader - "]
pub type Aec1AttenModeR = crate::FieldReader;
#[doc = "Field `aec_1_atten_mode` writer - "]
pub type Aec1AttenModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aec_record_vld_4s_div` reader - "]
pub type AecRecordVld4sDivR = crate::FieldReader;
#[doc = "Field `aec_record_vld_4s_div` writer - "]
pub type AecRecordVld4sDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aec_record_vld_4s_en` reader - "]
pub type AecRecordVld4sEnR = crate::BitReader;
#[doc = "Field `aec_record_vld_4s_en` writer - "]
pub type AecRecordVld4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn aec_0_atten_mode(&self) -> Aec0AttenModeR {
        Aec0AttenModeR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn aec_1_atten_mode(&self) -> Aec1AttenModeR {
        Aec1AttenModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn aec_record_vld_4s_div(&self) -> AecRecordVld4sDivR {
        AecRecordVld4sDivR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn aec_record_vld_4s_en(&self) -> AecRecordVld4sEnR {
        AecRecordVld4sEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn aec_0_atten_mode(&mut self) -> Aec0AttenModeW<'_, Aec0Spec> {
        Aec0AttenModeW::new(self, 5)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn aec_1_atten_mode(&mut self) -> Aec1AttenModeW<'_, Aec0Spec> {
        Aec1AttenModeW::new(self, 8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn aec_record_vld_4s_div(&mut self) -> AecRecordVld4sDivW<'_, Aec0Spec> {
        AecRecordVld4sDivW::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn aec_record_vld_4s_en(&mut self) -> AecRecordVld4sEnW<'_, Aec0Spec> {
        AecRecordVld4sEnW::new(self, 13)
    }
}
#[doc = "AEC Configuration 0: Acoustic Echo Cancellation control\n\nYou can [`read`](crate::Reg::read) this register and get [`aec_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aec_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aec0Spec;
impl crate::RegisterSpec for Aec0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aec_0::R`](R) reader structure"]
impl crate::Readable for Aec0Spec {}
#[doc = "`write(|w| ..)` method takes [`aec_0::W`](W) writer structure"]
impl crate::Writable for Aec0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets aec_0 to value 0"]
impl crate::Resettable for Aec0Spec {}
