use perf_group_lib::measurement_procedures::{register_measurement_group, has_registered, open_measurement,
    start_measurement, stop_measurement, close_measurement_and_release};

use perf_group_lib::bindings_perf_lib::{construct_cache_type, PerfMeasurement, PerfMeasurementGroup, MeasurementResult,
    PIDConfig, CPUConfig, PrivilegeMode};
use perf_group_lib::bindings_perf::{PERF_COUNT_SW_PAGE_FAULTS_MAJ, PERF_COUNT_SW_PAGE_FAULTS_MIN, PERF_TYPE_SOFTWARE, PERF_TYPE_HW_CACHE,
    PERF_COUNT_HW_CACHE_LL, PERF_COUNT_HW_CACHE_OP_READ, PERF_COUNT_HW_CACHE_RESULT_ACCESS, PERF_COUNT_HW_CACHE_RESULT_MISS};


pub fn perf_log_register(){
    if ! has_registered(){
        let mut perf_measurement_group = PerfMeasurementGroup::new();

        let mut llc_cache_read_acc = construct_cache_type(PERF_COUNT_HW_CACHE_LL, PERF_COUNT_HW_CACHE_OP_READ, PERF_COUNT_HW_CACHE_RESULT_ACCESS);
        let mut llc_cache_read_miss = construct_cache_type(PERF_COUNT_HW_CACHE_LL, PERF_COUNT_HW_CACHE_OP_READ, PERF_COUNT_HW_CACHE_RESULT_MISS);
        let mut perf_measurement1 = PerfMeasurement::new(PERF_TYPE_HW_CACHE,
            llc_cache_read_acc, PIDConfig::SelfPid, CPUConfig::All, PrivilegeMode::User);
        perf_measurement_group.add_perf_measurement(perf_measurement1);
        let mut perf_measurement2 = PerfMeasurement::new(PERF_TYPE_HW_CACHE,
            llc_cache_read_miss, PIDConfig::SelfPid, CPUConfig::All, PrivilegeMode::User);
        perf_measurement_group.add_perf_measurement(perf_measurement2);
        
        // let mut perf_measurement1 = PerfMeasurement::new(PERF_TYPE_SOFTWARE,
        //     PERF_COUNT_SW_PAGE_FAULTS_MAJ, PIDConfig::SelfPid, CPUConfig::All, PrivilegeMode::User);
        // perf_measurement_group.add_perf_measurement(perf_measurement1);
        // let mut perf_measurement2 = PerfMeasurement::new(PERF_TYPE_SOFTWARE,
        //     PERF_COUNT_SW_PAGE_FAULTS_MIN, PIDConfig::SelfPid, CPUConfig::All, PrivilegeMode::User);
        // perf_measurement_group.add_perf_measurement(perf_measurement2);

        register_measurement_group(perf_measurement_group);
        if open_measurement() < 0 {
            panic!("open measurement failed");
        }
    }
} 

pub fn perf_log_start(){
    start_measurement();
}

pub fn perf_log_stop()->MeasurementResult{
    stop_measurement()
}

pub fn perf_log_stop_and_log(){
    let result = perf_log_stop();
    info!("major faults: {}, minor faults: {}", result.results[0], result.results[1]);
}

pub fn perf_log_close(){
    close_measurement_and_release();
}