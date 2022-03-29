import Web3 from 'web3';
 
import util from 'util';
import { createServer } from "http";
import { Server } from "socket.io";
//import { emit } from 'process';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { cryptoWaitReady, mnemonicGenerate, mnemonicValidate, mnemonicToMiniSecret, sr25519PairFromSeed } from '@polkadot/util-crypto';
import { Keyring } from '@polkadot/keyring';

/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
///
///       CONNECTED USERS + PLAYER DATA
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

const clients = [];



/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
///
///       WALLET & SOCKET INFO
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

const mainAddr = "5H9MGAViHSoz2W6W44kD6pwByx4TuYpPnzuvsGw5QPkkN7tq";
let pubKey = "";
const provider = new WsProvider('ws://127.0.0.1:9944');

const httpServer = createServer();
const io = new Server(httpServer);

// Create the API and wait until ready

async function main () {
  // Initialise the provider to connect to the local node
 
  const api = await ApiPromise.create({ provider });

  // Retrieve the chain & node information information via rpc calls
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);

  const now = await api.query.timestamp.now();
  const nowReadable = now.toNumber();
  console.log(now.toNumber());
  console.log(pubKey);

  // Retrieve the account balance & nonce via the system module
  const balance  = await api.query.system.account(mainAddr);
  let readableBalance = balance.data.free.toNumber();
  console.log(readableBalance);

  io.emit('current num', "BlockNumber:  " +nowReadable);


}

async function getBalance() {
	const api = await ApiPromise.create({ provider });

	const balance  = await api.query.system.account(mainAddr);
	let readableBalance = balance.data.free.toNumber();
	console.log(readableBalance);
	io.emit('get balance',  (readableBalance / (10 **9) ) + "  VRMETA.");
	console.log(readableBalance);
	
}

async function connectAndStartMatch() {
	const api = await ApiPromise.create({ provider });
   // Constuct the keyring after the API (crypto has an async init)
   const keyring = new Keyring({ type: 'sr25519' });

   // Add Alice to our keyring with a hard-deived path (empty phrase, so uses dev)
   const alicex = keyring.addFromUri('doll pizza govern cart thunder gentle pulse century coin suggest carbon shock');

  await api.tx.timestake.connect().signAndSend(alicex, (result) => {
    if (result.status.isInBlock) {
      console.log('in a block');
    } else if (result.status.isFinalized) {
      console.log('finalized');
      
    }
  });

	io.emit('match started',  "result");

	
}

async function createWallet() {

    await cryptoWaitReady();
    const mnemonic = mnemonicGenerate(12);
    const seed = mnemonic;

      // Validate the mnemic string that was generated
    const isValidMnemonic = mnemonicValidate(mnemonic);


// Create valid Substrate-compatible seed from mnemonic
    const seedUser = mnemonicToMiniSecret(mnemonic);
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });

// Generate new public/secret keypair for Alice from the supplied seed
     const { publicKey, secretKey } = sr25519PairFromSeed(seedUser);
    const publicKey1 = keyring.encodeAddress(publicKey);
    const secretKey1 = secretKey;

const pair = keyring.createFromUri(mnemonic);
console.log(mnemonic);
console.log(publicKey);



pubKey = publicKey1;

io.emit('wallet created', "New wallet address: "+ pubKey);


}


/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
///
///       CONNECTION + COMMUNICATION
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
io.on('connection', function(socket){

 clients.push(socket.id);

 var clientConnectedMsg = 'User connected ' + util.inspect(socket.id) + ', total: ' + clients.length;
 console.log(clientConnectedMsg);

 socket.on('change', (num) => {
	setNum(num);
});


socket.on('get balance', () => {
	getBalance();
});

socket.on('create wallet', () => {
	createWallet();
});

socket.on('start match', () => {
  connectAndStartMatch();
})

socket.on('disconnect', function(){
  clients.pop(socket.id);

  var clientDisconnectedMsg = 'User disconnected ' + util.inspect(socket.id) + ', total: ' + clients.length;
  console.log(clientDisconnectedMsg);
  
 })
});

/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
///
///       SERVER PORT
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

httpServer.listen(9945, function(){
  console.log('listening on *:9945');
});

/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
///
///       REPEATED FUNCTIONS IN MILLISECONDS
///
/// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

setInterval(main, 2000);
setInterval(connectAndStartMatch, 3000);

//setInterval(getData, 6000);