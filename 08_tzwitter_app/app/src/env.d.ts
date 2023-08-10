/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_TEZOS_RPC: string;
  readonly VITE_ROLLUP_RPC: string;
  readonly VITE_COMMITMENT_PERIOD: number;
  readonly VITE_CEMENTED_PERIOD: number;
  readonly VITE_BLOCK_TIME: number;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
