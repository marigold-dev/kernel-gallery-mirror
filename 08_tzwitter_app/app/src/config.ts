// Environment variable are injected during build time

// The rpc of the tezos node, by default, the rpc of a local tezos node is used
// export const TEZOS_RPC = process.env.TEZOS_RPC || "http://localhost:18731";
export const TEZOS_RPC = 'http://localhost:18731';

// The rpc of the rollup node, by default it's the local smart rollup node port
export const ROLLUP_RPC =
  import.meta.env.VITE_ROLLUP_RPC || 'http://localhost:8932';

// Commitment period
export const COMMITMENT_INTERVAL: number =
  Number(import.meta.env.COMMITMENT_PERIOD) || 60; // Commitment interval

//
export const CEMENTED_PERIOD: number =
  Number(import.meta.env.CEMENTED_PERIOD) || 80640; // Time for a commit to be cemented

export const BLOCK_TIME: number = Number(import.meta.env.BLOCK_TIME) || 15; // Time of a block is 15s
