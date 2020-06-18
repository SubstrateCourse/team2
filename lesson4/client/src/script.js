import { ApiPromise, WsProvider } from '@polkadot/api';
import { blake2AsHex } from '@polkadot/util-crypto';
import { promises as fs } from 'fs';

const testKeyring = require('@polkadot/keyring/testing');

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
// config
const WEB_SOCKET = 'ws://127.0.0.1:9944';

async function connect() {
  // Construct
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: { Address: 'AccountId' },
  });
  const keyring = testKeyring.default();

  return { api, keyring };
}

async function submitDocInfo(filePath, comment) {
  console.debug(`submitDocInfo: ${filePath}, ${comment}`);
  try {
    const { api, keyring } = await connect();
    const data = await fs.readFile(filePath);
    const content = Array.from(data)
      .map(b => b.toString(16).padStart(2, '0'))
      .join('');
    const claim = blake2AsHex(content, 256);
    const pair = keyring.getPair(ALICE);

    const hash = await api.tx.poeModule
      .createClaim(claim, comment)
      .signAndSend(pair);

    console.log(hash.toHex());
  } catch (err) {
    console.error(`Connect to Substrate error:`, err);
    process.exit(1);
  }

  process.exit(0);
}

async function getUserDocs(acct) {
  console.debug(`getUserDocs: ${acct}`);
  try {
    const { api } = await connect();

    const hashVec = await api.query.poeModule.accountToProofHashList(acct);
    const allClaimsPs = hashVec
      .toJSON()
      .map(v => api.query.poeModule.proofs(v));

    const allClaims = [];
    for await (const data of allClaimsPs) {
      allClaims.push(data.toJSON());
    }

    const answer = {};
    hashVec.toJSON().forEach((v, index) => (answer[v] = allClaims[index]));

    console.log(answer);
  } catch (err) {
    console.error(`Connect to Substrate error:`, err);
  }

  process.exit(0);
}

function main() {
  const args = process.argv.slice(2, 5);
  switch (args[0]) {
    case 'submitDocInfo':
      submitDocInfo(args[1], args[2]);
      break;
    case 'getUserDocs':
      getUserDocs(args[1]);
      break;
    default:
      console.error(
        'Unknown subcommand. Please use `submitDocInfo` or `getUserDocs` only.'
      );
  }
}

main();
