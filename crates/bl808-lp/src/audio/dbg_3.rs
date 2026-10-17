#[doc = "Register `dbg_3` reader"]
pub type R = crate::R<Dbg3Spec>;
#[doc = "Register `dbg_3` writer"]
pub type W = crate::W<Dbg3Spec>;
#[doc = "Field `dac_4s_val` reader - DAC 4S Value"]
pub type Dac4sValR = crate::FieldReader<u32>;
#[doc = "Field `dac_4s_val` writer - DAC 4S Value"]
pub type Dac4sValW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `dac_dwa_0_4s_en` reader - "]
pub type DacDwa0_4sEnR = crate::BitReader;
#[doc = "Field `dac_dwa_0_4s_en` writer - "]
pub type DacDwa0_4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_dwa_1_4s_en` reader - "]
pub type DacDwa1_4sEnR = crate::BitReader;
#[doc = "Field `dac_dwa_1_4s_en` writer - "]
pub type DacDwa1_4sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_in_0_test_sel` reader - "]
pub type DacIn0TestSelR = crate::BitReader;
#[doc = "Field `dac_in_0_test_sel` writer - "]
pub type DacIn0TestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_in_1_test_sel` reader - "]
pub type DacIn1TestSelR = crate::BitReader;
#[doc = "Field `dac_in_1_test_sel` writer - "]
pub type DacIn1TestSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - DAC 4S Value"]
    #[inline(always)]
    pub fn dac_4s_val(&self) -> Dac4sValR {
        Dac4sValR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dac_dwa_0_4s_en(&self) -> DacDwa0_4sEnR {
        DacDwa0_4sEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dac_dwa_1_4s_en(&self) -> DacDwa1_4sEnR {
        DacDwa1_4sEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dac_in_0_test_sel(&self) -> DacIn0TestSelR {
        DacIn0TestSelR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dac_in_1_test_sel(&self) -> DacIn1TestSelR {
        DacIn1TestSelR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - DAC 4S Value"]
    #[inline(always)]
    pub fn dac_4s_val(&mut self) -> Dac4sValW<'_, Dbg3Spec> {
        Dac4sValW::new(self, 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dac_dwa_0_4s_en(&mut self) -> DacDwa0_4sEnW<'_, Dbg3Spec> {
        DacDwa0_4sEnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dac_dwa_1_4s_en(&mut self) -> DacDwa1_4sEnW<'_, Dbg3Spec> {
        DacDwa1_4sEnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dac_in_0_test_sel(&mut self) -> DacIn0TestSelW<'_, Dbg3Spec> {
        DacIn0TestSelW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dac_in_1_test_sel(&mut self) -> DacIn1TestSelW<'_, Dbg3Spec> {
        DacIn1TestSelW::new(self, 23)
    }
}
#[doc = "Debug Register 3: DAC DWA and input test selection\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg3Spec;
impl crate::RegisterSpec for Dbg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_3::R`](R) reader structure"]
impl crate::Readable for Dbg3Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_3::W`](W) writer structure"]
impl crate::Writable for Dbg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dbg_3 to value 0"]
impl crate::Resettable for Dbg3Spec {}
