#[doc = "Register `DBI_CONFIG` reader"]
pub type R = crate::R<DbiConfigSpec>;
#[doc = "Register `DBI_CONFIG` writer"]
pub type W = crate::W<DbiConfigSpec>;
#[doc = "Field `cr_dbi_en` reader - DBI module enable"]
pub type CrDbiEnR = crate::BitReader;
#[doc = "Field `cr_dbi_en` writer - DBI module enable"]
pub type CrDbiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_sel` reader - DBI interface selection (0: Type B, 1: Type C)"]
pub type CrDbiSelR = crate::BitReader;
#[doc = "Field `cr_dbi_sel` writer - DBI interface selection (0: Type B, 1: Type C)"]
pub type CrDbiSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_cmd_en` reader - Command phase enable"]
pub type CrDbiCmdEnR = crate::BitReader;
#[doc = "Field `cr_dbi_cmd_en` writer - Command phase enable"]
pub type CrDbiCmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_dat_en` reader - Data phase enable"]
pub type CrDbiDatEnR = crate::BitReader;
#[doc = "Field `cr_dbi_dat_en` writer - Data phase enable"]
pub type CrDbiDatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_dat_wr` reader - Data write operation mode"]
pub type CrDbiDatWrR = crate::BitReader;
#[doc = "Field `cr_dbi_dat_wr` writer - Data write operation mode"]
pub type CrDbiDatWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_dat_tp` reader - Data phase type (0: 8-bit, 1: 16-bit packed)"]
pub type CrDbiDatTpR = crate::BitReader;
#[doc = "Field `cr_dbi_dat_tp` writer - Data phase type (0: 8-bit, 1: 16-bit packed)"]
pub type CrDbiDatTpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_dat_bc` reader - Data phase byte count (0:1 byte, 1:2 bytes, 2:3 bytes, 3:4 bytes)"]
pub type CrDbiDatBcR = crate::FieldReader;
#[doc = "Field `cr_dbi_dat_bc` writer - Data phase byte count (0:1 byte, 1:2 bytes, 2:3 bytes, 3:4 bytes)"]
pub type CrDbiDatBcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_dbi_cmd` reader - 8-bit Command value for transfer"]
pub type CrDbiCmdR = crate::FieldReader;
#[doc = "Field `cr_dbi_cmd` writer - 8-bit Command value for transfer"]
pub type CrDbiCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_dbi_scl_pol` reader - SCL Clock polarity (0: Active High, 1: Active Low)"]
pub type CrDbiSclPolR = crate::BitReader;
#[doc = "Field `cr_dbi_scl_pol` writer - SCL Clock polarity (0: Active High, 1: Active Low)"]
pub type CrDbiSclPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_scl_ph` reader - SCL Clock phase"]
pub type CrDbiSclPhR = crate::BitReader;
#[doc = "Field `cr_dbi_scl_ph` writer - SCL Clock phase"]
pub type CrDbiSclPhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_cont_en` reader - Continuous transfer enable (for large data payload)"]
pub type CrDbiContEnR = crate::BitReader;
#[doc = "Field `cr_dbi_cont_en` writer - Continuous transfer enable (for large data payload)"]
pub type CrDbiContEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_dmy_en` reader - Dummy cycle enable"]
pub type CrDbiDmyEnR = crate::BitReader;
#[doc = "Field `cr_dbi_dmy_en` writer - Dummy cycle enable"]
pub type CrDbiDmyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_dmy_cnt` reader - Dummy cycle count"]
pub type CrDbiDmyCntR = crate::FieldReader;
#[doc = "Field `cr_dbi_dmy_cnt` writer - Dummy cycle count"]
pub type CrDbiDmyCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_dbi_tc_3w_mode` reader - 3-wire TC mode select (if applicable)"]
pub type CrDbiTc3wModeR = crate::BitReader;
#[doc = "Field `cr_dbi_tc_3w_mode` writer - 3-wire TC mode select (if applicable)"]
pub type CrDbiTc3wModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_tc_deg_en` reader - Timing control deglitch enable"]
pub type CrDbiTcDegEnR = crate::BitReader;
#[doc = "Field `cr_dbi_tc_deg_en` writer - Timing control deglitch enable"]
pub type CrDbiTcDegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_dbi_tc_deg_cnt` reader - Timing control deglitch cycle count"]
pub type CrDbiTcDegCntR = crate::FieldReader;
#[doc = "Field `cr_dbi_tc_deg_cnt` writer - Timing control deglitch cycle count"]
pub type CrDbiTcDegCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - DBI module enable"]
    #[inline(always)]
    pub fn cr_dbi_en(&self) -> CrDbiEnR {
        CrDbiEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBI interface selection (0: Type B, 1: Type C)"]
    #[inline(always)]
    pub fn cr_dbi_sel(&self) -> CrDbiSelR {
        CrDbiSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command phase enable"]
    #[inline(always)]
    pub fn cr_dbi_cmd_en(&self) -> CrDbiCmdEnR {
        CrDbiCmdEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data phase enable"]
    #[inline(always)]
    pub fn cr_dbi_dat_en(&self) -> CrDbiDatEnR {
        CrDbiDatEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data write operation mode"]
    #[inline(always)]
    pub fn cr_dbi_dat_wr(&self) -> CrDbiDatWrR {
        CrDbiDatWrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data phase type (0: 8-bit, 1: 16-bit packed)"]
    #[inline(always)]
    pub fn cr_dbi_dat_tp(&self) -> CrDbiDatTpR {
        CrDbiDatTpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data phase byte count (0:1 byte, 1:2 bytes, 2:3 bytes, 3:4 bytes)"]
    #[inline(always)]
    pub fn cr_dbi_dat_bc(&self) -> CrDbiDatBcR {
        CrDbiDatBcR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit Command value for transfer"]
    #[inline(always)]
    pub fn cr_dbi_cmd(&self) -> CrDbiCmdR {
        CrDbiCmdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - SCL Clock polarity (0: Active High, 1: Active Low)"]
    #[inline(always)]
    pub fn cr_dbi_scl_pol(&self) -> CrDbiSclPolR {
        CrDbiSclPolR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCL Clock phase"]
    #[inline(always)]
    pub fn cr_dbi_scl_ph(&self) -> CrDbiSclPhR {
        CrDbiSclPhR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Continuous transfer enable (for large data payload)"]
    #[inline(always)]
    pub fn cr_dbi_cont_en(&self) -> CrDbiContEnR {
        CrDbiContEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Dummy cycle enable"]
    #[inline(always)]
    pub fn cr_dbi_dmy_en(&self) -> CrDbiDmyEnR {
        CrDbiDmyEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Dummy cycle count"]
    #[inline(always)]
    pub fn cr_dbi_dmy_cnt(&self) -> CrDbiDmyCntR {
        CrDbiDmyCntR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - 3-wire TC mode select (if applicable)"]
    #[inline(always)]
    pub fn cr_dbi_tc_3w_mode(&self) -> CrDbiTc3wModeR {
        CrDbiTc3wModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timing control deglitch enable"]
    #[inline(always)]
    pub fn cr_dbi_tc_deg_en(&self) -> CrDbiTcDegEnR {
        CrDbiTcDegEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Timing control deglitch cycle count"]
    #[inline(always)]
    pub fn cr_dbi_tc_deg_cnt(&self) -> CrDbiTcDegCntR {
        CrDbiTcDegCntR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DBI module enable"]
    #[inline(always)]
    pub fn cr_dbi_en(&mut self) -> CrDbiEnW<'_, DbiConfigSpec> {
        CrDbiEnW::new(self, 0)
    }
    #[doc = "Bit 1 - DBI interface selection (0: Type B, 1: Type C)"]
    #[inline(always)]
    pub fn cr_dbi_sel(&mut self) -> CrDbiSelW<'_, DbiConfigSpec> {
        CrDbiSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Command phase enable"]
    #[inline(always)]
    pub fn cr_dbi_cmd_en(&mut self) -> CrDbiCmdEnW<'_, DbiConfigSpec> {
        CrDbiCmdEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Data phase enable"]
    #[inline(always)]
    pub fn cr_dbi_dat_en(&mut self) -> CrDbiDatEnW<'_, DbiConfigSpec> {
        CrDbiDatEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Data write operation mode"]
    #[inline(always)]
    pub fn cr_dbi_dat_wr(&mut self) -> CrDbiDatWrW<'_, DbiConfigSpec> {
        CrDbiDatWrW::new(self, 4)
    }
    #[doc = "Bit 5 - Data phase type (0: 8-bit, 1: 16-bit packed)"]
    #[inline(always)]
    pub fn cr_dbi_dat_tp(&mut self) -> CrDbiDatTpW<'_, DbiConfigSpec> {
        CrDbiDatTpW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Data phase byte count (0:1 byte, 1:2 bytes, 2:3 bytes, 3:4 bytes)"]
    #[inline(always)]
    pub fn cr_dbi_dat_bc(&mut self) -> CrDbiDatBcW<'_, DbiConfigSpec> {
        CrDbiDatBcW::new(self, 6)
    }
    #[doc = "Bits 8:15 - 8-bit Command value for transfer"]
    #[inline(always)]
    pub fn cr_dbi_cmd(&mut self) -> CrDbiCmdW<'_, DbiConfigSpec> {
        CrDbiCmdW::new(self, 8)
    }
    #[doc = "Bit 16 - SCL Clock polarity (0: Active High, 1: Active Low)"]
    #[inline(always)]
    pub fn cr_dbi_scl_pol(&mut self) -> CrDbiSclPolW<'_, DbiConfigSpec> {
        CrDbiSclPolW::new(self, 16)
    }
    #[doc = "Bit 17 - SCL Clock phase"]
    #[inline(always)]
    pub fn cr_dbi_scl_ph(&mut self) -> CrDbiSclPhW<'_, DbiConfigSpec> {
        CrDbiSclPhW::new(self, 17)
    }
    #[doc = "Bit 18 - Continuous transfer enable (for large data payload)"]
    #[inline(always)]
    pub fn cr_dbi_cont_en(&mut self) -> CrDbiContEnW<'_, DbiConfigSpec> {
        CrDbiContEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Dummy cycle enable"]
    #[inline(always)]
    pub fn cr_dbi_dmy_en(&mut self) -> CrDbiDmyEnW<'_, DbiConfigSpec> {
        CrDbiDmyEnW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Dummy cycle count"]
    #[inline(always)]
    pub fn cr_dbi_dmy_cnt(&mut self) -> CrDbiDmyCntW<'_, DbiConfigSpec> {
        CrDbiDmyCntW::new(self, 20)
    }
    #[doc = "Bit 27 - 3-wire TC mode select (if applicable)"]
    #[inline(always)]
    pub fn cr_dbi_tc_3w_mode(&mut self) -> CrDbiTc3wModeW<'_, DbiConfigSpec> {
        CrDbiTc3wModeW::new(self, 27)
    }
    #[doc = "Bit 28 - Timing control deglitch enable"]
    #[inline(always)]
    pub fn cr_dbi_tc_deg_en(&mut self) -> CrDbiTcDegEnW<'_, DbiConfigSpec> {
        CrDbiTcDegEnW::new(self, 28)
    }
    #[doc = "Bits 29:31 - Timing control deglitch cycle count"]
    #[inline(always)]
    pub fn cr_dbi_tc_deg_cnt(&mut self) -> CrDbiTcDegCntW<'_, DbiConfigSpec> {
        CrDbiTcDegCntW::new(self, 29)
    }
}
#[doc = "DBI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiConfigSpec;
impl crate::RegisterSpec for DbiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_config::R`](R) reader structure"]
impl crate::Readable for DbiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dbi_config::W`](W) writer structure"]
impl crate::Writable for DbiConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_CONFIG to value 0x0005_2c1c"]
impl crate::Resettable for DbiConfigSpec {
    const RESET_VALUE: u32 = 0x0005_2c1c;
}
