#[doc = "Register `switch_hypervisor` reader"]
pub type R = crate::R<SwitchHypervisorSpec>;
#[doc = "Register `switch_hypervisor` writer"]
pub type W = crate::W<SwitchHypervisorSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SwitchHypervisorSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Switch the the hypervisor.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`switch_hypervisor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`switch_hypervisor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwitchHypervisorSpec;
impl crate::RegisterSpec for SwitchHypervisorSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`switch_hypervisor::R`](R) reader structure"]
impl crate::Readable for SwitchHypervisorSpec {}
#[doc = "`write(|w| ..)` method takes [`switch_hypervisor::W`](W) writer structure"]
impl crate::Writable for SwitchHypervisorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets switch_hypervisor to value 0"]
impl crate::Resettable for SwitchHypervisorSpec {
    const RESET_VALUE: u64 = 0;
}
