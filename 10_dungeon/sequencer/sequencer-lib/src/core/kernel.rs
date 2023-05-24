use tezos_smart_rollup_host::runtime::Runtime;

pub trait Kernel {
    fn entry<Host: Runtime>(host: &mut Host);
}
