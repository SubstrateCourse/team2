import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card, Statistic } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

import {blake2AsHex} from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [receiver, setReceiver] = useState('');
  const [remark, setRemark] = useState('');

  const handleFileChosen = (file) =>{

    const bufferToDigest = () =>{

      const content = Array.from(new Uint8Array(fileReader.result))
      .map((b) => b.toString(16).padStart(2,'0'))
      .join('');

      const hash = blake2AsHex(content, 256);
      setDigest(hash);


    }

    let fileReader = new FileReader();
    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);

  }


  


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

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <Form>
        <Form.Field>
          <Input
           type='file'
           id='file'
           label='your file'
           onChange = {e => handleFileChosen(e.target.files[0])}>
          </Input>
        </Form.Field>

        <Form.Field>
          <Input
           type='text'
           id='remark'
           label='your remark'
           onChange = {(e) => setRemark(e.target.value)}>
          </Input>
        </Form.Field>

        <Form.Field>
          <Input
           type='text'
           id='receiver'
           label='claim receiver'
           
           onChange = {(e) => setReceiver(e.target.value)}/>

        </Form.Field>

      <Form.Field>
        <TxButton
          accountPair={accountPair}
          label="Create Claim"
          setStatus={setStatus}
          type="SIGNED-TX"
          attrs={{
            palletRpc: 'poeModule',
            callable: 'createClaim',
            inputParams: [digest, remark],
            paramFields: [true, false]
          }}
        />

      <TxButton
          accountPair={accountPair}
          label="Revoke Claim"
          setStatus={setStatus}
          type="SIGNED-TX"
          attrs={{
            palletRpc: 'poeModule',
            callable: 'revokeClaim',
            inputParams: [digest],
            paramFields: [true]
          }}
        />

      <TxButton
          accountPair={accountPair}
          label="Transfer Claim"
          setStatus={setStatus}
          type="SIGNED-TX"
          attrs={{
            palletRpc: 'poeModule',
            callable: 'transferClaim',
            inputParams: [digest, receiver],
            paramFields: [true, true]
          }}
        />

        <div>{status}</div>
        <div>{`claim Info owner: ${owner}, block number: ${blockNumber} `}</div>

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
