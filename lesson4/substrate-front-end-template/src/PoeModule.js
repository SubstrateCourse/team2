import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Button } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');

  // The currently stored value
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState('');
  const [note, setNote] = useState('');
  const [userAddress, setUserAddress] = useState('');

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const handleFileChosen = (file) => {
    const fileReader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    };

    fileReader.onloadend = bufferToDigest;

    fileReader.readAsArrayBuffer(file);
  };

  const handleFileNote = (value) => {
    setNote(Buffer.from(value));
  };

  const handleUserAddress = (value) => {
    setUserAddress(value);
  }

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label='New File'
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
          <Input
            type='text'
            id='note'
            label='Note'
            onChange={ (_, { value }) => handleFileNote(value) }
          />
        </Form.Field>

        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, note],
              paramFields: [true]
            }}
          />
        </Form.Field>
        <div>{status}</div>
        {
          status.includes('Finalized') && <div>{`You have successfully claimed file with hash ${digest} with note ${note.toString()}.`}</div>
        }
        <Form.Field>
          <Input
            type='text'
            id='address'
            label='User Address'
            onChange={ (_, { value }) => handleUserAddress(value) }
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Query User Doc'
            type='QUERY'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'accountToProofs',
              inputParams: [userAddress],
              paramFields: [true]
            }}
          />
          {/* <Button>Query User Doc</Button> */}
        </Form.Field>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
