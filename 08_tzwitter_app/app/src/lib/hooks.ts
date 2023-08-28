import { useEffect } from "react";
import { TEZOS_RPC } from "../config";

/**
 * Call the callback on each new block.
 * @param callback the callback to call
 * @param dependencies the list of dependencies as useEffect
 */
export const useBlock = (callback: any, dependencies: any[]) => {
    let rpc = TEZOS_RPC + "/monitor/heads/main";
    console.log(rpc);

    useEffect(() => {
        // start fetching
        const controller = new AbortController();
        const signal = controller.signal;
        fetch(rpc, { signal })
            .then(res => {
                if (res.body) {
                    return res.body
                }
                throw "Body is null";
            })
            .then(async body => {
                const reader = body.getReader();
                while (true) {
                    const { done } = await reader.read();
                    if (done) break;
                    callback();
                }
            })
        return () => {
            controller.abort();
        }
    }, dependencies)
}