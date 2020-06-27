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
  const [digest, setDigest] = useState('');
  const [memo, setMemo] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [addressTo, setFormState] = useState({ addressTo: null });
  const [accountId, setAccountId] = useState('');
  const [proofs, setProofs] = useState([]);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
      setMemo(result[3].toString());
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

  const MAX_MEMO_LENGTH = 256;
  const handleMemoChange = (memo) => {
    if (memo.value && memo.value.length > MAX_MEMO_LENGTH) {
      memo.value = memo.value.substring(0, MAX_MEMO_LENGTH);
    }
    setMemo(memo.value);
  };

  const convertToString = (hex) => {
    if (hex && hex.length) {
      return decodeURIComponent(hex.replace(/\s+/g, '').replace(/[0-9a-f]{2}/g, '%$&')).substr(2);
    }
    return '';
  };

  const queryProofs = () => {
    // unsub && unsub();
    api.query.poeModule.accounts(accountId, (result) => {
      setProofs([]);
      if (result && result.length) {
        const claimProofs = [];
        result.forEach((digest) => api.query.poeModule.proofs(digest.toString(), (res) => {
          var date = new Date(parseInt(res[2]));
          var datetime = date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
          claimProofs.push({
            claim: digest.toString(),
            blockNumber: res[1].toNumber(),
            createTime: datetime,
            memo: convertToString(res[3].toString())
          });
          setProofs(claimProofs);
        }));
      }
    }).then()
      .catch(console.error);
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='text'
            placeholder='address'
            label="User Address"
            value={accountId}
            onChange={ (e) => setAccountId(e.target.value) }
          />
        </Form.Field>

        <Form.Field style={{ textAlign: 'center' }}>
          <Button onClick={queryProofs}>Query user owned proofs</Button>
        </Form.Field>
      </Form>

      <table class="ui very padded table">
        <thead className='theads'>
          <tr>
            <th >creationTime</th>
            <th >blockNumber</th>
            <th >claim</th>
            <th >memo</th>
          </tr>
        </thead>
        <tbody>
          {
            proofs.map((proof, index) => {
              return (
                <tr key={ index }>
                  <td>{proof.createTime}</td>
                  <td>{proof.blockNumber}</td>
                  <td>{proof.claim}</td>
                  <td>{proof.memo}</td>
                </tr>
              );
            })
          }
        </tbody>
      </table>

      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label="Your File"
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
        </Form.Field>

        <Form.Field>
          Comment:
          <Input
            type='input'
            id='memo'
            lable='Memo'
            value={memo}
            placeholder='Some note (max 256 chars)'
            max = {256}
            onChange={ handleMemoChange }
          />
        </Form.Field>

        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, memo],
              paramFields: [true]
            }}
          />
          <TxButton
            accountPair={accountPair}
            label='Revoke Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
        </Form.Field>

        <Form.Field>
          <Input
            type='file'
            id='file'
            label="Your File"
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
        </Form.Field>

        <Form.Field>
          <Input
            fluid
            label='To'
            type='text'
            placeholder='address'
            state='addressTo'
            onChange={ (_, data) =>
              setFormState(prev => ({ ...prev, [data.state]: data.value })) }
          />
        </Form.Field>

        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, addressTo],
              paramFields: [true, true]
            }}
          />
        </Form.Field>

        <div>{status}</div>
        <div>{`Claim info, owner: ${owner}, blockNumber: ${blockNumber}`}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
