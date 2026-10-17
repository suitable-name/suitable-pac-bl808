#[doc = "Register `config_0` reader"]
pub type R = crate::R<Config0Spec>;
#[doc = "Register `config_0` writer"]
pub type W = crate::W<Config0Spec>;
#[doc = "Field `pta_en` reader - Enable packet traffic arbitration"]
pub type PtaEnR = crate::BitReader;
#[doc = "Field `pta_en` writer - Enable packet traffic arbitration"]
pub type PtaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlan_active` reader - WLAN active; 1=WLAN is the preferred radio"]
pub type WlanActiveR = crate::BitReader;
#[doc = "Field `wlan_active` writer - WLAN active; 1=WLAN is the preferred radio"]
pub type WlanActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `priority_en` reader - Enable PTI-based priority arbitration"]
pub type PriorityEnR = crate::BitReader;
#[doc = "Field `priority_en` writer - Enable PTI-based priority arbitration"]
pub type PriorityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_wlan_tx` reader - Force-grant WLAN TX regardless of BT state"]
pub type ForceWlanTxR = crate::BitReader;
#[doc = "Field `force_wlan_tx` writer - Force-grant WLAN TX regardless of BT state"]
pub type ForceWlanTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_wlan_rx` reader - Force-grant WLAN RX regardless of BT state"]
pub type ForceWlanRxR = crate::BitReader;
#[doc = "Field `force_wlan_rx` writer - Force-grant WLAN RX regardless of BT state"]
pub type ForceWlanRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_bt_tx` reader - Force-grant BT TX regardless of WLAN state"]
pub type ForceBtTxR = crate::BitReader;
#[doc = "Field `force_bt_tx` writer - Force-grant BT TX regardless of WLAN state"]
pub type ForceBtTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `force_bt_rx` reader - Force-grant BT RX regardless of WLAN state"]
pub type ForceBtRxR = crate::BitReader;
#[doc = "Field `force_bt_rx` writer - Force-grant BT RX regardless of WLAN state"]
pub type ForceBtRxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable packet traffic arbitration"]
    #[inline(always)]
    pub fn pta_en(&self) -> PtaEnR {
        PtaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WLAN active; 1=WLAN is the preferred radio"]
    #[inline(always)]
    pub fn wlan_active(&self) -> WlanActiveR {
        WlanActiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable PTI-based priority arbitration"]
    #[inline(always)]
    pub fn priority_en(&self) -> PriorityEnR {
        PriorityEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Force-grant WLAN TX regardless of BT state"]
    #[inline(always)]
    pub fn force_wlan_tx(&self) -> ForceWlanTxR {
        ForceWlanTxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force-grant WLAN RX regardless of BT state"]
    #[inline(always)]
    pub fn force_wlan_rx(&self) -> ForceWlanRxR {
        ForceWlanRxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force-grant BT TX regardless of WLAN state"]
    #[inline(always)]
    pub fn force_bt_tx(&self) -> ForceBtTxR {
        ForceBtTxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force-grant BT RX regardless of WLAN state"]
    #[inline(always)]
    pub fn force_bt_rx(&self) -> ForceBtRxR {
        ForceBtRxR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable packet traffic arbitration"]
    #[inline(always)]
    pub fn pta_en(&mut self) -> PtaEnW<'_, Config0Spec> {
        PtaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - WLAN active; 1=WLAN is the preferred radio"]
    #[inline(always)]
    pub fn wlan_active(&mut self) -> WlanActiveW<'_, Config0Spec> {
        WlanActiveW::new(self, 1)
    }
    #[doc = "Bit 4 - Enable PTI-based priority arbitration"]
    #[inline(always)]
    pub fn priority_en(&mut self) -> PriorityEnW<'_, Config0Spec> {
        PriorityEnW::new(self, 4)
    }
    #[doc = "Bit 16 - Force-grant WLAN TX regardless of BT state"]
    #[inline(always)]
    pub fn force_wlan_tx(&mut self) -> ForceWlanTxW<'_, Config0Spec> {
        ForceWlanTxW::new(self, 16)
    }
    #[doc = "Bit 17 - Force-grant WLAN RX regardless of BT state"]
    #[inline(always)]
    pub fn force_wlan_rx(&mut self) -> ForceWlanRxW<'_, Config0Spec> {
        ForceWlanRxW::new(self, 17)
    }
    #[doc = "Bit 18 - Force-grant BT TX regardless of WLAN state"]
    #[inline(always)]
    pub fn force_bt_tx(&mut self) -> ForceBtTxW<'_, Config0Spec> {
        ForceBtTxW::new(self, 18)
    }
    #[doc = "Bit 19 - Force-grant BT RX regardless of WLAN state"]
    #[inline(always)]
    pub fn force_bt_rx(&mut self) -> ForceBtRxW<'_, Config0Spec> {
        ForceBtRxW::new(self, 19)
    }
}
#[doc = "Arbitration configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`config_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config0Spec;
impl crate::RegisterSpec for Config0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_0::R`](R) reader structure"]
impl crate::Readable for Config0Spec {}
#[doc = "`write(|w| ..)` method takes [`config_0::W`](W) writer structure"]
impl crate::Writable for Config0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets config_0 to value 0"]
impl crate::Resettable for Config0Spec {}
