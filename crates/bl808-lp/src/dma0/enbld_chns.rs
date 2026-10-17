#[doc = "Register `EnbldChns` reader"]
pub type R = crate::R<EnbldChnsSpec>;
#[doc = "Field `EnabledChannels` reader - Enable status for channels 0-7"]
pub type EnabledChannelsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Enable status for channels 0-7"]
    #[inline(always)]
    pub fn enabled_channels(&self) -> EnabledChannelsR {
        EnabledChannelsR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Channel enable status\n\nYou can [`read`](crate::Reg::read) this register and get [`enbld_chns::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnbldChnsSpec;
impl crate::RegisterSpec for EnbldChnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enbld_chns::R`](R) reader structure"]
impl crate::Readable for EnbldChnsSpec {}
#[doc = "`reset()` method sets EnbldChns to value 0"]
impl crate::Resettable for EnbldChnsSpec {}
