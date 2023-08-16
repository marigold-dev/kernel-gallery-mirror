import { TezosToolkit } from '@taquito/taquito';
import { SmartRollupAddMessagesOperation } from '@taquito/taquito/dist/types/operations/smart-rollup-add-messages-operation';

interface RollupBlock {
  block_hash: string;
  level: number;
}

interface Signer {
  sign: (bytes: string) => Promise<{
    bytes: string;
    sig: string;
    prefixSig: string;
    sbytes: string;
  }>;
  publicKey: () => Promise<string>;
  publicKeyHash: () => Promise<string>;
}

class RollupClient {
  private tezos: TezosToolkit;
  private rollupUrl: string;

  constructor({
    tezos,
    rollupUrl,
  }: {
    tezos: TezosToolkit;
    rollupUrl: string;
  }) {
    console.log('Initializing RollupClient with:', { tezos, rollupUrl });
    this.tezos = tezos;
    this.rollupUrl = rollupUrl;
  }

  async send(payload: string): Promise<SmartRollupAddMessagesOperation> {
    console.log('send called with payload:', payload);
    const op = await this.tezos.contract.smartRollupAddMessages({
      message: [payload],
    });
    return op;
  }

  async getState(path: string) {
    console.log('getState called with path:', path);
    const rollupUrl = this.rollupUrl;
    const url = `${rollupUrl}/global/block/head/durable/wasm_2_0_0/value?key=${path}`.replace("{}/", "global/");
    console.log('Constructed URL for getState:', url);
    const res = await fetch(url);
    if (!res.ok) {
      console.error('Error fetching from URL:', url, 'Status:', res.status);
    } else {
      console.log('Successful fetch from URL:', url);
    }
    const result = await res.json();
    console.log('Fetched data for getState:', result);
    return result;
  }

  async getSubkeys(path: string) {
    console.log('getSubkeys called with path:', path);
    const rollupUrl = this.rollupUrl;
    const url = `${rollupUrl}/global/block/head/durable/wasm_2_0_0/subkeys?key=${path}`.replace("{}/", "global/");
    console.log('Constructed URL for getSubkeys:', url);
    const res = await fetch(url);
    if (!res.ok) {
      console.error('Error fetching from URL:', url, 'Status:', res.status);
    } else {
      console.log('Successful fetch from URL:', url);
    }
    const result = await res.json();
    console.log('Fetched data for getSubkeys:', result);
    return result;
  }

  async tezosLevel(): Promise<number> {
    console.log('tezosLevel called');
    const rollupUrl = this.rollupUrl;
    const url = `${rollupUrl}/global/tezos_level`.replace("{}/", "global/");
    console.log('Constructed URL for tezosLevel:', url);
    const res = await fetch(url);
    if (!res.ok) {
      console.error('Error fetching from URL:', url, 'Status:', res.status);
    } else {
      console.log('Successful fetch from URL:', url);
    }
    const result = await res.text();
    console.log('Fetched data for tezosLevel:', result);
    return Number(result);
  }

  async getBlock(blockHash: string): Promise<RollupBlock> {
    console.log('getBlock called with blockHash:', blockHash);
    const rollupUrl = this.rollupUrl;
    const url = `${rollupUrl}/global/block/${blockHash}`.replace("{}/", "global/");
    console.log('Constructed URL for getBlock:', url);
    const res = await fetch(url);
    if (!res.ok) {
      console.error('Error fetching from URL:', url, 'Status:', res.status);
    } else {
      console.log('Successful fetch from URL:', url);
    }
    const result = await res.json();
    console.log('Fetched data for getBlock:', result);
    return result;
  }
}

export { RollupClient, type Signer };
