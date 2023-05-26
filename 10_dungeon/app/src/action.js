export const move = (data, signer) => async () => {
  const address = await signer.publicKeyHash();

  // The data send to rollup is 01->0... We add the public key to the
  // data and connect it with the `-`, later we can retrieve the {data}
  // from this combination.
  // {publicKeyHash}-{data}
  const formated = `${address}-${data}`;
  // we need to convert to string to hex so that the sequencer
  const bytes = Buffer.from(formated).toString("hex");

  const action = { data: bytes };
  const headers = new Headers();
  headers.append("Content-Type", "application/json");

  const res = await fetch("http://localhost:8080/operations", {
    body: JSON.stringify(action),
    method: "POST",
    headers,
  });
};
