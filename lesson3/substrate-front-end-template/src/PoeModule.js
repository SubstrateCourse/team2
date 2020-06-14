/** @format */

import React, { useEffect, useState } from "react";
import { Form, Input, Grid } from "semantic-ui-react";

import { useSubstrate } from "./substrate-lib";
import { TxButton } from "./substrate-lib/components";
import { blake2AsHex } from "@polkadot/util-crypto";

function Main(props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState("");
  const [digest, setDigest] = useState("");
  const [owner, setOwner] = useState("");
  const [blockNumber, setBlockNumber] = useState(0);

  const [toAccountId, setToAccountId] = useState("");

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule
      .proofs(digest, (result) => {
        setDigest(digest);
        setOwner(result[0].toString());
        setBlockNumber(result[1].toNumber());
      })
      .then((unsub) => {
        unsubscribe = unsub;
      })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [api.query.poeModule, digest]);

  const handleFileChosen = (file) => {
    const reader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(reader.result))
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    };

    reader.onloadend = bufferToDigest;
    reader.readAsArrayBuffer(file);
  };

  return (
    <Grid.Column width={8}>
      <h1>Poe Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label='your file'
            onChange={(e) => handleFileChosen(e.target.files[0])}
          ></Input>
        </Form.Field>
        <Form.Field>
          <Input
            label='transfer to AccountId'
            state='newValue'
            type='text'
            onChange={(_, { value }) => setToAccountId(value)}
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: "poeModule",
              callable: "createClaim",
              inputParams: [digest],
              paramFields: [true],
            }}
          />

          <TxButton
            accountPair={accountPair}
            label='Revoke Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: "poeModule",
              callable: "revokeClaim",
              inputParams: [digest],
              paramFields: [true],
            }}
          />

          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: "poeModule",
              callable: "transferClaim",
              inputParams: [digest, toAccountId],
              paramFields: [true],
            }}
          />
        </Form.Field>
        <div>{status}</div>
        <div>{`Claim info, owner: ${owner}, blockNumber: ${blockNumber}`}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule(props) {
  const { api } = useSubstrate();
  return api.query.poeModule && api.query.poeModule.proofs ? (
    <Main {...props} />
  ) : null;
}
