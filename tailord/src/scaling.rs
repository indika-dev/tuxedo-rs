use tuxedo_sysfs::sysfs_util::read_to_string;

pub const ACPI_CPUFREQ: &str = "acpi-cpufreq"; // CPUFreq driver which utilizes the ACPI Processor Performance States. This driver also supports the Intel Enhanced SpeedStep (previously supported by the deprecated speedstep_centrino module). For AMD Ryzen it only provides 3 frequency states.
pub const AMD_PSTATE: &str = "amd-pstate"; // This driver has three modes corresponding to different degrees of autonomy from the CPU hardware: active, passive, and guided. The amd_pstate CPU power scaling driver is used automatically in "active mode" on supported CPUs (Zen 2 and newer) since kernel version 6.5. See #amd_pstate for details.
pub const AMD_PSTATE_EPP: &str = "amd-pstate-epp"; // This driver implements a scaling driver selected by amd_pstate=active with an internal governor for AMD Ryzen (some Zen 2 and newer) processors.
pub const CPPC_CPUFREQ: &str = "cppc-cpufreq"; // CPUFreq driver based on ACPI CPPC system (see #Collaborative processor performance control). Common default on AArch64 systems. Works on modern x86 too, but the intel_pstate and amd_pstate drivers are better.
pub const INTEL_CPUFREQ: &str = "intel-cpufreq"; // Starting with kernel 5.7, the intel_pstate scaling driver selects "passive mode" aka intel_cpufreq for CPUs that do not support hardware-managed P-states (HWP), i.e. Intel Core i 5th generation or older. This "passive" driver acts similar to the ACPI driver on Intel CPUs, except that it does not have the 16-pstate limit of ACPI.
pub const INTEL_PSTATE: &str = "intel-pstate"; // This driver implements a scaling driver with an internal governor for Intel Core (Sandy Bridge and newer) processors. It is used automatically for these processors instead of the other drivers below. This driver takes priority over other drivers and is built-in as opposed to being a module. intel_pstate may run in "passive mode" via the intel_cpufreq driver for older CPUs. If you encounter a problem while using this driver, add intel_pstate=disable to your kernel line in order to revert to using the acpi_cpufreq driver.
pub const P4_CLOCKMOD: &str = "p4-clockmod"; // CPUFreq driver for Intel Pentium 4/Xeon/Celeron processors which lowers the CPU temperature by skipping clocks. (You probably want to use speedstep_lib instead.)
pub const PCC_CPUFREQ: &str = "pcc-cpufreq"; // This driver supports Processor Clocking Control interface by Hewlett-Packard and Microsoft Corporation which is useful on some ProLiant servers.
pub const POWERNOW_K8: &str = "powernow-k8"; // CPUFreq driver for K8/K10 Athlon 64/Opteron/Phenom processors. Since Linux 3.7, 'acpi_cpufreq' will automatically be used for more modern AMD CPUs.
pub const SPEEDSTEP_LIB: &str = "speedstep-lib"; // CPUFreq driver for Intel SpeedStep-enabled processors (mostly Atoms and older Pentiums)

pub const SYSFS_PATH_SCALING_DRIVER: &str = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_driver";

#[allow(unused)]
pub struct ScalingDriver;

impl ScalingDriver {
    pub fn get_available_scaling_profiles(&self) -> Vec<String> {
        return vec![
            ACPI_CPUFREQ.to_string(),
            AMD_PSTATE.to_string(),
            AMD_PSTATE_EPP.to_string(),
            CPPC_CPUFREQ.to_string(),
            INTEL_CPUFREQ.to_string(),
            INTEL_PSTATE.to_string(),
            P4_CLOCKMOD.to_string(),
            PCC_CPUFREQ.to_string(),
            POWERNOW_K8.to_string(),
            SPEEDSTEP_LIB.to_string(),
        ];
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct ScalingDriverHandle {
    current_driver: String,
}

impl ScalingDriverHandle {
    pub fn set_active_scaling_driver(&mut self, name: &str) {}
    pub fn get_active_scaling_driver(&self) -> &str {
        self.current_driver = sysfs_util.read_string(&SYSFS_PATH_SCALING_DRIVER).unwrap();
        return current_driver;
    }
    pub fn is_pstate_active(&self) -> bool {
        self.current_driver = self
            .sysfs_util
            .read_string(&SYSFS_PATH_SCALING_DRIVER)
            .unwrap();
        current_driver == INTEL_PSTATE.to_string()
            || current_driver == AMD_PSTATE.to_string()
            || current_driver == AMD_PSTATE_EPP.to_string()
    }
}

impl std::fmt::Display for ScalingDriverHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
