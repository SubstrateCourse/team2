import React, { useEffect, useState } from 'react';
import { Form, Input, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('none');
  const [receiver, setReceiver] = useState('none');
  const [blockNumber, setBlockNumber] = useState(0);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      if (result.isNone) {
        setOwner('none');
        setBlockNumber(0);
      } else {
        result = result.unwrap();
        setOwner(result[0].toString());
        setBlockNumber(result[1].toNumber());
      }
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  function handleFileChosen (file) {
    let fileReader = new FileReader();

    function bufferToDigest () {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');
      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    }

    fileReader.onload = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  }

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <br/>
      <Form>
        <Form.Field>
          <Input type='file' id='file' label='Your File'
                 onChange={(e) => handleFileChosen(e.target.files[0])}
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
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
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true],
            }}
          />
        </Form.Field>
      </Form>
      <br/>
      <Form>
        <Form.Field>
          <Input type='file' id='file' label='Your File'
                 onChange={(e) => handleFileChosen(e.target.files[0])}
          />
          <Input
            label='Claim Receiver'
            state='receiver'
            type='string'
            onChange={(_, { value }) => setReceiver(value)}
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, receiver],
              paramFields: [true],
            }}
          />
        </Form.Field>
      </Form>
      <br/>
      <div>{status}</div>
      <div>{`Claim info, owner: ${owner}, blockNumber: ${blockNumber}`}</div>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
